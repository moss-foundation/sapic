use async_trait::async_trait;
use atomic_fs::Rollback;
use joinerror::ResultExt;
use moss_fs::{CreateOptions, FileSystem, RemoveOptions};
use moss_git::repository::Repository;
use sapic_base::{
    other::GitProviderKind,
    project::{
        config::{CONFIG_FILE_NAME, ProjectConfig},
        manifest::{MANIFEST_FILE_NAME, ManifestVcs, ProjectManifest},
    },
};
use sapic_core::context::AnyAsyncContext;
use sapic_system::{
    project::{
        CloneProjectParams, CreateConfigParams, CreateProjectParams, ExportArchiveParams,
        ImportArchivedProjectParams, ImportExternalProjectParams, ProjectBackend,
    },
    user::account::Account,
};
use std::{
    cell::LazyCell,
    path::{Path, PathBuf},
    sync::Arc,
};

mod dirs {
    pub const ASSETS_DIR: &str = "assets";
    pub const ENVIRONMENTS_DIR: &str = "environments";
    pub const RESOURCES_DIR: &str = "resources";
}

const OTHER_DIRS: [&str; 3] = [
    dirs::ASSETS_DIR,
    dirs::ENVIRONMENTS_DIR,
    dirs::RESOURCES_DIR,
];

/// List of files that are always created when a project is created.
/// This list should include only files whose content is fixed and doesn't
/// depend on any parameters or conditions.
///
/// Example:
/// * .gitignore — This file is always created with the exact same content, regardless of context.
/// * config.json — While it's always created, its content depends on the specific parameters of the
/// project being created, so it is **not included** in the list of predefined files.
const PREDEFINED_FILES: LazyCell<Vec<PredefinedFile>> = LazyCell::new(|| {
    vec![
        PredefinedFile {
            path: PathBuf::from(".gitignore"),
            content: Some("config.json\n**/state.db".as_bytes().to_vec()),
        },
        // ---------------------------------------------------------------------------
        // We need to create `.gitkeep` files; otherwise, when committing the project
        // to the repository, that folder won't be included in the commit.
        //
        // This will cause errors when cloning the project, since we expect that folder
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

const ARCHIVE_EXCLUDED_ENTRIES: [&'static str; 6] = [
    "config.json",
    "state.bak",
    "state.sqlite3",
    "state.sqlite3-shm",
    "state.sqlite3-wal",
    ".git",
];

struct PredefinedFile {
    path: PathBuf,
    content: Option<Vec<u8>>,
}

pub struct FsProjectBackend {
    fs: Arc<dyn FileSystem>,
}

impl FsProjectBackend {
    pub fn new(fs: Arc<dyn FileSystem>) -> Arc<Self> {
        Self { fs }.into()
    }

    async fn create_config_file(
        &self,
        ctx: &dyn AnyAsyncContext,
        rb: &mut Rollback,
        params: &CreateConfigParams,
    ) -> joinerror::Result<()> {
        self.fs
            .create_file_with_content_with_rollback(
                ctx,
                rb,
                &params.internal_abs_path.join(CONFIG_FILE_NAME),
                serde_json::to_string(&ProjectConfig {
                    archived: false,
                    external_path: params.external_abs_path.clone().map(|p| p.to_path_buf()),
                    account_id: params.account_id.clone(),
                })?
                .as_bytes(),
                CreateOptions {
                    overwrite: false,
                    ignore_if_exists: false,
                },
            )
            .await?;

        Ok(())
    }

    async fn create_manifest_file(
        &self,
        ctx: &dyn AnyAsyncContext,
        rb: &mut Rollback,
        abs_path: &Path,
        params: &CreateProjectParams,
    ) -> joinerror::Result<()> {
        let vcs = if let Some(git_params) = params.git_params.as_ref() {
            match git_params.repository_url.normalize_to_string() {
                Ok(normalized_repository) => match git_params.provider_kind {
                    GitProviderKind::GitHub => Some(ManifestVcs::GitHub {
                        repository: normalized_repository,
                    }),
                    GitProviderKind::GitLab => Some(ManifestVcs::GitLab {
                        repository: normalized_repository,
                    }),
                },
                Err(e) => {
                    tracing::error!(
                        "failed to normalize repository url `{:?}`: {}",
                        git_params.repository_url,
                        e.to_string()
                    );
                    None
                }
            }
        } else {
            None
        };

        self.fs
            .create_file_with_content_with_rollback(
                ctx,
                rb,
                &abs_path.join(MANIFEST_FILE_NAME),
                serde_json::to_string(&ProjectManifest {
                    name: params.name.clone().unwrap_or("New Project".to_string()),
                    vcs,
                })?
                .as_bytes(),
                CreateOptions {
                    overwrite: false,
                    ignore_if_exists: false,
                },
            )
            .await?;

        Ok(())
    }

    async fn create_project_internal(
        &self,
        ctx: &dyn AnyAsyncContext,
        rb: &mut Rollback,
        abs_path: &Path,
        params: &CreateProjectParams,
    ) -> joinerror::Result<()> {
        self.fs
            .create_dir_with_rollback(ctx, rb, &abs_path)
            .await
            .join_err_with::<()>(|| {
                format!("failed to create directory `{}`", abs_path.display())
            })?;

        for dir in &OTHER_DIRS {
            self.fs
                .create_dir_with_rollback(ctx, rb, &abs_path.join(dir))
                .await
                .join_err::<()>("failed to create directory")?;
        }

        self.create_manifest_file(ctx, rb, &abs_path, &params)
            .await
            .join_err::<()>("failed to create manifest file")?;
        self.create_config_file(
            ctx,
            rb,
            &CreateConfigParams {
                internal_abs_path: params.internal_abs_path.clone(),
                external_abs_path: params.external_abs_path.clone(),
                account_id: None,
            },
        )
        .await
        .join_err::<()>("failed to create config file")?;

        for file in PREDEFINED_FILES.iter() {
            self.fs
                .create_file_with_content_with_rollback(
                    ctx,
                    rb,
                    &abs_path.join(&file.path),
                    file.content.as_deref().unwrap_or(&[]),
                    CreateOptions {
                        overwrite: false,
                        ignore_if_exists: false,
                    },
                )
                .await
                .join_err::<()>("failed to create predefined file")?;
        }

        Ok(())
    }

    async fn clone_project_internal(
        &self,
        ctx: &dyn AnyAsyncContext,
        rb: &mut Rollback,
        account: &Account,
        abs_path: &Path,
        params: &CloneProjectParams,
    ) -> joinerror::Result<Repository> {
        // 1. Create directory
        // 2. Do cloning
        // 3. Setup config
        self.fs
            .create_dir_with_rollback(ctx, rb, &abs_path)
            .await
            .join_err_with::<()>(|| {
                format!("failed to create directory `{}`", abs_path.display())
            })?;

        self.create_config_file(
            ctx,
            rb,
            &CreateConfigParams {
                internal_abs_path: params.internal_abs_path.clone(),
                external_abs_path: None,
                account_id: Some(account.id()),
            },
        )
        .await
        .join_err::<()>("failed to create config file")?;

        let token = account.session().token(ctx).await?;
        let username = account.username();
        let mut cb = git2::RemoteCallbacks::new();
        cb.credentials(move |_url, username_from_url, _allowed| {
            git2::Cred::userpass_plaintext(username_from_url.unwrap_or(&username), &token)
        });

        let repository = Repository::clone(
            &params.git_params.repository_url.to_string_with_suffix()?,
            abs_path,
            cb,
        )
        .join_err::<()>("failed to clone project repository")?;

        if let Some(branch) = &params.git_params.branch_name {
            // Try to check out to the user-selected branch
            // if it fails, we consider the repo creation to also fail
            repository
                .checkout_branch(None, branch, true)
                .join_err_with::<()>(|| format!("failed to checkout branch `{}`", branch))?;
        }

        Ok(repository)
    }

    async fn import_archived_project_internal(
        &self,
        ctx: &dyn AnyAsyncContext,
        rb: &mut Rollback,
        params: &ImportArchivedProjectParams,
    ) -> joinerror::Result<()> {
        self.fs
            .create_dir_with_rollback(ctx, rb, &params.internal_abs_path)
            .await
            .join_err_with::<()>(|| {
                format!(
                    "failed to create directory `{}`",
                    params.internal_abs_path.display()
                )
            })?;

        self.fs
            .unzip(
                ctx,
                params.archive_path.as_ref(),
                params.internal_abs_path.as_ref(),
            )
            .await
            .join_err::<()>("failed to unzip archive")?;

        self.create_config_file(
            ctx,
            rb,
            &CreateConfigParams {
                internal_abs_path: params.internal_abs_path.clone(),
                external_abs_path: None,
                account_id: None,
            },
        )
        .await
        .join_err::<()>("failed to create config file")?;

        Ok(())
    }

    async fn import_external_project_internal(
        &self,
        ctx: &dyn AnyAsyncContext,
        rb: &mut Rollback,
        params: &ImportExternalProjectParams,
    ) -> joinerror::Result<()> {
        self.fs
            .create_dir_with_rollback(ctx, rb, &params.internal_abs_path)
            .await
            .join_err_with::<()>(|| {
                format!(
                    "failed to create directory `{}`",
                    params.internal_abs_path.display()
                )
            })?;

        self.create_config_file(
            ctx,
            rb,
            &CreateConfigParams {
                internal_abs_path: params.internal_abs_path.clone(),
                external_abs_path: Some(params.external_abs_path.clone()),
                account_id: None,
            },
        )
        .await
        .join_err::<()>("failed to create config file")?;

        Ok(())
    }
}

#[async_trait]
impl ProjectBackend for FsProjectBackend {
    async fn read_project_config(
        &self,
        ctx: &dyn AnyAsyncContext,
        abs_path: &Path,
    ) -> joinerror::Result<ProjectConfig> {
        let config_path = abs_path.join(CONFIG_FILE_NAME);
        let rdr = self
            .fs
            .open_file(ctx, &config_path)
            .await
            .join_err::<()>("failed to open config file")?;
        let config: ProjectConfig =
            serde_json::from_reader(rdr).join_err::<()>("failed to parse config file")?;

        Ok(config)
    }

    async fn read_project_manifest(
        &self,
        ctx: &dyn AnyAsyncContext,
        abs_path: &Path,
    ) -> joinerror::Result<ProjectManifest> {
        let manifest_path = abs_path.join(MANIFEST_FILE_NAME);
        let rdr = self
            .fs
            .open_file(ctx, &manifest_path)
            .await
            .join_err::<()>("failed to open manifest file")?;
        let manifest: ProjectManifest =
            serde_json::from_reader(rdr).join_err::<()>("failed to parse manifest file")?;

        Ok(manifest)
    }

    async fn create_project_manifest(
        &self,
        ctx: &dyn AnyAsyncContext,
        abs_path: &Path,
    ) -> joinerror::Result<ProjectManifest> {
        let manifest_path = abs_path.join(MANIFEST_FILE_NAME);

        let rdr = self
            .fs
            .open_file(ctx, &manifest_path)
            .await
            .join_err_with::<()>(|| {
                format!("failed to open manifest file: {}", manifest_path.display())
            })?;

        serde_json::from_reader(rdr).join_err_with::<()>(|| {
            format!("failed to parse manifest file: {}", manifest_path.display())
        })
    }

    async fn create_project(
        &self,
        ctx: &dyn AnyAsyncContext,
        params: CreateProjectParams,
    ) -> joinerror::Result<()> {
        debug_assert!(params.internal_abs_path.is_absolute());

        let abs_path: Arc<Path> = params
            .external_abs_path
            .clone()
            .unwrap_or(params.internal_abs_path.clone())
            .into();

        if abs_path.exists() {
            return Err(joinerror::Error::new::<()>(format!(
                "project directory `{}` already exists",
                abs_path.display()
            )));
        }

        let mut rb = self.fs.start_rollback(ctx).await?;
        if let Err(e) = self
            .create_project_internal(ctx, &mut rb, &abs_path, &params)
            .await
        {
            let _ = rb.rollback().await.map_err(|e| {
                tracing::error!("failed to rollback fs changes: {}", e.to_string());
            });

            Err(e)
        } else {
            Ok(())
        }
    }

    async fn clone_project(
        &self,
        ctx: &dyn AnyAsyncContext,
        account: &Account,
        params: CloneProjectParams,
    ) -> joinerror::Result<Repository> {
        debug_assert!(params.internal_abs_path.is_absolute());

        let abs_path: Arc<Path> = params.internal_abs_path.clone().into();

        if abs_path.exists() {
            return Err(joinerror::Error::new::<()>(format!(
                "project directory `{}` already exists",
                abs_path.display()
            )));
        }

        let mut rb = self.fs.start_rollback(ctx).await?;

        match self
            .clone_project_internal(ctx, &mut rb, account, &abs_path, &params)
            .await
        {
            Ok(repository) => Ok(repository),
            Err(e) => {
                let _ = rb.rollback().await.map_err(|e| {
                    tracing::error!("failed to rollback fs changes: {}", e.to_string());
                });
                Err(e)
            }
        }
    }

    async fn import_archived_project(
        &self,
        ctx: &dyn AnyAsyncContext,
        params: ImportArchivedProjectParams,
    ) -> joinerror::Result<()> {
        debug_assert!(params.internal_abs_path.is_absolute());

        if params.internal_abs_path.exists() {
            return Err(joinerror::Error::new::<()>(format!(
                "project directory `{}` already exists",
                params.internal_abs_path.display()
            )));
        }

        let mut rb = self.fs.start_rollback(ctx).await?;
        if let Err(e) = self
            .import_archived_project_internal(ctx, &mut rb, &params)
            .await
        {
            let _ = rb.rollback().await.map_err(|e| {
                tracing::error!("failed to rollback fs changes: {}", e.to_string());
            });
            return Err(e);
        }

        Ok(())
    }

    async fn import_external_project(
        &self,
        ctx: &dyn AnyAsyncContext,
        params: ImportExternalProjectParams,
    ) -> joinerror::Result<()> {
        debug_assert!(params.internal_abs_path.is_absolute());

        if params.internal_abs_path.exists() {
            return Err(joinerror::Error::new::<()>(format!(
                "project directory `{}` already exists",
                params.internal_abs_path.display()
            )));
        }

        let mut rb = self.fs.start_rollback(ctx).await?;
        if let Err(e) = self
            .import_external_project_internal(ctx, &mut rb, &params)
            .await
        {
            let _ = rb.rollback().await.map_err(|e| {
                tracing::error!("failed to rollback fs changes: {}", e.to_string());
            });
            return Err(e);
        }

        Ok(())
    }

    async fn export_archive(
        &self,
        ctx: &dyn AnyAsyncContext,
        params: ExportArchiveParams,
    ) -> joinerror::Result<()> {
        self.fs
            .zip(
                ctx,
                &params.project_path,
                &params.archive_path,
                &ARCHIVE_EXCLUDED_ENTRIES,
            )
            .await?;

        Ok(())
    }

    async fn delete_project(
        &self,
        ctx: &dyn AnyAsyncContext,
        abs_path: &Path,
    ) -> joinerror::Result<Option<PathBuf>> {
        if abs_path.exists() {
            self.fs
                .remove_dir(
                    ctx,
                    &abs_path,
                    RemoveOptions {
                        recursive: true,
                        ignore_if_not_exists: true,
                    },
                )
                .await
                .join_err::<()>("failed to remove directory")?;
            Ok(Some(abs_path.to_path_buf()))
        } else {
            Ok(None)
        }
    }
}
