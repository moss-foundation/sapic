use joinerror::{Error, ResultExt};
use moss_fs::{CreateOptions, FileSystem};
use moss_git::{repository::Repository, url::GitUrl};
use moss_logging::session;
use moss_storage2::KvStorage;
use sapic_base::{
    other::GitProviderKind,
    project::{
        config::{CONFIG_FILE_NAME, ProjectConfig},
        manifest::{MANIFEST_FILE_NAME, ManifestVcs, ProjectManifest},
        types::primitives::ProjectId,
    },
    user::types::primitives::AccountId,
};
use sapic_core::{context::AnyAsyncContext, subscription::EventEmitter};
use std::{
    cell::LazyCell,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::OnceCell;

use crate::{
    Project, defaults, dirs, edit::ProjectEdit, errors::ErrorIo, git::GitClient,
    set_icon::SetIconService, vcs::Vcs, worktree::Worktree,
};

const PROJECT_ICON_SIZE: u32 = 128;
const OTHER_DIRS: [&str; 3] = [
    dirs::ASSETS_DIR,
    dirs::ENVIRONMENTS_DIR,
    dirs::RESOURCES_DIR,
];

struct PredefinedFile {
    path: PathBuf,
    content: Option<Vec<u8>>,
}

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

pub struct ProjectCreateParams {
    pub name: Option<String>,
    pub abs_path: PathBuf,
    pub config: ProjectConfig,
    // pub internal_abs_path: Arc<Path>,
    // pub external_abs_path: Option<Arc<Path>>,
    // pub git_params: Option<ProjectCreateGitParams>,
    pub icon_path: Option<PathBuf>,
}

#[derive(Clone)]
pub struct ProjectCreateGitParams {
    pub git_provider_type: GitProviderKind,
    pub repository: GitUrl,
    pub branch: String,
}

pub struct ProjectLoadParams {
    pub internal_abs_path: Arc<Path>,
}

pub struct ProjectCloneParams {
    pub internal_abs_path: Arc<Path>,
    pub account_id: AccountId,
    pub git_provider_type: GitProviderKind,
    pub repository: GitUrl,
    pub branch: Option<String>,
}

pub struct ProjectImportArchiveParams {
    pub internal_abs_path: Arc<Path>,
    pub archive_path: Arc<Path>,
}

pub struct ProjectImportExternalParams {
    pub internal_abs_path: Arc<Path>,
    pub external_abs_path: Arc<Path>,
}

pub struct ProjectBuilder {
    fs: Arc<dyn FileSystem>,
    storage: Arc<dyn KvStorage>,
    project_id: ProjectId,
}

impl ProjectBuilder {
    pub async fn new(
        fs: Arc<dyn FileSystem>,
        storage: Arc<dyn KvStorage>,
        project_id: ProjectId,
    ) -> Self {
        Self {
            fs,
            storage,
            project_id,
        }
    }

    pub async fn load(
        self,
        ctx: &dyn AnyAsyncContext,
        params: ProjectLoadParams,
    ) -> joinerror::Result<Project> {
        debug_assert!(params.internal_abs_path.is_absolute());

        // Check if the project has an external path
        let config: ProjectConfig = {
            let config_path = params.internal_abs_path.join(CONFIG_FILE_NAME);
            let rdr = self
                .fs
                .open_file(ctx, &config_path)
                .await
                .join_err_with::<()>(|| {
                    format!("failed to open config file: {}", config_path.display())
                })?;
            serde_json::from_reader(rdr).join_err_with::<()>(|| {
                format!("failed to parse config file: {}", config_path.display())
            })?
        };
        let abs_path = config
            .external_path
            .clone()
            .map(|p| p.into())
            .unwrap_or(params.internal_abs_path.clone());

        // Verify that the manifest file exists
        // We will handle proper validation later
        if !abs_path.join(MANIFEST_FILE_NAME).exists() {
            return Err(Error::new::<()>("project manifest file `{}` not found"));
        }

        let set_icon_service =
            SetIconService::new(abs_path.clone(), self.fs.clone(), PROJECT_ICON_SIZE);

        let worktree_service = if config.archived {
            OnceCell::new()
        } else {
            Arc::new(Worktree::new(
                self.storage.clone(),
                self.project_id.clone(),
                abs_path.clone(),
                self.fs.clone(),
            ))
            .into()
        };

        let edit = ProjectEdit::new(self.fs.clone(), abs_path.join(MANIFEST_FILE_NAME));

        Ok(Project {
            id: self.project_id,
            fs: self.fs,
            storage: self.storage,
            internal_abs_path: params.internal_abs_path,
            external_abs_path: config.external_path.map(|p| p.into()),
            edit,
            set_icon_service,
            vcs: OnceCell::new(),
            worktree: worktree_service,
            on_did_change: EventEmitter::new(),
            archived: config.archived.into(),
        })
    }

    pub async fn create(
        self,
        ctx: &dyn AnyAsyncContext,
        params: ProjectCreateParams,
    ) -> joinerror::Result<Project> {
        // debug_assert!(params.internal_abs_path.is_absolute());

        let abs_path: Arc<Path> = params
            .config
            .external_path
            .clone()
            .unwrap_or(params.abs_path.clone().into())
            .into();

        let worktree_service_inner: Arc<Worktree> = Worktree::new(
            self.storage.clone(),
            self.project_id.clone(),
            abs_path.clone(),
            self.fs.clone(),
        )
        .into();

        let set_icon_service =
            SetIconService::new(abs_path.clone(), self.fs.clone(), PROJECT_ICON_SIZE);

        // for dir in &OTHER_DIRS {
        //     self.fs.create_dir(ctx, &params.abs_path.join(dir)).await?;
        // }

        if let Some(icon_path) = params.icon_path {
            if let Err(err) = set_icon_service.set_icon(&icon_path) {
                session::warn!("failed to set project icon: {}", err.to_string());
            }
        }

        // let manifest_vcs_block =
        //     params
        //         .git_params
        //         .as_ref()
        //         .and_then(|p| match p.repository.normalize_to_string() {
        //             Ok(normalized_repository) => match p.git_provider_type {
        //                 GitProviderKind::GitHub => Some(ManifestVcs::GitHub {
        //                     repository: normalized_repository,
        //                 }),
        //                 GitProviderKind::GitLab => Some(ManifestVcs::GitLab {
        //                     repository: normalized_repository,
        //                 }),
        //             },
        //             Err(e) => {
        //                 session::error!(format!(
        //                     "failed to normalize repository url `{:?}`: {}",
        //                     p.repository,
        //                     e.to_string()
        //                 ));
        //                 None
        //             }
        //         });

        // self.fs
        //     .create_file_with(
        //         ctx,
        //         &abs_path.join(MANIFEST_FILE_NAME),
        //         serde_json::to_string(&ProjectManifest {
        //             name: params
        //                 .name
        //                 .unwrap_or(defaults::DEFAULT_PROJECT_NAME.to_string()),
        //             vcs: manifest_vcs_block,
        //         })?
        //         .as_bytes(),
        //         CreateOptions {
        //             overwrite: false,
        //             ignore_if_exists: false,
        //         },
        //     )
        //     .await?;

        // self.fs
        //     .create_file_with(
        //         ctx,
        //         &params.internal_abs_path.join(CONFIG_FILE_NAME),
        //         serde_json::to_string(&ProjectConfig {
        //             archived: false,
        //             external_path: params.external_abs_path.clone().map(|p| p.to_path_buf()),
        //             account_id: None,
        //         })?
        //         .as_bytes(),
        //         CreateOptions {
        //             overwrite: false,
        //             ignore_if_exists: false,
        //         },
        //     )
        //     .await?;

        // for file in PREDEFINED_FILES.iter() {
        //     self.fs
        //         .create_file_with(
        //             ctx,
        //             &abs_path.join(&file.path),
        //             file.content.as_deref().unwrap_or(&[]),
        //             CreateOptions {
        //                 overwrite: false,
        //                 ignore_if_exists: false,
        //             },
        //         )
        //         .await?;
        // }

        let edit = ProjectEdit::new(self.fs.clone(), abs_path.join(MANIFEST_FILE_NAME));

        Ok(Project {
            id: self.project_id,
            fs: self.fs,
            storage: self.storage,
            internal_abs_path: abs_path,
            external_abs_path: params.config.external_path.map(|p| p.into()),
            edit,
            set_icon_service,
            vcs: OnceCell::new(),
            worktree: worktree_service_inner.into(),
            on_did_change: EventEmitter::new(),
            archived: false.into(),
        })
    }

    // TODO: Handle non-collection repo
    pub async fn clone(
        self,
        ctx: &dyn AnyAsyncContext,
        git_client: GitClient,
        params: ProjectCloneParams,
    ) -> joinerror::Result<Project> {
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

        let worktree_inner: Arc<Worktree> = Worktree::new(
            self.storage.clone(),
            self.project_id.clone(),
            abs_path.clone(),
            self.fs.clone(),
        )
        .into();

        let set_icon_service =
            SetIconService::new(abs_path.clone(), self.fs.clone(), PROJECT_ICON_SIZE);

        self.fs
            .create_file_with(
                ctx,
                &abs_path.join(CONFIG_FILE_NAME),
                serde_json::to_string(&ProjectConfig {
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

        let edit = ProjectEdit::new(self.fs.clone(), abs_path.join(MANIFEST_FILE_NAME));
        Ok(Project {
            id: self.project_id,
            fs: self.fs,
            storage: self.storage,
            internal_abs_path: abs_path,
            external_abs_path: None,
            edit,
            set_icon_service,
            vcs: OnceCell::new_with(Some(Vcs::new(params.repository, repository, git_client))),
            worktree: worktree_inner.into(),
            on_did_change: EventEmitter::new(),
            archived: false.into(),
        })
    }

    pub async fn import_archive(
        self,
        ctx: &dyn AnyAsyncContext,
        params: ProjectImportArchiveParams,
    ) -> joinerror::Result<Project> {
        debug_assert!(params.internal_abs_path.is_absolute());

        let abs_path = params.internal_abs_path;
        let archive_path = params.archive_path;
        self.do_import(ctx, abs_path.clone(), archive_path).await?;

        let worktree_inner: Arc<Worktree> = Worktree::new(
            self.storage.clone(),
            self.project_id.clone(),
            abs_path.clone(),
            self.fs.clone(),
        )
        .into();

        let set_icon_service =
            SetIconService::new(abs_path.clone(), self.fs.clone(), PROJECT_ICON_SIZE);

        self.fs
            .create_file_with(
                ctx,
                &abs_path.join(CONFIG_FILE_NAME),
                serde_json::to_string(&ProjectConfig {
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

        let edit = ProjectEdit::new(self.fs.clone(), abs_path.join(MANIFEST_FILE_NAME));

        Ok(Project {
            id: self.project_id,
            fs: self.fs,
            storage: self.storage,
            internal_abs_path: abs_path,
            external_abs_path: None,
            edit,
            set_icon_service,
            vcs: OnceCell::new(),
            worktree: worktree_inner.into(),
            on_did_change: EventEmitter::new(),
            archived: false.into(),
        })
    }

    pub async fn import_external(
        self,
        ctx: &dyn AnyAsyncContext,
        params: ProjectImportExternalParams,
    ) -> joinerror::Result<Project> {
        self.fs
            .create_file_with(
                ctx,
                &params.internal_abs_path.join(CONFIG_FILE_NAME),
                serde_json::to_string(&ProjectConfig {
                    archived: false,
                    external_path: Some(params.external_abs_path.clone().to_path_buf()),
                    account_id: None,
                })?
                .as_bytes(),
                CreateOptions {
                    overwrite: false,
                    ignore_if_exists: false,
                },
            )
            .await?;

        self.load(
            ctx,
            ProjectLoadParams {
                internal_abs_path: params.internal_abs_path,
            },
        )
        .await
    }
}

impl ProjectBuilder {
    async fn do_clone(
        &self,
        ctx: &dyn AnyAsyncContext,
        git_client: &GitClient,
        abs_path: Arc<Path>,
        repo_url: String,
        branch: Option<String>,
    ) -> joinerror::Result<Repository> {
        let access_token = git_client.session().token(ctx).await?;
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
        ctx: &dyn AnyAsyncContext,
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
            .unzip(ctx, archive_path.as_ref(), internal_abs_path.as_ref())
            .await?;
        Ok(())
    }
}
