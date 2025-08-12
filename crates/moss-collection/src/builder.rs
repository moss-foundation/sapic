use joinerror::{Error, OptionExt, ResultExt};
use moss_applib::{AppRuntime, subscription::EventEmitter};
use moss_fs::{CreateOptions, FileSystem, RemoveOptions};
use moss_git::{
    repo::{BranchType, IndexAddOption, RepoHandle, Signature},
    url::normalize_git_url,
};
use moss_git_hosting_provider::{
    GitHostingProvider, common::GitProviderType, github::client::GitHubClient,
    gitlab::client::GitLabClient,
};
use moss_hcl::Block;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::{Mutex, OnceCell};

use crate::{
    Collection,
    config::{CONFIG_FILE_NAME, ConfigFile},
    constants::COLLECTION_ROOT_PATH,
    defaults, dirs,
    edit::CollectionEdit,
    manifest::{MANIFEST_FILE_NAME, ManifestFile},
    models::{
        primitives::EntryId,
        types::configuration::docschema::{
            RawDirComponentConfiguration, RawDirConfiguration, RawDirEndpointConfiguration,
            RawDirRequestConfiguration, RawDirSchemaConfiguration,
        },
    },
    services::{
        set_icon_service::SetIconService,
        storage_service::StorageService,
        worktree_service::{EntryMetadata, WorktreeService},
    },
};

const COLLECTION_ICON_SIZE: u32 = 128;
const OTHER_DIRS: [&str; 2] = [dirs::ASSETS_DIR, dirs::ENVIRONMENTS_DIR];

const WORKTREE_DIRS: [(&str, isize); 4] = [
    (dirs::REQUESTS_DIR, 0),
    (dirs::ENDPOINTS_DIR, 1),
    (dirs::COMPONENTS_DIR, 2),
    (dirs::SCHEMAS_DIR, 3),
];

pub struct CollectionCreateParams {
    pub name: Option<String>,
    pub internal_abs_path: Arc<Path>,
    pub external_abs_path: Option<Arc<Path>>,
    pub repository: Option<String>,
    pub icon_path: Option<PathBuf>,
}

pub struct CollectionLoadParams {
    pub internal_abs_path: Arc<Path>,
}

pub struct CollectionCloneParams {
    pub git_provider_type: GitProviderType,
    pub internal_abs_path: Arc<Path>,
    pub repository: String,
}

pub struct CollectionBuilder {
    fs: Arc<dyn FileSystem>,
    //
    github_client: Arc<GitHubClient>,
    gitlab_client: Arc<GitLabClient>,
}

impl CollectionBuilder {
    pub fn new(
        fs: Arc<dyn FileSystem>,
        github_client: Arc<GitHubClient>,
        gitlab_client: Arc<GitLabClient>,
    ) -> Self {
        Self {
            fs,
            github_client,
            gitlab_client,
        }
    }

    pub async fn load<R: AppRuntime>(
        self,
        params: CollectionLoadParams,
    ) -> joinerror::Result<Collection<R>> {
        debug_assert!(params.internal_abs_path.is_absolute());

        let storage_service: Arc<StorageService<R>> =
            StorageService::new(params.internal_abs_path.as_ref())
                .join_err::<()>("failed to create collection storage service")?
                .into();

        let worktree_service: Arc<WorktreeService<R>> = WorktreeService::new(
            params.internal_abs_path.clone(),
            self.fs.clone(),
            storage_service.clone(),
        )
        .into();

        let set_icon_service = SetIconService::new(
            params.internal_abs_path.clone(),
            self.fs.clone(),
            COLLECTION_ICON_SIZE,
        );

        let edit = CollectionEdit::new(
            self.fs.clone(),
            params.internal_abs_path.join(MANIFEST_FILE_NAME),
        );
        // TODO: Load environments

        // TODO: Load Git repo
        Ok(Collection {
            fs: self.fs.clone(),
            abs_path: params.internal_abs_path,
            edit,
            set_icon_service,
            storage_service,
            worktree_service,
            environments: OnceCell::new(),
            on_did_change: EventEmitter::new(),
            repo_handle: Arc::new(Mutex::new(None)),
        })
    }

    // TODO: A nicer functionality to have might be to help the user create a remote Git repository
    // Using Git provider API, and help them make the initial commit
    // I'll come to this question later.

    pub async fn create<R: AppRuntime>(
        self,
        ctx: &R::AsyncContext,
        params: CollectionCreateParams,
    ) -> joinerror::Result<Collection<R>> {
        debug_assert!(params.internal_abs_path.is_absolute());

        let abs_path: Arc<Path> = params
            .external_abs_path
            .clone()
            .unwrap_or(params.internal_abs_path.clone())
            .into();

        let storage_service: Arc<StorageService<R>> = StorageService::new(abs_path.as_ref())
            .join_err::<()>("failed to create collection storage service")?
            .into();

        let worktree_service: Arc<WorktreeService<R>> =
            WorktreeService::new(abs_path.clone(), self.fs.clone(), storage_service.clone()).into();

        let set_icon_service =
            SetIconService::new(abs_path.clone(), self.fs.clone(), COLLECTION_ICON_SIZE);

        for (dir, order) in &WORKTREE_DIRS {
            let id = EntryId::new();
            let configuration = match *dir {
                dirs::REQUESTS_DIR => {
                    RawDirConfiguration::Request(Block::new(RawDirRequestConfiguration::new(&id)))
                }
                dirs::ENDPOINTS_DIR => {
                    RawDirConfiguration::Endpoint(Block::new(RawDirEndpointConfiguration::new(&id)))
                }
                dirs::COMPONENTS_DIR => RawDirConfiguration::Component(Block::new(
                    RawDirComponentConfiguration::new(&id),
                )),
                dirs::SCHEMAS_DIR => {
                    RawDirConfiguration::Schema(Block::new(RawDirSchemaConfiguration::new(&id)))
                }
                _ => unreachable!(),
            };

            worktree_service
                .create_dir_entry(
                    ctx,
                    &id,
                    dir,
                    Path::new(COLLECTION_ROOT_PATH),
                    configuration,
                    EntryMetadata {
                        order: *order,
                        expanded: false,
                    },
                )
                .await?;
        }

        for dir in &OTHER_DIRS {
            self.fs.create_dir(&abs_path.join(dir)).await?;
        }

        if let Some(icon_path) = params.icon_path {
            // TODO: Log the error here
            set_icon_service.set_icon(&icon_path)?;
        }

        self.fs
            .create_file_with(
                &abs_path.join(MANIFEST_FILE_NAME),
                serde_json::to_string(&ManifestFile {
                    name: params
                        .name
                        .unwrap_or(defaults::DEFAULT_COLLECTION_NAME.to_string()),
                    // FIXME: We might consider removing this field from the manifest file
                    repository: params
                        .repository
                        .as_ref()
                        .and_then(|repo| normalize_git_url(repo).ok()),
                })?
                .as_bytes(),
                CreateOptions {
                    overwrite: false,
                    ignore_if_exists: false,
                },
            )
            .await?;

        self.fs
            .create_file_with(
                &params.internal_abs_path.join(CONFIG_FILE_NAME),
                serde_json::to_string(&ConfigFile {
                    external_path: params.external_abs_path.map(|p| p.to_path_buf()),
                })?
                .as_bytes(),
                CreateOptions {
                    overwrite: false,
                    ignore_if_exists: false,
                },
            )
            .await?;

        let edit = CollectionEdit::new(self.fs.clone(), abs_path.join(MANIFEST_FILE_NAME));

        let repo_handle = Arc::new(std::sync::Mutex::new(None));

        if let Some(repository) = params.repository {
            // TODO: Automatically generate a README

            // We will try to initiate a local repo, make initial commit, and push it to the remote
            // If any of the steps failed, we consider repo creation to have failed
            // We will then delete the .git folder, allowing for clean creation in the future

            self.fs
                .create_file_with(
                    &abs_path.join(".gitignore"),
                    "config.json
**/state.db"
                        .as_bytes(),
                    CreateOptions {
                        overwrite: false,
                        ignore_if_exists: false,
                    },
                )
                .await?;

            let abs_path_clone = abs_path.clone();
            let repo_handle_clone = repo_handle.clone();

            let _github_client_clone = self.github_client.clone();
            let gitlab_client_clone = self.gitlab_client.clone();

            // FIXME: Use the actual git provider and auth agent based on user input
            // Hardcoded using GitHub auth agent for now
            let client = gitlab_client_clone;
            let user_info = client.current_user().await?;

            let result = tokio::task::spawn_blocking(move || {
                // TODO: Allow the user to set the default branch name
                let new_default_branch_name = "main";

                // git init
                // Note: This will not create a default branch
                let repo_handle = Arc::new(RepoHandle::init(
                    abs_path_clone.as_ref(),
                    client.git_auth_agent(),
                )?);

                // git remote add origin {repository_url}
                repo_handle.add_remote(Some("origin"), &repository)?;

                // git fetch
                repo_handle.fetch(Some("origin"))?;

                // Check remote branches
                // git branch -r
                let remote_branches = repo_handle.list_branches(Some(BranchType::Remote))?;

                // We will push a default branch to the remote, if no remote branches exist
                // TODO: Support connecting with a remote repo that already has branches?
                if !remote_branches.is_empty() {
                    return Err(Error::new::<()>(
                        "connecting with a non-empty repo is unimplemented",
                    ));
                }

                // git add .
                repo_handle.add(["."].iter(), IndexAddOption::DEFAULT)?;

                // git commit
                // This will create a default branch

                let author = Signature::now(&user_info.name, &user_info.email).map_err(|e| {
                    joinerror::Error::new::<()>(format!(
                        "failed to generate commit signature: {}",
                        e.to_string()
                    ))
                })?;
                repo_handle.commit("Initial Commit", author)?;

                // git branch -m {old_default_branch_name} {default_branch_name}
                let old_default_branch_name = repo_handle
                    .list_branches(Some(BranchType::Local))?
                    .first()
                    .cloned()
                    .ok_or_join_err::<()>("no local branch exists")?;
                repo_handle.rename_branch(
                    &old_default_branch_name,
                    new_default_branch_name,
                    false,
                )?;

                // Don't push during integration tests
                // git push
                #[cfg(not(any(test, feature = "integration-tests")))]
                repo_handle.push(
                    None,
                    Some(&new_default_branch_name),
                    Some(&new_default_branch_name),
                    true,
                )?;

                *(repo_handle_clone.lock()?) = Some(repo_handle);
                Ok::<(), Error>(())
            })
            .await;

            // If the repo operations fail, we want to clean up the .git folder
            // So that the next time we can re-start git operations in a clean state
            match result {
                Ok(Ok(())) => {}
                Ok(Err(e)) => {
                    // TODO: tell the frontend that git operations failed
                    println!("failed to complete git operations: {}", e.to_string());
                    self.fs
                        .remove_dir(
                            &abs_path.join(".git"),
                            RemoveOptions {
                                recursive: true,
                                ignore_if_not_exists: true,
                            },
                        )
                        .await?;
                }
                Err(e) => {
                    println!("failed to complete git operations: {}", e.to_string());
                    self.fs
                        .remove_dir(
                            &abs_path.join(".git"),
                            RemoveOptions {
                                recursive: true,
                                ignore_if_not_exists: true,
                            },
                        )
                        .await?;
                }
            }
        }

        let repo_handle = repo_handle.lock()?.take();

        let repo_handle = if let Some(repo_handle) = repo_handle {
            // This should always succeed, since the other reference should be dropped
            // Once the spawned thread ends
            Arc::try_unwrap(repo_handle).ok()
        } else {
            None
        };

        // TODO: Load environments

        Ok(Collection {
            fs: self.fs.clone(),
            abs_path: params.internal_abs_path.to_owned().into(),
            edit,
            set_icon_service,
            storage_service,
            worktree_service,
            environments: OnceCell::new(),
            on_did_change: EventEmitter::new(),
            repo_handle: Arc::new(Mutex::new(repo_handle)),
        })
    }

    pub async fn clone<R: AppRuntime>(
        self,
        _ctx: &R::AsyncContext,
        params: CollectionCloneParams,
    ) -> joinerror::Result<Collection<R>> {
        debug_assert!(params.internal_abs_path.is_absolute());

        let abs_path = params.internal_abs_path.clone();

        let abs_path_clone = abs_path.clone();

        let github_client_clone = self.github_client.clone();
        let gitlab_client_clone = self.gitlab_client.clone();
        let join = tokio::task::spawn_blocking(move || {
            let auth_agent = match params.git_provider_type {
                GitProviderType::GitHub => github_client_clone.git_auth_agent(),
                GitProviderType::GitLab => gitlab_client_clone.git_auth_agent(),
            };
            Ok(RepoHandle::clone(
                &params.repository,
                abs_path_clone.as_ref(),
                // Different git providers require different auth agent
                auth_agent,
            )?)
        })
        .await;

        let repo_handle = match join {
            Ok(Ok(repo_handle)) => repo_handle,
            Ok(Err(err)) => return Err(err),
            Err(err) => return Err(err.into()),
        };

        let storage_service: Arc<StorageService<R>> = StorageService::new(abs_path.as_ref())
            .join_err::<()>("failed to create collection storage service")?
            .into();

        let worktree_service: Arc<WorktreeService<R>> =
            WorktreeService::new(abs_path.clone(), self.fs.clone(), storage_service.clone()).into();

        let set_icon_service =
            SetIconService::new(abs_path.clone(), self.fs.clone(), COLLECTION_ICON_SIZE);

        self.fs
            .create_file_with(
                &abs_path.join(CONFIG_FILE_NAME),
                serde_json::to_string(&ConfigFile {
                    external_path: None,
                })?
                .as_bytes(),
                CreateOptions {
                    overwrite: false,
                    ignore_if_exists: false,
                },
            )
            .await?;

        let edit = CollectionEdit::new(self.fs.clone(), abs_path.join(MANIFEST_FILE_NAME));

        // TODO: Load environments

        Ok(Collection {
            fs: self.fs.clone(),
            abs_path,
            edit,
            set_icon_service,
            storage_service,
            worktree_service,
            environments: OnceCell::new(),
            on_did_change: EventEmitter::new(),
            repo_handle: Arc::new(Mutex::new(Some(repo_handle))),
        })
    }
}
