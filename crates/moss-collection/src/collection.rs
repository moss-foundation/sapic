use anyhow::Result;
use chrono::{DateTime, Utc};
use joinerror::ResultExt;
use json_patch::{
    AddOperation, PatchOperation, RemoveOperation, ReplaceOperation, jsonptr::PointerBuf,
};
use moss_applib::{
    AppRuntime, EventMarker,
    subscription::{Event, EventEmitter},
};
use moss_bindingutils::primitives::{ChangePath, ChangeString};
use moss_edit::json::EditOptions;
use moss_environment::{environment::Environment, models::primitives::EnvironmentId};
use moss_fs::{FileSystem, FsResultExt};
use moss_git::{models::types::BranchInfo, url::normalize_git_url};
use moss_git_hosting_provider::{
    GitHostingProvider,
    common::GitUrlForAPI,
    github::client::GitHubClient,
    gitlab::client::GitLabClient,
    models::{primitives::GitProviderType, types::Contributor},
};
use serde_json::Value as JsonValue;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::OnceCell;

use crate::{
    DescribeCollection, DescribeRepository,
    edit::CollectionEdit,
    helpers::{fetch_contributors, fetch_vcs_summary},
    manifest::{MANIFEST_FILE_NAME, ManifestFile},
    services::{
        git_service::GitService, set_icon_service::SetIconService, storage_service::StorageService,
    },
    worktree::Worktree,
};

pub struct EnvironmentItem<R: AppRuntime> {
    pub id: EnvironmentId,
    pub name: String,
    pub inner: Environment<R>,
}

type EnvironmentMap<R> = HashMap<EnvironmentId, Arc<EnvironmentItem<R>>>;

#[derive(Debug, Clone)]
pub enum OnDidChangeEvent {
    Toggled(bool),
}

impl EventMarker for OnDidChangeEvent {}

pub struct CollectionModifyParams {
    pub name: Option<String>,
    pub repository: Option<ChangeString>,
    pub icon_path: Option<ChangePath>,
}

pub enum VcsSummary {
    GitHub {
        branch: BranchInfo,
        url: String,
        updated_at: Option<String>,
        owner: Option<String>,
    },
    GitLab {
        branch: BranchInfo,
        url: String,
        updated_at: Option<String>,
        owner: Option<String>,
    },
}

pub struct CollectionDetails {
    pub name: String,
    pub vcs: Option<VcsSummary>,
    pub contributors: Vec<Contributor>,
    pub created_at: String, // File created time
}

pub struct Collection<R: AppRuntime> {
    #[allow(dead_code)]
    pub(super) fs: Arc<dyn FileSystem>,
    pub(super) abs_path: Arc<Path>,
    pub(super) edit: CollectionEdit,
    pub(super) worktree: Arc<Worktree<R>>,
    pub(super) set_icon_service: SetIconService,
    pub(super) storage_service: Arc<StorageService<R>>,
    pub(super) git_service: Arc<GitService>,
    // TODO: Extract Git Provider Service
    pub(super) github_client: Arc<GitHubClient>,
    pub(super) gitlab_client: Arc<GitLabClient>,
    #[allow(dead_code)]
    pub(super) environments: OnceCell<EnvironmentMap<R>>,

    pub(super) on_did_change: EventEmitter<OnDidChangeEvent>,
}

#[rustfmt::skip]
impl<R: AppRuntime> Collection<R> {
    pub fn on_did_change(&self) -> Event<OnDidChangeEvent> { self.on_did_change.event() }
}

impl<R: AppRuntime> Collection<R> {
    pub fn abs_path(&self) -> &Arc<Path> {
        &self.abs_path
    }

    pub fn external_path(&self) -> Option<&Arc<Path>> {
        unimplemented!()
    }

    pub fn icon_path(&self) -> Option<PathBuf> {
        self.set_icon_service.icon_path()
    }
    pub async fn describe(&self) -> joinerror::Result<DescribeCollection> {
        let manifest_path = self.abs_path.join(MANIFEST_FILE_NAME);

        let rdr = self
            .fs
            .open_file(&manifest_path)
            .await
            .join_err_with::<()>(|| {
                format!("failed to open manifest file: {}", manifest_path.display())
            })?;

        let manifest: ManifestFile = serde_json::from_reader(rdr).join_err_with::<()>(|| {
            format!("failed to parse manifest file: {}", manifest_path.display())
        })?;

        Ok(DescribeCollection {
            name: manifest.name,
            repository: manifest.repository.map(|repo| DescribeRepository {
                repository: repo.url,
                git_provider_type: repo.git_provider_type,
            }),
        })
    }

    pub async fn describe_details(&self) -> joinerror::Result<CollectionDetails> {
        let desc = self.describe().await?;
        let created_at_time =
            std::fs::metadata(self.abs_path.join(MANIFEST_FILE_NAME))?.created()?;
        let created_at: DateTime<Utc> = created_at_time.into();

        let mut output = CollectionDetails {
            name: desc.name,
            vcs: None,
            contributors: vec![],
            created_at: created_at.to_rfc3339(),
        };

        if let Some(repo_desc) = desc.repository {
            let repo_ref = match GitUrlForAPI::parse(&repo_desc.repository) {
                Ok(repo_ref) => repo_ref,
                Err(e) => {
                    // TODO: Tell the frontend
                    println!(
                        "unable to parse repository {}: {}",
                        repo_desc.repository,
                        e.to_string()
                    );
                    return Ok(output);
                }
            };
            let git_provider_type = repo_desc.git_provider_type;
            let client: Arc<dyn GitHostingProvider> = match &git_provider_type {
                GitProviderType::GitHub => self.github_client.clone(),
                GitProviderType::GitLab => self.gitlab_client.clone(),
            };

            output.contributors = fetch_contributors(&repo_ref, client.clone())
                .await
                .unwrap_or_else(|e| {
                    // TODO: Tell the frontend
                    println!("unable to fetch contributors: {}", e);
                    Vec::new()
                });

            output.vcs =
                match fetch_vcs_summary(self, &repo_ref, git_provider_type, client.clone()).await {
                    Ok(vcs) => Some(vcs),
                    Err(e) => {
                        // TODO: Tell the fronend
                        println!("unable to fetch vcs: {}", e);
                        None
                    }
                };
        }

        Ok(output)
    }

    pub async fn modify(&self, params: CollectionModifyParams) -> joinerror::Result<()> {
        let mut patches = Vec::new();

        if let Some(new_name) = params.name {
            patches.push((
                PatchOperation::Replace(ReplaceOperation {
                    path: unsafe { PointerBuf::new_unchecked("/name") },
                    value: JsonValue::String(new_name),
                }),
                EditOptions {
                    create_missing_segments: false,
                    ignore_if_not_exists: false,
                },
            ));
        }

        match params.repository {
            Some(ChangeString::Update(url)) => {
                let normalized_url = normalize_git_url(&url)?;
                patches.push((
                    PatchOperation::Add(AddOperation {
                        path: unsafe { PointerBuf::new_unchecked("/repository/url") },
                        value: JsonValue::String(normalized_url),
                    }),
                    EditOptions {
                        create_missing_segments: false,
                        ignore_if_not_exists: false,
                    },
                ));
            }
            Some(ChangeString::Remove) => {
                patches.push((
                    PatchOperation::Remove(RemoveOperation {
                        path: unsafe { PointerBuf::new_unchecked("/repository/url") },
                    }),
                    EditOptions {
                        create_missing_segments: false,
                        ignore_if_not_exists: true,
                    },
                ));
            }
            None => {}
        }

        match params.icon_path {
            None => {}
            Some(ChangePath::Update(new_icon_path)) => {
                self.set_icon_service.set_icon(&new_icon_path)?;
            }
            Some(ChangePath::Remove) => {
                self.set_icon_service.remove_icon().await?;
            }
        }
        self.edit
            .edit(&patches)
            .await
            .join_err::<()>("failed to edit collection")?;

        Ok(())
    }
    pub async fn environments(&self) -> Result<&EnvironmentMap<R>> {
        let result = self
            .environments
            .get_or_try_init(|| async move {
                let environments = HashMap::new();
                Ok::<_, anyhow::Error>(environments)
            })
            .await?;

        Ok(result)
    }

    pub async fn get_current_branch_info(&self) -> joinerror::Result<BranchInfo> {
        self.git_service.get_current_branch_info().await
    }
    pub async fn dispose(&self) -> joinerror::Result<()> {
        self.git_service.dispose(self.fs.clone()).await
    }
}

#[cfg(any(test, feature = "integration-tests"))]
impl<R: AppRuntime> Collection<R> {
    pub fn db(&self) -> &Arc<dyn moss_storage::CollectionStorage<R::AsyncContext>> {
        self.storage_service.storage()
    }
}
