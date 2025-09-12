use joinerror::{Error, ResultExt};
use moss_applib::{AppRuntime, subscription::EventEmitter};
use moss_fs::{CreateOptions, FileSystem, FsResultExt};
use moss_git::{repository::Repository, url::GitUrl};
use moss_git_hosting_provider::GitProviderKind;
use moss_logging::session;
use moss_user::models::primitives::AccountId;
use std::{
    cell::LazyCell,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::OnceCell;

use crate::{
    Collection,
    config::{CONFIG_FILE_NAME, ConfigFile},
    defaults, dirs,
    edit::CollectionEdit,
    errors::ErrorIo,
    git::GitClient,
    manifest::{MANIFEST_FILE_NAME, ManifestFile, ManifestVcs},
    services::{set_icon_service::SetIconService, storage_service::StorageService},
    vcs::Vcs,
    worktree::Worktree,
};

const COLLECTION_ICON_SIZE: u32 = 128;
const OTHER_DIRS: [&str; 3] = [
    dirs::ASSETS_DIR,
    dirs::ENVIRONMENTS_DIR,
    dirs::RESOURCES_DIR,
];

struct PredefinedFile {
    path: PathBuf,
    content: Option<Vec<u8>>,
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
    vec![
        PredefinedFile {
            path: PathBuf::from(".gitignore"),
            content: Some("config.json\n**/state.db".as_bytes().to_vec()),
        },
        // ---------------------------------------------------------------------------
        // We need to create `.gitkeep` files; otherwise, when committing the collection
        // to the repository, that folder won't be included in the commit.
        //
        // This will cause errors when cloning the collection, since we expect that folder
        // to always exist.
        // ---------------------------------------------------------------------------
        PredefinedFile {
            path: PathBuf::from(format!("{}/.gitkeep", dirs::ENVIRONMENTS_DIR)),
            content: None,
        },
        PredefinedFile {
            path: PathBuf::from(format!("{}/.gitkeep", dirs::ASSETS_DIR)),
            content: None,
        },
        PredefinedFile {
            path: PathBuf::from(format!("{}/.gitkeep", dirs::RESOURCES_DIR)),
            content: None,
        },
    ]
});

pub struct CollectionCreateParams {
    pub name: Option<String>,
    pub internal_abs_path: Arc<Path>,
    pub external_abs_path: Option<Arc<Path>>,
    pub git_params: Option<CollectionCreateGitParams>,
    pub icon_path: Option<PathBuf>,
}

#[derive(Clone)]
pub struct CollectionCreateGitParams {
    pub git_provider_type: GitProviderKind,
    pub repository: GitUrl,
    pub branch: String,
}

pub struct CollectionLoadParams {
    pub internal_abs_path: Arc<Path>,
}

pub struct CollectionCloneParams {
    pub internal_abs_path: Arc<Path>,
    pub account_id: AccountId,
    pub git_provider_type: GitProviderKind,
    pub repository: GitUrl,
    pub branch: Option<String>,
}

pub struct CollectionImportParams {
    pub internal_abs_path: Arc<Path>,
    pub archive_path: Arc<Path>,
}

pub struct CollectionBuilder {
    fs: Arc<dyn FileSystem>,
}

impl CollectionBuilder {
    pub async fn new(fs: Arc<dyn FileSystem>) -> Self {
        Self { fs }
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
        let set_icon_service = SetIconService::new(
            params.internal_abs_path.clone(),
            self.fs.clone(),
            COLLECTION_ICON_SIZE,
        );

        // Check if the collection is archived
        // If so, we avoid loading the worktree service
        let config: ConfigFile = {
            let config_path = params.internal_abs_path.join(CONFIG_FILE_NAME);
            let rdr = self
                .fs
                .open_file(&config_path)
                .await
                .join_err_with::<()>(|| {
                    format!("failed to open config file: {}", config_path.display())
                })?;
            serde_json::from_reader(rdr).join_err_with::<()>(|| {
                format!("failed to parse config file: {}", config_path.display())
            })?
        };

        let worktree_service = if config.archived {
            OnceCell::new()
        } else {
            Arc::new(Worktree::new(
                params.internal_abs_path.clone(),
                self.fs.clone(),
                storage_service.clone(),
            ))
            .into()
        };

        let edit = CollectionEdit::new(
            self.fs.clone(),
            params.internal_abs_path.join(MANIFEST_FILE_NAME),
        );

        Ok(Collection {
            fs: self.fs,
            abs_path: params.internal_abs_path,
            edit,
            set_icon_service,
            storage_service,
            vcs: OnceCell::new(),
            worktree: worktree_service,
            on_did_change: EventEmitter::new(),
            archived: config.archived.into(),
        })
    }

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

        // Create expandedEntries key in the database to prevent warnings
        storage_service
            .put_expanded_entries(ctx, Vec::new())
            .await?;

        let worktree_service_inner: Arc<Worktree<R>> =
            Worktree::new(abs_path.clone(), self.fs.clone(), storage_service.clone()).into();

        let set_icon_service =
            SetIconService::new(abs_path.clone(), self.fs.clone(), COLLECTION_ICON_SIZE);

        for dir in &OTHER_DIRS {
            self.fs.create_dir(&abs_path.join(dir)).await?;
        }

        if let Some(icon_path) = params.icon_path {
            if let Err(err) = set_icon_service.set_icon(&icon_path) {
                session::warn!("failed to set collection icon: {}", err.to_string());
            }
        }

        let manifest_vcs_block =
            params
                .git_params
                .as_ref()
                .and_then(|p| match p.repository.normalize_to_string() {
                    Ok(normalized_repository) => match p.git_provider_type {
                        GitProviderKind::GitHub => Some(ManifestVcs::GitHub {
                            repository: normalized_repository,
                        }),
                        GitProviderKind::GitLab => Some(ManifestVcs::GitLab {
                            repository: normalized_repository,
                        }),
                    },
                    Err(e) => {
                        session::error!(format!(
                            "failed to normalize repository url `{:?}`: {}",
                            p.repository,
                            e.to_string()
                        ));
                        None
                    }
                });

        self.fs
            .create_file_with(
                &abs_path.join(MANIFEST_FILE_NAME),
                serde_json::to_string(&ManifestFile {
                    name: params
                        .name
                        .unwrap_or(defaults::DEFAULT_COLLECTION_NAME.to_string()),
                    vcs: manifest_vcs_block,
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
                    archived: false,
                    external_path: params.external_abs_path.map(|p| p.to_path_buf()),
                    account_id: None,
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
                    file.content.as_deref().unwrap_or(&[]),
                    CreateOptions {
                        overwrite: false,
                        ignore_if_exists: false,
                    },
                )
                .await?;
        }

        let edit = CollectionEdit::new(self.fs.clone(), abs_path.join(MANIFEST_FILE_NAME));

        Ok(Collection {
            fs: self.fs,
            abs_path: params.internal_abs_path.to_owned().into(),
            edit,
            set_icon_service,
            storage_service,
            vcs: OnceCell::new(),
            worktree: worktree_service_inner.into(),
            on_did_change: EventEmitter::new(),
            archived: false.into(),
        })
    }

    // TODO: Handle non-collection repo
    pub async fn clone<R: AppRuntime>(
        self,
        ctx: &R::AsyncContext,
        git_client: GitClient<R>,
        params: CollectionCloneParams,
    ) -> joinerror::Result<Collection<R>> {
        debug_assert!(params.internal_abs_path.is_absolute());

        let abs_path = params.internal_abs_path.clone();
        let repository = self
            .do_clone(
                ctx,
                &git_client,
                abs_path.clone(),
                params.repository.to_string_with_suffix()?,
                params.branch,
            )
            .await?;

        let storage_service: Arc<StorageService<R>> = StorageService::new(abs_path.as_ref())
            .join_err::<()>("failed to create collection storage service")?
            .into();

        // Create expandedEntries key in the database to prevent warnings
        storage_service
            .put_expanded_entries(ctx, Vec::new())
            .await?;

        let worktree_inner: Arc<Worktree<R>> =
            Worktree::new(abs_path.clone(), self.fs.clone(), storage_service.clone()).into();

        let set_icon_service =
            SetIconService::new(abs_path.clone(), self.fs.clone(), COLLECTION_ICON_SIZE);

        self.fs
            .create_file_with(
                &abs_path.join(CONFIG_FILE_NAME),
                serde_json::to_string(&ConfigFile {
                    archived: false,
                    external_path: None,
                    account_id: Some(git_client.account_id()),
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
            fs: self.fs,
            abs_path,
            edit,
            set_icon_service,
            storage_service,
            vcs: OnceCell::new_with(Some(Vcs::new(params.repository, repository, git_client))),
            worktree: worktree_inner.into(),
            on_did_change: EventEmitter::new(),
            archived: false.into(),
        })
    }

    pub async fn import_archive<R: AppRuntime>(
        self,
        ctx: &R::AsyncContext,
        params: CollectionImportParams,
    ) -> joinerror::Result<Collection<R>> {
        debug_assert!(params.internal_abs_path.is_absolute());

        let abs_path = params.internal_abs_path;
        let archive_path = params.archive_path;
        self.do_import(abs_path.clone(), archive_path).await?;

        let storage_service: Arc<StorageService<R>> = StorageService::new(abs_path.as_ref())
            .join_err::<()>("failed to create collection storage service")?
            .into();

        // Create expandedEntries key in the database to prevent warnings
        storage_service
            .put_expanded_entries(ctx, Vec::new())
            .await?;

        let worktree_inner: Arc<Worktree<R>> =
            Worktree::new(abs_path.clone(), self.fs.clone(), storage_service.clone()).into();

        let set_icon_service =
            SetIconService::new(abs_path.clone(), self.fs.clone(), COLLECTION_ICON_SIZE);

        self.fs
            .create_file_with(
                &abs_path.join(CONFIG_FILE_NAME),
                serde_json::to_string(&ConfigFile {
                    archived: false,
                    external_path: None,
                    account_id: None,
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
            fs: self.fs,
            abs_path,
            edit,
            set_icon_service,
            storage_service,
            vcs: OnceCell::new(),
            worktree: worktree_inner.into(),
            on_did_change: EventEmitter::new(),
            archived: false.into(),
        })
    }
}

impl CollectionBuilder {
    async fn do_clone<R: AppRuntime>(
        &self,
        ctx: &R::AsyncContext,
        git_client: &GitClient<R>,
        abs_path: Arc<Path>,
        repo_url: String,
        branch: Option<String>,
    ) -> joinerror::Result<Repository> {
        let access_token = git_client.session().access_token(ctx).await?;
        let username = git_client.username();

        let mut cb = git2::RemoteCallbacks::new();
        cb.credentials(move |_url, username_from_url, _allowed| {
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

    async fn do_import(
        &self,
        internal_abs_path: Arc<Path>,
        archive_path: Arc<Path>,
    ) -> joinerror::Result<()> {
        if !archive_path.exists() {
            return Err(Error::new::<ErrorIo>(format!(
                "archive file {} not found",
                archive_path.display()
            ))
            .into());
        }

        self.fs
            .unzip(archive_path.as_ref(), internal_abs_path.as_ref())
            .await?;
        Ok(())
    }
}
