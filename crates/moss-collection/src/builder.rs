use joinerror::{Error, OptionExt, ResultExt};
use moss_activity_indicator::ActivityIndicator;
use moss_applib::{AppRuntime, subscription::EventEmitter};
use moss_fs::{CreateOptions, FileSystem, FsResultExt, RemoveOptions};
use moss_git::repo::{BranchType, IndexAddOption, RepoHandle, Signature};
use moss_git_hosting_provider::{
    GitAuthProvider, GitHostingProvider, github::client::GitHubClient,
    gitlab::client::GitLabClient, models::primitives::GitProviderType,
};
use std::{
    cell::LazyCell,
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::{
    Collection,
    config::{CONFIG_FILE_NAME, ConfigFile},
    constants::COLLECTION_ROOT_PATH,
    defaults, dirs,
    edit::CollectionEdit,
    manifest::{MANIFEST_FILE_NAME, ManifestFile, ManifestRepository},
    models::primitives::{EntryClass, EntryId},
    services::{
        git_service::GitService, set_icon_service::SetIconService, storage_service::StorageService,
    },
    worktree::{Worktree, entry::model::EntryModel},
};

const COLLECTION_ICON_SIZE: u32 = 128;
const OTHER_DIRS: [&str; 2] = [dirs::ASSETS_DIR, dirs::ENVIRONMENTS_DIR];

const WORKTREE_DIRS: [(&str, isize); 4] = [
    (dirs::REQUESTS_DIR, 0),
    (dirs::ENDPOINTS_DIR, 1),
    (dirs::COMPONENTS_DIR, 2),
    (dirs::SCHEMAS_DIR, 3),
];

struct PredefinedFile {
    path: PathBuf,
    content: Vec<u8>,
}

const PREDEFINED_FILES: LazyCell<Vec<PredefinedFile>> = LazyCell::new(|| {
    vec![PredefinedFile {
        path: PathBuf::from(".gitignore"),
        content: "config.json\n**/state.db".as_bytes().to_vec(),
    }]
});

pub struct CollectionCreateParams {
    pub name: Option<String>,
    pub internal_abs_path: Arc<Path>,
    pub external_abs_path: Option<Arc<Path>>,
    pub git_params: Option<CollectionCreateGitParams>,
    pub icon_path: Option<PathBuf>,
}

pub struct CollectionCreateGitParams {
    pub git_provider_type: GitProviderType,
    pub repository: String,
    pub branch: String,
}

pub struct CollectionLoadParams {
    pub internal_abs_path: Arc<Path>,
}

pub struct CollectionCloneParams {
    pub internal_abs_path: Arc<Path>,
    pub git_params: CollectionCloneGitParams,
}

pub struct CollectionCloneGitParams {
    pub git_provider_type: GitProviderType,
    pub repository: String,
    pub branch: Option<String>,
}

pub struct CollectionBuilder<R: AppRuntime> {
    fs: Arc<dyn FileSystem>,
    activity_indicator: ActivityIndicator<R::EventLoop>,
    github_client: Arc<GitHubClient>,
    gitlab_client: Arc<GitLabClient>,
}

impl<R: AppRuntime> CollectionBuilder<R> {
    pub fn new(
        fs: Arc<dyn FileSystem>,
        activity_indicator: ActivityIndicator<R::EventLoop>,
        github_client: Arc<GitHubClient>,
        gitlab_client: Arc<GitLabClient>,
    ) -> Self {
        Self {
            fs,
            activity_indicator,
            github_client,
            gitlab_client,
        }
    }

    pub async fn load(self, params: CollectionLoadParams) -> joinerror::Result<Collection<R>> {
        debug_assert!(params.internal_abs_path.is_absolute());

        let storage_service: Arc<StorageService<R>> =
            StorageService::new(params.internal_abs_path.as_ref())
                .join_err::<()>("failed to create collection storage service")?
                .into();

        let worktree_service: Arc<Worktree<R>> = Worktree::new(
            params.internal_abs_path.clone(),
            self.fs.clone(),
            self.activity_indicator.clone(),
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

        // FIXME: This logic needs to be updated when we implement support for external collections
        // Since the repo should be at the external_abs_path
        let manifest_path = params.internal_abs_path.join(MANIFEST_FILE_NAME);

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

        let repo_handle = if params.internal_abs_path.join(".git").exists() {
            self.load_repo_handle(
                manifest.repository.map(|repo| repo.git_provider_type),
                params.internal_abs_path.clone(),
            )
            .await
        } else {
            None
        };

        let git_service = Arc::new(GitService::new(repo_handle));

        Ok(Collection {
            fs: self.fs.clone(),
            abs_path: params.internal_abs_path,
            edit,
            set_icon_service,
            storage_service,
            git_service,
            github_client: self.github_client.clone(),
            gitlab_client: self.gitlab_client.clone(),
            worktree: worktree_service,
            on_did_change: EventEmitter::new(),
        })
    }

    pub async fn create(
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

        // Create expandedEntries key in the database to prevent warnings
        storage_service
            .put_expanded_entries(ctx, Vec::new())
            .await?;

        let worktree_service: Arc<Worktree<R>> = Worktree::new(
            abs_path.clone(),
            self.fs.clone(),
            self.activity_indicator.clone(),
            storage_service.clone(),
        )
        .into();

        let set_icon_service =
            SetIconService::new(abs_path.clone(), self.fs.clone(), COLLECTION_ICON_SIZE);

        for (dir, order) in &WORKTREE_DIRS {
            let id = EntryId::new();
            let model = match *dir {
                dirs::REQUESTS_DIR => EntryModel::from((id, EntryClass::Request)),
                dirs::ENDPOINTS_DIR => EntryModel::from((id, EntryClass::Endpoint)),
                dirs::COMPONENTS_DIR => EntryModel::from((id, EntryClass::Component)),
                dirs::SCHEMAS_DIR => EntryModel::from((id, EntryClass::Schema)),
                _ => unreachable!(),
            };

            worktree_service
                .create_dir_entry(
                    ctx,
                    dir,
                    Path::new(COLLECTION_ROOT_PATH),
                    model,
                    *order,
                    false,
                )
                .await?;
        }

        for dir in &OTHER_DIRS {
            self.fs.create_dir(&abs_path.join(dir)).await?;
        }

        if let Some(icon_path) = params.icon_path {
            if let Err(err) = set_icon_service.set_icon(&icon_path) {
                // TODO: Log the error here
                println!("failed to set collection icon: {}", err.to_string());
            }
        }

        // FIXME: I'm not sure why we need to store a repo url that's different from what we expect from the user
        let repository = params.git_params.as_ref().map(|p| ManifestRepository {
            url: p.repository.clone(),
            git_provider_type: p.git_provider_type.clone(),
        });

        self.fs
            .create_file_with(
                &abs_path.join(MANIFEST_FILE_NAME),
                serde_json::to_string(&ManifestFile {
                    name: params
                        .name
                        .unwrap_or(defaults::DEFAULT_COLLECTION_NAME.to_string()),
                    // INFO: We might consider removing this field from the manifest file
                    repository,
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

        for file in PREDEFINED_FILES.iter() {
            self.fs
                .create_file_with(
                    &abs_path.join(&file.path),
                    file.content.as_slice(),
                    CreateOptions {
                        overwrite: false,
                        ignore_if_exists: false,
                    },
                )
                .await?;
        }

        let edit = CollectionEdit::new(self.fs.clone(), abs_path.join(MANIFEST_FILE_NAME));

        let repo_handle = if let Some(git_params) = params.git_params {
            let git_provider_type = git_params.git_provider_type;
            let repository = git_params.repository;
            let branch = git_params.branch;

            let result = self
                .create_repo_handle(git_provider_type, abs_path.clone(), repository, branch)
                .await;

            match result {
                Ok(repo_handle) => Some(repo_handle),
                Err(err) => {
                    // TODO: send the error to the frontend
                    println!("failed to create repo: {}", err.to_string());
                    None
                }
            }
        } else {
            None
        };

        let git_service = Arc::new(GitService::new(repo_handle));

        Ok(Collection {
            fs: self.fs.clone(),
            abs_path: params.internal_abs_path.to_owned().into(),
            edit,
            set_icon_service,
            storage_service,
            git_service,
            github_client: self.github_client.clone(),
            gitlab_client: self.gitlab_client.clone(),
            worktree: worktree_service,
            on_did_change: EventEmitter::new(),
        })
    }

    // TODO: Handle non-collection repo
    pub async fn clone(
        self,
        ctx: &R::AsyncContext,
        params: CollectionCloneParams,
    ) -> joinerror::Result<Collection<R>> {
        debug_assert!(params.internal_abs_path.is_absolute());

        let abs_path = params.internal_abs_path.clone();
        let repo_handle = self
            .clone_repo_handle(
                params.git_params.git_provider_type,
                abs_path.clone(),
                params.git_params.repository,
                params.git_params.branch,
            )
            .await?;

        let git_service = Arc::new(GitService::new(Some(repo_handle)));

        let storage_service: Arc<StorageService<R>> = StorageService::new(abs_path.as_ref())
            .join_err::<()>("failed to create collection storage service")?
            .into();

        // Create expandedEntries key in the database to prevent warnings
        storage_service
            .put_expanded_entries(ctx, Vec::new())
            .await?;

        let worktree: Arc<Worktree<R>> = Worktree::new(
            abs_path.clone(),
            self.fs.clone(),
            self.activity_indicator.clone(),
            storage_service.clone(),
        )
        .into();

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

        Ok(Collection {
            fs: self.fs.clone(),
            abs_path,
            edit,
            set_icon_service,
            storage_service,
            git_service,
            github_client: self.github_client.clone(),
            gitlab_client: self.gitlab_client.clone(),
            worktree,
            on_did_change: EventEmitter::new(),
        })
    }
}

impl<R: AppRuntime> CollectionBuilder<R> {
    async fn load_repo_handle(
        &self,
        git_provider_type: Option<GitProviderType>,
        abs_path: Arc<Path>,
    ) -> Option<RepoHandle> {
        let github_client_clone = self.github_client.clone();
        let gitlab_client_clone = self.gitlab_client.clone();

        let join = tokio::task::spawn_blocking(move || {
            let auth_agent = match git_provider_type {
                None => {
                    return Err(joinerror::Error::new::<()>(
                        "no git provider type provided for reloading a repo",
                    ));
                }
                Some(GitProviderType::GitHub) => github_client_clone.git_auth_agent(),
                Some(GitProviderType::GitLab) => gitlab_client_clone.git_auth_agent(),
            };
            Ok(RepoHandle::open(abs_path.as_ref(), auth_agent)?)
        })
        .await;

        match join {
            Ok(Ok(repo_handle)) => Some(repo_handle),
            Ok(Err(err)) => {
                // TODO: log the error
                println!("failed to open local repo: {}", err.to_string());
                None
            }
            Err(err) => {
                // TODO: log the error
                println!("failed to open local repo: {}", err.to_string());
                None
            }
        }
    }

    async fn create_repo_handle(
        &self,
        git_provider_type: GitProviderType,
        abs_path: Arc<Path>,
        repo_url: String,
        default_branch: String,
    ) -> joinerror::Result<RepoHandle> {
        let github_client_clone = self.github_client.clone();
        let gitlab_client_clone = self.gitlab_client.clone();
        let abs_path_clone = abs_path.clone();

        let user_info = match git_provider_type {
            GitProviderType::GitHub => github_client_clone.current_user().await?,
            GitProviderType::GitLab => gitlab_client_clone.current_user().await?,
        };

        let result = tokio::task::spawn_blocking(move || {
            let auth_agent = match git_provider_type {
                GitProviderType::GitHub => github_client_clone.git_auth_agent(),
                GitProviderType::GitLab => gitlab_client_clone.git_auth_agent(),
            };

            // git init
            // Note: This will not create a default branch
            let repo_handle = RepoHandle::init(abs_path_clone.as_ref(), auth_agent)?;

            // git remote add origin {repository_url}
            repo_handle.add_remote(None, &repo_url)?;

            // git fetch
            repo_handle.fetch(None)?;

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
            repo_handle.rename_branch(&old_default_branch_name, &default_branch, false)?;

            // Don't push during integration tests
            // git push
            #[cfg(not(any(test, feature = "integration-tests")))]
            repo_handle.push(None, Some(&default_branch), Some(&default_branch), true)?;

            Ok(repo_handle)
        })
        .await;

        // If the repo operations fail, we want to clean up the .git folder
        // So that the next time we can re-start git operations in a clean state
        match result {
            Ok(Ok(repo_handle)) => Ok(repo_handle),
            Ok(Err(err)) => {
                self.fs
                    .remove_dir(
                        &abs_path.join(".git"),
                        RemoveOptions {
                            recursive: true,
                            ignore_if_not_exists: true,
                        },
                    )
                    .await?;
                Err(err.join::<()>("failed to complete git operations"))
            }
            Err(err) => {
                self.fs
                    .remove_dir(
                        &abs_path.join(".git"),
                        RemoveOptions {
                            recursive: true,
                            ignore_if_not_exists: true,
                        },
                    )
                    .await?;
                Err(Error::from(err).join::<()>("failed to complete git operations"))
            }
        }
    }

    async fn clone_repo_handle(
        &self,
        git_provider_type: GitProviderType,
        abs_path: Arc<Path>,
        repo_url: String,
        branch: Option<String>,
    ) -> joinerror::Result<RepoHandle> {
        let github_client_clone = self.github_client.clone();
        let gitlab_client_clone = self.gitlab_client.clone();

        let join = tokio::task::spawn_blocking(move || {
            let auth_agent = match git_provider_type {
                GitProviderType::GitHub => github_client_clone.git_auth_agent(),
                GitProviderType::GitLab => gitlab_client_clone.git_auth_agent(),
            };

            let repo = RepoHandle::clone(
                &repo_url,
                abs_path.as_ref(),
                // Different git providers require different auth agent
                auth_agent,
            )?;

            if let Some(branch) = branch {
                // Try to check out to the user-selected branch
                // if it fails, we consider the repo creation to also fail
                repo.checkout_branch(None, &branch, true)?;
            }

            Ok(repo)
        })
        .await;

        match join {
            Ok(Ok(repo_handle)) => Ok(repo_handle),
            Ok(Err(err)) => Err(err),
            Err(err) => Err(err.into()),
        }
    }
}
