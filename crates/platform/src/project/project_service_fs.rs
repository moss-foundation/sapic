use async_trait::async_trait;
use atomic_fs::Rollback;
use joinerror::{ResultExt, bail};
use moss_common::continue_if_err;
use moss_fs::{CreateOptions, FileSystem, RemoveOptions};
use moss_git::repository::Repository;
use sapic_base::{
    other::GitProviderKind,
    project::{
        config::{CONFIG_FILE_NAME, ProjectConfig},
        manifest::{MANIFEST_FILE_NAME, ManifestVcs, ProjectManifest},
        types::primitives::ProjectId,
    },
};
use sapic_core::context::AnyAsyncContext;
use sapic_system::{
    project::{
        CloneProjectParams, CreateConfigParams, CreateProjectParams, ExportArchiveParams,
        ImportArchivedProjectParams, ImportExternalProjectParams, LookedUpProject,
        ProjectServiceFs as ProjectServiceFsPort,
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

pub struct ProjectServiceFs {
    projects_dir: PathBuf,
    fs: Arc<dyn FileSystem>,
}

impl ProjectServiceFs {
    pub fn new(fs: Arc<dyn FileSystem>, projects_dir: PathBuf) -> Arc<Self> {
        Self { fs, projects_dir }.into()
    }

    async fn create_config_file(
        &self,
        ctx: &dyn AnyAsyncContext,
        rb: &mut Rollback,
        id: &ProjectId,
        params: &CreateConfigParams,
    ) -> joinerror::Result<()> {
        let internal_abs_path = self.projects_dir.join(id.to_string());
        self.fs
            .create_file_with_content_with_rollback(
                ctx,
                rb,
                &internal_abs_path.join(CONFIG_FILE_NAME),
                serde_json::to_string(&ProjectConfig {
                    archived: false,
                    external_path: params.external_abs_path.clone().map(|p| p.to_path_buf()),
                    account_id: params.account_id.clone(),
                    repository: params.repository.clone(),
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
        id: &ProjectId,
        params: &CreateProjectParams,
    ) -> joinerror::Result<()> {
        let internal_abs_path = self.projects_dir.join(id.to_string());
        let abs_path = params
            .external_abs_path
            .clone()
            .unwrap_or(internal_abs_path.to_path_buf());

        self.fs
            .create_dir_with_rollback(ctx, rb, &abs_path)
            .await
            .join_err_with::<()>(|| {
                format!("failed to create directory `{}`", abs_path.display())
            })?;

        // Make sure that the internal folder is created for external projects
        if internal_abs_path != abs_path {
            self.fs
                .create_dir_with_rollback(ctx, rb, &internal_abs_path)
                .await
                .join_err_with::<()>(|| {
                    format!("failed to create directory `{}`", abs_path.display())
                })?
        }

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
            &id,
            &CreateConfigParams {
                external_abs_path: params.external_abs_path.clone(),
                account_id: None,
                repository: None,
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
        id: &ProjectId,
        params: &CloneProjectParams,
    ) -> joinerror::Result<Repository> {
        let internal_abs_path = self.projects_dir.join(id.to_string());

        // 1. Create directory
        // 2. Do cloning
        // 3. Setup config
        self.fs
            .create_dir_with_rollback(ctx, rb, &internal_abs_path)
            .await
            .join_err_with::<()>(|| {
                format!(
                    "failed to create directory `{}`",
                    internal_abs_path.display()
                )
            })?;

        let token = account.session().token(ctx).await?;
        let username = account.username();

        let repository = {
            let mut cb = git2::RemoteCallbacks::new();
            cb.credentials(move |_url, username_from_url, _allowed| {
                git2::Cred::userpass_plaintext(username_from_url.unwrap_or(&username), &token)
            });
            let repository = Repository::clone(
                &params.git_params.repository_url.to_string_with_suffix()?,
                &internal_abs_path,
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
            repository
        };

        self.create_config_file(
            ctx,
            rb,
            &id,
            &CreateConfigParams {
                external_abs_path: None,
                account_id: Some(account.id()),
                repository: Some(params.git_params.repository_url.to_string()?),
            },
        )
        .await
        .join_err::<()>("failed to create config file")?;

        Ok(repository)
    }

    async fn import_archived_project_internal(
        &self,
        ctx: &dyn AnyAsyncContext,
        rb: &mut Rollback,
        id: &ProjectId,
        params: &ImportArchivedProjectParams,
    ) -> joinerror::Result<()> {
        let internal_abs_path = self.projects_dir.join(id.to_string());
        self.fs
            .create_dir_with_rollback(ctx, rb, &internal_abs_path)
            .await
            .join_err_with::<()>(|| {
                format!(
                    "failed to create directory `{}`",
                    internal_abs_path.display()
                )
            })?;

        self.fs
            .unzip(
                ctx,
                params.archive_path.as_ref(),
                internal_abs_path.as_ref(),
            )
            .await
            .join_err::<()>("failed to unzip archive")?;

        self.create_config_file(
            ctx,
            rb,
            id,
            &CreateConfigParams {
                external_abs_path: None,
                account_id: None,
                repository: None,
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
        id: &ProjectId,
        params: &ImportExternalProjectParams,
    ) -> joinerror::Result<()> {
        let internal_abs_path = self.projects_dir.join(id.to_string());
        self.fs
            .create_dir_with_rollback(ctx, rb, &internal_abs_path)
            .await
            .join_err_with::<()>(|| {
                format!(
                    "failed to create directory `{}`",
                    internal_abs_path.display()
                )
            })?;

        self.create_config_file(
            ctx,
            rb,
            id,
            &CreateConfigParams {
                external_abs_path: Some(params.external_abs_path.clone()),
                account_id: None,
                repository: None,
            },
        )
        .await
        .join_err::<()>("failed to create config file")?;

        Ok(())
    }
}

#[async_trait]
impl ProjectServiceFsPort for ProjectServiceFs {
    async fn lookup_projects(
        &self,
        ctx: &dyn AnyAsyncContext,
    ) -> joinerror::Result<Vec<LookedUpProject>> {
        let mut read_dir = self.fs.read_dir(ctx, &self.projects_dir).await?;
        let mut projects = vec![];

        while let Some(entry) = read_dir.next_entry().await? {
            if !entry.file_type().await?.is_dir() {
                continue;
            }

            let id_str = entry.file_name().to_string_lossy().to_string();
            let id: ProjectId = id_str.into();

            let manifest = match self.read_project_manifest(ctx, &id).await {
                Ok(manifest) => manifest,
                Err(e) => {
                    tracing::warn!("failed to parse manifest file: {}", e);
                    continue;
                }
            };

            let config = match self.read_project_config(ctx, &id).await {
                Ok(manifest) => manifest,
                Err(e) => {
                    tracing::warn!("failed to parse manifest file: {}", e);
                    continue;
                }
            };

            projects.push(LookedUpProject {
                id,
                abs_path: entry.path(),
                manifest,
                config,
            });
        }

        Ok(projects)
    }

    async fn read_project_config(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &ProjectId,
    ) -> joinerror::Result<ProjectConfig> {
        let config_path = self
            .projects_dir
            .join(id.to_string())
            .join(CONFIG_FILE_NAME);
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
        id: &ProjectId,
    ) -> joinerror::Result<ProjectManifest> {
        let manifest_path = self
            .projects_dir
            .join(id.to_string())
            .join(MANIFEST_FILE_NAME);
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
        id: &ProjectId,
    ) -> joinerror::Result<ProjectManifest> {
        let manifest_path = self
            .projects_dir
            .join(id.to_string())
            .join(MANIFEST_FILE_NAME);

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
        id: &ProjectId,
        params: CreateProjectParams,
    ) -> joinerror::Result<PathBuf> {
        let internal_abs_path = self.projects_dir.join(id.to_string());

        if internal_abs_path.exists() {
            return Err(joinerror::Error::new::<()>(format!(
                "project directory `{}` already exists",
                internal_abs_path.display()
            )));
        }

        let mut rb = self.fs.start_rollback(ctx).await?;
        if let Err(e) = self
            .create_project_internal(ctx, &mut rb, id, &params)
            .await
        {
            let _ = rb.rollback().await.map_err(|e| {
                tracing::error!("failed to rollback fs changes: {}", e.to_string());
            });

            Err(e)
        } else {
            Ok(internal_abs_path)
        }
    }

    async fn clone_project(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &ProjectId,
        account: &Account,
        params: CloneProjectParams,
    ) -> joinerror::Result<(Repository, PathBuf)> {
        let internal_abs_path = self.projects_dir.join(id.to_string());

        if internal_abs_path.exists() {
            return Err(joinerror::Error::new::<()>(format!(
                "project directory `{}` already exists",
                internal_abs_path.display()
            )));
        }

        let mut rb = self.fs.start_rollback(ctx).await?;

        match self
            .clone_project_internal(ctx, &mut rb, account, id, &params)
            .await
        {
            Ok(repository) => Ok((repository, internal_abs_path)),
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
        id: &ProjectId,
        params: ImportArchivedProjectParams,
    ) -> joinerror::Result<PathBuf> {
        let internal_abs_path = self.projects_dir.join(id.to_string());

        if internal_abs_path.exists() {
            return Err(joinerror::Error::new::<()>(format!(
                "project directory `{}` already exists",
                internal_abs_path.display()
            )));
        }

        let mut rb = self.fs.start_rollback(ctx).await?;
        if let Err(e) = self
            .import_archived_project_internal(ctx, &mut rb, id, &params)
            .await
        {
            let _ = rb.rollback().await.map_err(|e| {
                tracing::error!("failed to rollback fs changes: {}", e.to_string());
            });
            return Err(e);
        }

        Ok(internal_abs_path)
    }

    async fn import_external_project(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &ProjectId,
        params: ImportExternalProjectParams,
    ) -> joinerror::Result<PathBuf> {
        let internal_abs_path = self.projects_dir.join(id.to_string());

        if internal_abs_path.exists() {
            return Err(joinerror::Error::new::<()>(format!(
                "project directory `{}` already exists",
                internal_abs_path.display()
            )));
        }

        let mut rb = self.fs.start_rollback(ctx).await?;
        if let Err(e) = self
            .import_external_project_internal(ctx, &mut rb, id, &params)
            .await
        {
            let _ = rb.rollback().await.map_err(|e| {
                tracing::error!("failed to rollback fs changes: {}", e.to_string());
            });
            return Err(e);
        }

        Ok(internal_abs_path)
    }

    async fn export_archive(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &ProjectId,
        params: ExportArchiveParams,
    ) -> joinerror::Result<()> {
        let internal_abs_path = self.projects_dir.join(id.to_string());
        if params.archive_path.starts_with(&internal_abs_path) {
            bail!("cannot export archive file into the project folder");
        }

        self.fs
            .zip(
                ctx,
                &internal_abs_path,
                &params.archive_path,
                &ARCHIVE_EXCLUDED_ENTRIES,
            )
            .await?;

        Ok(())
    }

    async fn delete_project(
        &self,
        ctx: &dyn AnyAsyncContext,
        project_id: &ProjectId,
    ) -> joinerror::Result<Option<PathBuf>> {
        let internal_abs_path = self.projects_dir.join(project_id.to_string());
        if internal_abs_path.exists() {
            self.fs
                .remove_dir(
                    ctx,
                    &internal_abs_path,
                    RemoveOptions {
                        recursive: true,
                        ignore_if_not_exists: true,
                    },
                )
                .await
                .join_err::<()>("failed to remove directory")?;
            Ok(Some(internal_abs_path.to_path_buf()))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use moss_fs::RealFileSystem;
    use moss_testutils::random_name::random_string;
    use sapic_base::project::types::primitives::ProjectId;
    use sapic_core::context::ArcContext;

    async fn set_test_project_service_fs() -> (ArcContext, Arc<ProjectServiceFs>, PathBuf) {
        let ctx = ArcContext::background();
        let test_path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
            .join("tests")
            .join("data")
            .join(random_string(10));

        let tmp_path = test_path.with_extension("tmp");
        let projects_dir = test_path.join("projects");

        tokio::fs::create_dir_all(&tmp_path).await.unwrap();
        tokio::fs::create_dir_all(&projects_dir).await.unwrap();

        let fs = Arc::new(RealFileSystem::new(&tmp_path));
        let project_fs = ProjectServiceFs::new(fs.clone(), projects_dir);

        (ctx, project_fs, test_path)
    }

    #[tokio::test]
    async fn test_create_project_normal() {
        let (ctx, service_fs, test_path) = set_test_project_service_fs().await;
        let id = ProjectId::new();
        let internal_abs_path = test_path.join("projects").join(id.to_string());

        service_fs
            .create_project(
                &ctx,
                &id,
                CreateProjectParams {
                    name: Some(random_string(10)),
                    external_abs_path: None,
                    git_params: None,
                    icon_path: None,
                },
            )
            .await
            .unwrap();

        assert!(internal_abs_path.exists());
        for dir in OTHER_DIRS {
            assert!(internal_abs_path.join(dir).exists());
        }

        for file in PREDEFINED_FILES.iter() {
            let path = internal_abs_path.join(&file.path);
            assert!(path.exists());
        }

        tokio::fs::remove_dir_all(&internal_abs_path).await.unwrap();
    }

    #[tokio::test]
    async fn test_create_project_no_name() {
        let (ctx, service_fs, test_path) = set_test_project_service_fs().await;
        let id = ProjectId::new();
        let internal_abs_path = test_path.join("projects").join(id.to_string());

        service_fs
            .create_project(
                &ctx,
                &id,
                CreateProjectParams {
                    name: None,
                    external_abs_path: None,
                    git_params: None,
                    icon_path: None,
                },
            )
            .await
            .unwrap();

        assert!(internal_abs_path.exists());
        for dir in OTHER_DIRS {
            assert!(internal_abs_path.join(dir).exists());
        }

        for file in PREDEFINED_FILES.iter() {
            let path = internal_abs_path.join(&file.path);
            assert!(path.exists());
        }

        tokio::fs::remove_dir_all(&test_path).await.unwrap();
    }

    #[tokio::test]
    async fn test_create_project_already_exists() {
        let (ctx, service_fs, test_path) = set_test_project_service_fs().await;
        let id = ProjectId::new();
        let internal_abs_path = test_path.join("projects").join(id.to_string());

        service_fs
            .create_project(
                &ctx,
                &id,
                CreateProjectParams {
                    name: None,
                    external_abs_path: None,
                    git_params: None,
                    icon_path: None,
                },
            )
            .await
            .unwrap();

        assert!(
            service_fs
                .create_project(
                    &ctx,
                    &id,
                    CreateProjectParams {
                        name: None,
                        external_abs_path: None,
                        git_params: None,
                        icon_path: None,
                    }
                )
                .await
                .is_err()
        );

        tokio::fs::remove_dir_all(&test_path).await.unwrap();
    }

    #[tokio::test]
    async fn test_create_project_external() {
        let (ctx, service_fs, test_path) = set_test_project_service_fs().await;
        let id = ProjectId::new();
        let internal_abs_path = test_path.join("projects").join(id.to_string());

        let external_abs_path = test_path.join("external");

        service_fs
            .create_project(
                &ctx,
                &id,
                CreateProjectParams {
                    name: Some(random_string(10)),
                    external_abs_path: Some(external_abs_path.clone()),
                    git_params: None,
                    icon_path: None,
                },
            )
            .await
            .unwrap();

        assert!(internal_abs_path.exists());
        assert!(internal_abs_path.join("config.json").exists());

        assert!(external_abs_path.join("Sapic.json").exists());
        for dir in OTHER_DIRS {
            assert!(external_abs_path.join(dir).exists());
        }

        for file in PREDEFINED_FILES.iter() {
            let path = external_abs_path.join(&file.path);
            assert!(path.exists());
        }
        tokio::fs::remove_dir_all(&test_path).await.unwrap();
    }

    #[tokio::test]
    async fn test_delete_project_success() {
        let (ctx, service_fs, test_path) = set_test_project_service_fs().await;
        let id = ProjectId::new();
        let internal_abs_path = test_path.join("projects").join(id.to_string());

        service_fs
            .create_project(
                &ctx,
                &id,
                CreateProjectParams {
                    name: None,
                    external_abs_path: None,
                    git_params: None,
                    icon_path: None,
                },
            )
            .await
            .unwrap();

        service_fs.delete_project(&ctx, &id).await.unwrap();

        assert!(!internal_abs_path.exists());
        tokio::fs::remove_dir_all(&test_path).await.unwrap();
    }

    // Deleting a nonexistent project should be handled gracefully
    #[tokio::test]
    async fn test_delete_project_nonexistent() {
        let (ctx, service_fs, test_path) = set_test_project_service_fs().await;
        let id = ProjectId::new();
        let internal_abs_path = test_path.join("projects").join(id.to_string());

        let result = service_fs.delete_project(&ctx, &id).await.unwrap();

        assert!(result.is_none());
        tokio::fs::remove_dir_all(&test_path).await.unwrap();
    }

    // Skip clone_project in unit tests since it requires git account

    #[tokio::test]
    async fn test_export_and_import_archive() {
        let (ctx, service_fs, test_path) = set_test_project_service_fs().await;
        let id = ProjectId::new();
        let internal_abs_path = test_path.join("projects").join(id.to_string());

        service_fs
            .create_project(
                &ctx,
                &id,
                CreateProjectParams {
                    name: None,
                    external_abs_path: None,
                    git_params: None,
                    icon_path: None,
                },
            )
            .await
            .unwrap();

        let archive_path = test_path.join("archive.zip");

        service_fs
            .export_archive(
                &ctx,
                &id,
                ExportArchiveParams {
                    archive_path: archive_path.clone(),
                },
            )
            .await
            .unwrap();

        assert!(archive_path.exists());

        let new_id = ProjectId::new();
        let imported_internal_abs_path = test_path.join("projects").join(new_id.to_string());
        service_fs
            .import_archived_project(&ctx, &id, ImportArchivedProjectParams { archive_path })
            .await
            .unwrap();

        assert!(imported_internal_abs_path.exists());

        for dir in OTHER_DIRS {
            assert!(imported_internal_abs_path.join(dir).exists());
        }

        for file in PREDEFINED_FILES.iter() {
            let path = imported_internal_abs_path.join(&file.path);
            assert!(path.exists());
        }
        tokio::fs::remove_dir_all(&test_path).await.unwrap();
    }

    #[tokio::test]
    async fn test_import_project_external() {
        let (ctx, service_fs, test_path) = set_test_project_service_fs().await;
        let id = ProjectId::new();
        let old_internal_abs_path = test_path.join("projects").join(id.to_string());

        let external_abs_path = test_path.join("external");

        service_fs
            .create_project(
                &ctx,
                &id,
                CreateProjectParams {
                    name: Some(random_string(10)),
                    external_abs_path: Some(external_abs_path.clone()),
                    git_params: None,
                    icon_path: None,
                },
            )
            .await
            .unwrap();

        let new_id = ProjectId::new();
        let new_internal_abs_path = test_path.join("projects").join(new_id.to_string());

        service_fs
            .import_external_project(&ctx, &id, ImportExternalProjectParams { external_abs_path })
            .await
            .unwrap();

        assert!(new_internal_abs_path.exists());
        assert!(new_internal_abs_path.join("config.json").exists());
        tokio::fs::remove_dir_all(&test_path).await.unwrap();
    }
}
