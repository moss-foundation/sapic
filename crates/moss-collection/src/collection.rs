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
use moss_fs::{FileSystem, FsResultExt};
use moss_git::{models::types::BranchInfo, url::normalize_git_url};
use moss_git_hosting_provider::{
    GitHostingProvider,
    common::GitUrl,
    models::{primitives::GitProviderType, types::Contributor},
};
use serde_json::Value as JsonValue;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::{
    dirs,
    edit::CollectionEdit,
    git::GitClient,
    manifest::{MANIFEST_FILE_NAME, ManifestFile},
    services::{
        git_service::GitService, set_icon_service::SetIconService, storage_service::StorageService,
    },
    worktree::Worktree,
};

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

impl VcsSummary {
    pub fn url(&self) -> Option<String> {
        match self {
            VcsSummary::GitHub { url, .. } => Some(url.clone()),
            VcsSummary::GitLab { url, .. } => Some(url.clone()),
        }
    }

    pub fn branch(&self) -> Option<BranchInfo> {
        match self {
            VcsSummary::GitHub { branch, .. } => Some(branch.clone()),
            VcsSummary::GitLab { branch, .. } => Some(branch.clone()),
        }
    }
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
    pub(super) git_service: Option<Arc<GitService>>,
    // TODO: Extract Git Provider Service
    // pub(super) github_client: Arc<GitHubClient>,
    // pub(super) gitlab_client: Arc<GitLabClient>,

    // FIXME: Should be optional
    // pub(super) git_client: Option<GitClient>,
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

    pub fn environments_path(&self) -> PathBuf {
        self.abs_path.join(dirs::ENVIRONMENTS_DIR)
    }

    pub async fn details(&self) -> joinerror::Result<CollectionDetails> {
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

        let created_at: DateTime<Utc> = std::fs::metadata(&manifest_path)?.created()?.into();

        let mut output = CollectionDetails {
            name: manifest.name,
            vcs: None,
            contributors: vec![], // FIXME: hardcoded for now
            created_at: created_at.to_rfc3339(),
        };

        let Some(repo_desc) = manifest.repository else {
            return Ok(output);
        };

        let repo_url = match GitUrl::parse(&repo_desc.url) {
            Ok(repo_ref) => repo_ref,
            Err(e) => {
                println!("unable to parse repository url{}: {}", repo_desc.url, e);
                return Ok(output);
            }
        };

        // match &self.git_client {
        //     GitClient::GitHub { api, session } => {
        //         let branch = self.git_service.get_current_branch_info().await?;
        //         let resp = match api.get_repository(session, &repo_url).await {
        //             Ok(resp) => Some(resp),
        //             Err(e) => {
        //                 println!("unable to fetch repository: {}", e);
        //                 None
        //             }
        //         };

        //         output.vcs = Some(VcsSummary::GitHub {
        //             branch,
        //             url: repo_url.to_string(),
        //             updated_at: resp.as_ref().map(|r| r.updated_at.to_owned()),
        //             owner: resp.as_ref().map(|r| r.owner.login.to_owned()),
        //         });
        //     }
        //     GitClient::GitLab { api, session } => {
        //         let branch = self.git_service.get_current_branch_info().await?;
        //         let resp = match api.get_repository(session, &repo_url).await {
        //             Ok(resp) => Some(resp),
        //             Err(e) => {
        //                 println!("unable to fetch repository: {}", e);
        //                 None
        //             }
        //         };

        //         output.vcs = Some(VcsSummary::GitLab {
        //             branch,
        //             url: repo_url.to_string(),
        //             updated_at: resp.as_ref().map(|r| r.updated_at.to_owned()),
        //             owner: resp.as_ref().map(|r| r.owner.username.to_owned()),
        //         });
        //     }
        // };

        // FIXME: should be implemented in a cleaner way
        // output.contributors = fetch_contributors(&repo_ref, client.clone())
        //     .await
        //     .unwrap_or_else(|e| {
        //         println!("unable to fetch contributors: {}", e);
        //         Vec::new()
        //     });

        Ok(output)
    }

    // async fn fetch_vcs_summary(
    //     &self,
    //     url: &GitUrl,
    //     git_provider_type: GitProviderType,
    //     client: Arc<dyn GitHostingProvider>,
    // ) -> joinerror::Result<VcsSummary> {
    //     let branch = self.git_service.get_current_branch_info().await?;

    //     // Even if provider API call fails, we want to return repo_url and current branch
    //     let (updated_at, owner) = match client.repository_metadata(url).await {
    //         Ok(repository_metadata) => (
    //             Some(repository_metadata.updated_at),
    //             Some(repository_metadata.owner),
    //         ),
    //         Err(e) => {
    //             // TODO: Tell the frontend provider API call fails
    //             println!("git provider api call fails: {}", e);

    //             (None, None)
    //         }
    //     };

    //     match git_provider_type {
    //         GitProviderType::GitHub => Ok(VcsSummary::GitHub {
    //             branch,
    //             url: url.to_string(),
    //             updated_at,
    //             owner,
    //         }),
    //         GitProviderType::GitLab => Ok(VcsSummary::GitLab {
    //             branch,
    //             url: url.to_string(),
    //             updated_at,
    //             owner,
    //         }),
    //     }
    // }

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

    pub async fn dispose(&self) -> joinerror::Result<()> {
        if let Some(git_service) = &self.git_service {
            git_service.dispose(self.fs.clone()).await?;
        }

        Ok(())
    }
}

#[cfg(any(test, feature = "integration-tests"))]
impl<R: AppRuntime> Collection<R> {
    pub fn db(&self) -> &Arc<dyn moss_storage::CollectionStorage<R::AsyncContext>> {
        self.storage_service.storage()
    }
}
