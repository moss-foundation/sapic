use git2::{BranchType, IndexAddOption, Signature};
use joinerror::{Error, OptionExt, ResultExt};
use moss_activity_broadcaster::ActivityBroadcaster;
use moss_applib::{AppRuntime, subscription::EventEmitter};
use moss_fs::{CreateOptions, FileSystem, FsResultExt, RemoveOptions};
use moss_git::{
    // repo::{BranchType, IndexAddOption, RepoHandle, Signature},
    repository::Repository,
    url::normalize_git_url,
};
use moss_git_hosting_provider::{
    GitAuthProvider, GitHostingProvider, models::primitives::GitProviderType,
};
use moss_user::models::primitives::AccountId;

use moss_logging::session;
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
    git::GitClient,
    manifest::{MANIFEST_FILE_NAME, ManifestFile, ManifestVcs},
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

/// List of files that are always created when a collection is created.
/// This list should include only files whose content is fixed and doesn't
/// depend on any parameters or conditions.
///
/// Example:
/// * .gitignore — This file is always created with the exact same content, regardless of context.
/// * config.json — While it's always created, its content depends on the specific parameters of the
/// collection being created, so it is **not included** in the list of predefined files.
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
    pub account_id: AccountId,
    pub git_params: CollectionCloneGitParams,
}

pub struct CollectionCloneGitParams {
    pub git_provider_type: GitProviderType,
    pub repository: String,
    pub branch: Option<String>,
}

pub struct CollectionBuilder<R: AppRuntime> {
    fs: Arc<dyn FileSystem>,
    broadcaster: ActivityBroadcaster<R::EventLoop>,
    abs_path: Arc<Path>,
    // conf: ConfigFile,
    // github_client: Arc<GitHubClient>,
    // gitlab_client: Arc<GitLabClient>,
}

impl<R: AppRuntime> CollectionBuilder<R> {
    pub async fn new(
        fs: Arc<dyn FileSystem>,
        broadcaster: ActivityBroadcaster<R::EventLoop>,

        // Collection internal path
        abs_path: Arc<Path>,
    ) -> joinerror::Result<Self> {
        Ok(Self {
            fs,
            broadcaster,
            abs_path,
            // conf,
        })
    }

    // pub fn has_repository(&self) -> (bool, Option<AccountId>) {
    //     if let Some(account_id) = &self.conf.account_id {
    //         (
    //             true && self.abs_path.join(".git").exists(),
    //             Some(account_id.clone()),
    //         )
    //     } else {
    //         (false, None)
    //     }
    // }

    pub async fn load(
        self,
        git_client: Option<GitClient>,
        params: CollectionLoadParams,
    ) -> joinerror::Result<Collection<R>> {
        debug_assert!(params.internal_abs_path.is_absolute());

        let storage_service: Arc<StorageService<R>> =
            StorageService::new(params.internal_abs_path.as_ref())
                .join_err::<()>("failed to create collection storage service")?
                .into();

        let worktree_service: Arc<Worktree<R>> = Worktree::new(
            params.internal_abs_path.clone(),
            self.fs.clone(),
            self.broadcaster.clone(),
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

        let vcs_service = if let Some(git_client) = git_client {
            let repository = self
                .load_repo_handle(
                    manifest.vcs.map(|vcs| vcs.provider()),
                    params.internal_abs_path.clone(),
                )
                .await;

            Some(Arc::new(GitService::new(repository, git_client)))
        } else {
            None
        };

        // let repo_handle = if params.internal_abs_path.join(".git").exists() {
        //     self.load_repo_handle(
        //         manifest.repository.map(|repo| repo.git_provider_type),
        //         params.internal_abs_path.clone(),
        //     )
        //     .await
        // } else {
        //     None
        // };

        // let git_service = Arc::new(GitService::new(repository, git_client));

        Ok(Collection {
            fs: self.fs.clone(),
            abs_path: params.internal_abs_path,
            edit,
            set_icon_service,
            storage_service,
            git_service: vcs_service,
            // git_client: git_client,
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
            self.broadcaster.clone(),
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
        let vcs = params.git_params.as_ref().and_then(|p| {
            match normalize_git_url(&p.repository) {
                Ok(normalized_repository) => match p.git_provider_type {
                    GitProviderType::GitHub => Some(ManifestVcs::GitHub {
                        repository: normalized_repository,
                    }),
                    GitProviderType::GitLab => Some(ManifestVcs::GitLab {
                        repository: normalized_repository,
                    }),
                },
                Err(e) => {
                    // TODO: let the frontend know we cannot normalize the repository
                    session::error!(format!(
                        "failed to normalize repository url `{}`: {}",
                        p.repository,
                        e.to_string()
                    ));
                    None
                }
            }
        });

        self.fs
            .create_file_with(
                &abs_path.join(MANIFEST_FILE_NAME),
                serde_json::to_string(&ManifestFile {
                    name: params
                        .name
                        .unwrap_or(defaults::DEFAULT_COLLECTION_NAME.to_string()),
                    // INFO: We might consider removing this field from the manifest file
                    vcs,
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
                    account_id: None,
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

        // let repo_handle = if let Some(git_params) = params.git_params {
        //     let git_provider_type = git_params.git_provider_type;
        //     let repository = git_params.repository;
        //     let branch = git_params.branch;

        //     let result = self
        //         .create_repo_handle(git_provider_type, abs_path.clone(), repository, branch)
        //         .await;

        //     match result {
        //         Ok(repo_handle) => Some(repo_handle),
        //         Err(err) => {
        //             // TODO: send the error to the frontend
        //             println!("failed to create repo: {}", err.to_string());
        //             None
        //         }
        //     }
        // } else {
        //     None
        // };

        // let git_service = Arc::new(GitService::new(repo_handle, self.git_client.clone()));

        Ok(Collection {
            fs: self.fs.clone(),
            abs_path: params.internal_abs_path.to_owned().into(),
            edit,
            set_icon_service,
            storage_service,
            git_service: None,
            // git_client: self.git_client,
            worktree: worktree_service,
            on_did_change: EventEmitter::new(),
        })
    }

    // TODO: Handle non-collection repo
    pub async fn clone(
        self,
        ctx: &R::AsyncContext,
        git_client: GitClient,
        params: CollectionCloneParams,
    ) -> joinerror::Result<Collection<R>> {
        debug_assert!(params.internal_abs_path.is_absolute());

        let abs_path = params.internal_abs_path.clone();
        let repository = self
            .do_clone(
                &git_client,
                params.git_params.git_provider_type,
                abs_path.clone(),
                params.git_params.repository,
                params.git_params.branch,
            )
            .await?;

        let git_service = Arc::new(GitService::new(Some(repository), git_client));

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
            self.broadcaster.clone(),
            storage_service.clone(),
        )
        .into();

        let set_icon_service =
            SetIconService::new(abs_path.clone(), self.fs.clone(), COLLECTION_ICON_SIZE);

        self.fs
            .create_file_with(
                &abs_path.join(CONFIG_FILE_NAME),
                serde_json::to_string(&ConfigFile {
                    account_id: Some(params.account_id),
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
            git_service: Some(git_service),
            // git_client: self.git_client,
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
    ) -> Option<Repository> {
        // let github_client_clone = self.github_client.clone();
        // let gitlab_client_clone = self.gitlab_client.clone();

        // let join = tokio::task::spawn_blocking(move || {
        //     let auth_agent = match git_provider_type {
        //         None => {
        //             return Err(joinerror::Error::new::<()>(
        //                 "no git provider type provided for reloading a repo",
        //             ));
        //         }
        //         Some(GitProviderType::GitHub) => github_client_clone.git_auth_agent(),
        //         Some(GitProviderType::GitLab) => gitlab_client_clone.git_auth_agent(),
        //     };
        //     Ok(RepoHandle::open(abs_path.as_ref(), auth_agent)?)
        // })
        // .await;

        // match join {
        //     Ok(Ok(repo_handle)) => Some(repo_handle),
        //     Ok(Err(err)) => {
        //         // TODO: log the error
        //         println!("failed to open local repo: {}", err.to_string());
        //         None
        //     }
        //     Err(err) => {
        //         // TODO: log the error
        //         println!("failed to open local repo: {}", err.to_string());
        //         None
        //     }
        // }

        // Ok(Repository::open(abs_path.as_ref())?)

        // TODO: helper function should not decide whether to return a repo handle or a repository
        // It should be the responsibility of the caller to decide what to do with the result
        match Repository::open(abs_path.as_ref()) {
            Ok(repo) => Some(repo),
            Err(err) => {
                // TODO: log the error
                println!("failed to open local repo: {}", err.to_string());
                None
            }
        }
    }

    async fn create_repo_handle(
        &self,
        git_client: &GitClient,
        git_provider_type: GitProviderType,
        abs_path: Arc<Path>,
        repo_url: String,
        default_branch: String,
    ) -> joinerror::Result<Repository> {
        // let github_client_clone = self.github_client.clone();
        // let gitlab_client_clone = self.gitlab_client.clone();
        let abs_path_clone = abs_path.clone();

        // let user_info = match git_provider_type {
        //     GitProviderType::GitHub => github_client_clone.current_user().await?,
        //     GitProviderType::GitLab => gitlab_client_clone.current_user().await?,
        // };

        // let result = tokio::task::spawn_blocking(move || {
        //     let auth_agent = match git_provider_type {
        //         GitProviderType::GitHub => github_client_clone.git_auth_agent(),
        //         GitProviderType::GitLab => gitlab_client_clone.git_auth_agent(),
        //     };

        //     // git init
        //     // Note: This will not create a default branch
        //     let repo_handle = RepoHandle::init(abs_path_clone.as_ref(), auth_agent)?;

        //     // git remote add origin {repository_url}
        //     repo_handle.add_remote(None, &repo_url)?;

        //     // git fetch
        //     repo_handle.fetch(None)?;

        //     // Check remote branches
        //     // git branch -r
        //     let remote_branches = repo_handle.list_branches(Some(BranchType::Remote))?;

        //     // We will push a default branch to the remote, if no remote branches exist
        //     // TODO: Support connecting with a remote repo that already has branches?
        //     if !remote_branches.is_empty() {
        //         return Err(Error::new::<()>(
        //             "connecting with a non-empty repo is unimplemented",
        //         ));
        //     }

        //     // git add .
        //     repo_handle.add(["."].iter(), IndexAddOption::DEFAULT)?;

        //     // git commit
        //     // This will create a default branch
        //     let author = Signature::now(&user_info.name, &user_info.email).map_err(|e| {
        //         joinerror::Error::new::<()>(format!(
        //             "failed to generate commit signature: {}",
        //             e.to_string()
        //         ))
        //     })?;
        //     repo_handle.commit("Initial Commit", author)?;

        //     // git branch -m {old_default_branch_name} {default_branch_name}
        //     let old_default_branch_name = repo_handle
        //         .list_branches(Some(BranchType::Local))?
        //         .first()
        //         .cloned()
        //         .ok_or_join_err::<()>("no local branch exists")?;
        //     repo_handle.rename_branch(&old_default_branch_name, &default_branch, false)?;

        //     // Don't push during integration tests
        //     // git push
        //     #[cfg(not(any(test, feature = "integration-tests")))]
        //     repo_handle.push(None, Some(&default_branch), Some(&default_branch), true)?;

        //     Ok(repo_handle)
        // })
        // .await;

        // // If the repo operations fail, we want to clean up the .git folder
        // // So that the next time we can re-start git operations in a clean state
        // match result {
        //     Ok(Ok(repo_handle)) => Ok(repo_handle),
        //     Ok(Err(err)) => {
        //         self.fs
        //             .remove_dir(
        //                 &abs_path.join(".git"),
        //                 RemoveOptions {
        //                     recursive: true,
        //                     ignore_if_not_exists: true,
        //                 },
        //             )
        //             .await?;
        //         Err(err.join::<()>("failed to complete git operations"))
        //     }
        //     Err(err) => {
        //         self.fs
        //             .remove_dir(
        //                 &abs_path.join(".git"),
        //                 RemoveOptions {
        //                     recursive: true,
        //                     ignore_if_not_exists: true,
        //                 },
        //             )
        //             .await?;
        //         Err(Error::from(err).join::<()>("failed to complete git operations"))
        //     }
        // }

        let (access_token, username) = match git_client {
            GitClient::GitHub { account, .. } => {
                (account.session().access_token().await?, account.username())
            }
            GitClient::GitLab { account, .. } => {
                (account.session().access_token().await?, account.username())
            }
        };
        let mut cb = git2::RemoteCallbacks::new();
        let username_clone = username.clone();
        let access_token_clone = access_token.clone();
        cb.credentials(move |_url, username_from_url, _allowed| {
            // let rt = tokio::runtime::Handle::try_current();
            // let fut = self.session_for_remote(ws, repo_root, remote_name);
            // let (acc, tok) = match rt {
            //     Ok(h) => h.block_on(fut),
            //     Err(_) => tokio::runtime::Runtime::new().unwrap().block_on(fut),
            // }
            // .map_err(|e| git2::Error::from_str(&format!("auth error: {e}")))?;
            // let user = username_from_url.unwrap_or(&acc.login);

            git2::Cred::userpass_plaintext(
                username_from_url.unwrap_or(&username_clone),
                &access_token_clone,
            )
        });

        let repository = Repository::init(abs_path_clone.as_ref())?;
        repository.add_remote(None, &repo_url)?;
        repository.fetch(None, cb)?;

        let remote_branches = repository.list_branches(Some(BranchType::Remote))?;

        // We will push a default branch to the remote, if no remote branches exist
        // TODO: Support connecting with a remote repo that already has branches?
        if !remote_branches.is_empty() {
            return Err(Error::new::<()>(
                "connecting with a non-empty repo is unimplemented",
            ));
        }

        repository.add_all(["."].iter(), IndexAddOption::DEFAULT)?;
        let author = Signature::now(
            &username,
            // FIXME: This is a temporary solution to avoid the error
            format!("{}@git.noreply.com", username).as_str(),
        )
        .map_err(|e| {
            joinerror::Error::new::<()>(format!(
                "failed to generate commit signature: {}",
                e.to_string()
            ))
        })?;
        repository.commit("Initial Commit", author)?;

        let old_default_branch_name = repository
            .list_branches(Some(BranchType::Local))?
            .first()
            .cloned()
            .ok_or_join_err::<()>("no local branch exists")?;
        repository.rename_branch(&old_default_branch_name, &default_branch, false)?;

        // Don't push during integration tests
        // git push
        #[cfg(not(any(test, feature = "integration-tests")))]
        {
            let mut cb = git2::RemoteCallbacks::new();
            let username_clone = username.clone();
            cb.credentials(move |_url, username_from_url, _allowed| {
                // let rt = tokio::runtime::Handle::try_current();
                // let fut = self.session_for_remote(ws, repo_root, remote_name);
                // let (acc, tok) = match rt {
                //     Ok(h) => h.block_on(fut),
                //     Err(_) => tokio::runtime::Runtime::new().unwrap().block_on(fut),
                // }
                // .map_err(|e| git2::Error::from_str(&format!("auth error: {e}")))?;
                // let user = username_from_url.unwrap_or(&acc.login);

                git2::Cred::userpass_plaintext(
                    username_from_url.unwrap_or(&username_clone),
                    &access_token,
                )
            });
            repository.push(None, Some(&default_branch), Some(&default_branch), true, cb)?;
        }

        Ok(repository)
    }

    async fn do_clone(
        &self,
        git_client: &GitClient,
        git_provider_type: GitProviderType,
        abs_path: Arc<Path>,
        repo_url: String,
        branch: Option<String>,
    ) -> joinerror::Result<Repository> {
        // let github_client_clone = self.github_client.clone();
        // let gitlab_client_clone = self.gitlab_client.clone();

        // let join = tokio::task::spawn_blocking(move || {
        //     let auth_agent = match git_provider_type {
        //         GitProviderType::GitHub => github_client_clone.git_auth_agent(),
        //         GitProviderType::GitLab => gitlab_client_clone.git_auth_agent(),
        //     };

        //     let repo = RepoHandle::clone(
        //         &repo_url,
        //         abs_path.as_ref(),
        //         // Different git providers require different auth agent
        //         auth_agent,
        //     )?;

        //     if let Some(branch) = branch {
        //         // Try to check out to the user-selected branch
        //         // if it fails, we consider the repo creation to also fail
        //         repo.checkout_branch(None, &branch, true)?;
        //     }

        //     Ok(repo)
        // })
        // .await;

        // match join {
        //     Ok(Ok(repo_handle)) => Ok(repo_handle),
        //     Ok(Err(err)) => Err(err),
        //     Err(err) => Err(err.into()),
        // }

        ////////

        let (access_token, username) = match git_client {
            GitClient::GitHub { account, .. } => {
                (account.session().access_token().await?, account.username())
            }
            GitClient::GitLab { account, .. } => {
                (account.session().access_token().await?, account.username())
            }
        };
        let mut cb = git2::RemoteCallbacks::new();
        cb.credentials(move |_url, username_from_url, _allowed| {
            // let rt = tokio::runtime::Handle::try_current();
            // let fut = self.session_for_remote(ws, repo_root, remote_name);
            // let (acc, tok) = match rt {
            //     Ok(h) => h.block_on(fut),
            //     Err(_) => tokio::runtime::Runtime::new().unwrap().block_on(fut),
            // }
            // .map_err(|e| git2::Error::from_str(&format!("auth error: {e}")))?;
            // let user = username_from_url.unwrap_or(&acc.login);

            git2::Cred::userpass_plaintext(username_from_url.unwrap_or(&username), &access_token)
        });

        let repository = Repository::clone(&repo_url, abs_path.as_ref(), cb)?;

        if let Some(branch) = branch {
            // Try to check out to the user-selected branch
            // if it fails, we consider the repo creation to also fail
            repository.checkout_branch(None, &branch, true)?;
        }

        Ok(repository)
    }
}
