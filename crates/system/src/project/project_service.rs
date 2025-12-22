use joinerror::{Error, ResultExt};
use moss_fs::FileSystem;
use moss_git::{repository::Repository, url::GitUrl};
use moss_storage2::{KvStorage, models::primitives::StorageScope};
use rustc_hash::FxHashMap;
use sapic_base::{
    other::GitProviderKind,
    project::{config::ProjectConfig, manifest::ProjectManifest, types::primitives::ProjectId},
    user::types::primitives::AccountId,
    workspace::types::primitives::WorkspaceId,
};
use sapic_core::context::AnyAsyncContext;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::{
    project::{
        CloneProjectGitParams, CloneProjectParams, CreateProjectGitParams, CreateProjectParams,
        ImportArchivedProjectParams, ImportExternalProjectParams, ProjectBackend,
    },
    user::account::Account,
};

pub static KEY_PROJECT_PREFIX: &'static str = "project";

pub fn key_project_order(id: &ProjectId) -> String {
    format!("{KEY_PROJECT_PREFIX}.{id}.order")
}

pub struct ProjectItem {
    pub id: ProjectId,
    pub abs_path: PathBuf,
    pub manifest: ProjectManifest,
    pub config: ProjectConfig,

    // DEPRECATED: we will get rid of this field in the future
    pub order: Option<isize>,
}

pub struct ProjectService {
    workspace_id: WorkspaceId,
    backend: Arc<dyn ProjectBackend>,
    abs_path: PathBuf,
    fs: Arc<dyn FileSystem>,
    storage: Arc<dyn KvStorage>,
}

impl ProjectService {
    pub fn new(
        workspace_id: WorkspaceId,
        backend: Arc<dyn ProjectBackend>,
        abs_path: PathBuf,
        fs: Arc<dyn FileSystem>,
        storage: Arc<dyn KvStorage>,
    ) -> Self {
        Self {
            workspace_id,
            backend,
            abs_path,
            fs,
            storage,
        }
    }

    pub async fn create_project(
        &self,
        ctx: &dyn AnyAsyncContext,
        name: String,
        order: isize,
        external_path: Option<PathBuf>,
        git_params: Option<CreateProjectGitParams>,
        icon_path: Option<PathBuf>,
    ) -> joinerror::Result<ProjectItem> {
        let id = ProjectId::new();
        let internal_abs_path = self.abs_path.join(id.to_string());
        let external_abs_path = external_path.map(|p| p);
        self.backend
            .create_project(
                ctx,
                CreateProjectParams {
                    name: Some(name),
                    internal_abs_path: internal_abs_path.clone(),
                    external_abs_path,
                    git_params,
                    icon_path: icon_path.map(|p| p.into()),
                },
            )
            .await
            .join_err::<()>("failed to create project")?;

        let manifest = self
            .backend
            .create_project_manifest(ctx, &internal_abs_path)
            .await?;
        let config = self
            .backend
            .read_project_config(ctx, &internal_abs_path)
            .await?;

        Ok(ProjectItem {
            id,
            abs_path: internal_abs_path,
            manifest,
            config,
            order: Some(order),
        })
    }

    // FIXME: I think repo cloning is at the same level as fs operations, handled by platform backends
    // However, we also need to Repository handle when building the Project
    // Not sure if there's a better way than passing the repository from here

    pub async fn clone_project(
        &self,
        ctx: &dyn AnyAsyncContext,
        account: &Account,
        git_provider_kind: GitProviderKind,
        repo_url: GitUrl,
        branch: Option<String>,
    ) -> joinerror::Result<(ProjectItem, Repository)> {
        let id = ProjectId::new();
        let abs_path = self.abs_path.join(id.to_string());
        let repository = self
            .backend
            .clone_project(
                ctx,
                account,
                CloneProjectParams {
                    internal_abs_path: abs_path.clone(),
                    git_params: CloneProjectGitParams {
                        provider_kind: git_provider_kind,
                        repository_url: repo_url,
                        branch_name: branch,
                    },
                },
            )
            .await?;

        let manifest = self.backend.read_project_manifest(ctx, &abs_path).await?;

        let config = self.backend.read_project_config(ctx, &abs_path).await?;

        let project_item = ProjectItem {
            id,
            abs_path,
            manifest,
            config,
            order: None,
        };

        Ok((project_item, repository))
    }

    pub async fn import_archived_project(
        &self,
        ctx: &dyn AnyAsyncContext,
        archive_path: &Path,
    ) -> joinerror::Result<ProjectItem> {
        let id = ProjectId::new();

        let abs_path = self.abs_path.join(id.to_string());

        self.backend
            .import_archived_project(
                ctx,
                ImportArchivedProjectParams {
                    internal_abs_path: abs_path.to_path_buf(),
                    archive_path: archive_path.to_path_buf(),
                },
            )
            .await?;

        let manifest = self.backend.read_project_manifest(ctx, &abs_path).await?;

        let config = self.backend.read_project_config(ctx, &abs_path).await?;

        let project_item = ProjectItem {
            id,
            abs_path,
            manifest,
            config,
            order: None,
        };

        Ok(project_item)
    }

    pub async fn import_external_project(
        &self,
        ctx: &dyn AnyAsyncContext,
        external_abs_path: &Path,
    ) -> joinerror::Result<ProjectItem> {
        let id = ProjectId::new();

        let internal_abs_path = self.abs_path.join(id.to_string());

        self.backend
            .import_external_project(
                ctx,
                ImportExternalProjectParams {
                    internal_abs_path: internal_abs_path.clone(),
                    external_abs_path: external_abs_path.to_path_buf(),
                },
            )
            .await?;

        let manifest = self
            .backend
            .read_project_manifest(ctx, &internal_abs_path)
            .await?;

        let config = self
            .backend
            .read_project_config(ctx, &internal_abs_path)
            .await?;

        let project_item = ProjectItem {
            id,
            // FIXME: Should this be internal abs path?
            abs_path: internal_abs_path,
            manifest,
            config,
            order: None,
        };

        Ok(project_item)
    }

    pub async fn delete_project(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &ProjectId,
    ) -> joinerror::Result<Option<PathBuf>> {
        let abs_path = self.abs_path.join(id.to_string());
        if abs_path.exists() {
            self.backend.delete_project(ctx, &abs_path).await?;
            Ok(Some(abs_path))
        } else {
            Ok(None)
        }
    }

    pub async fn archive_project(&self, id: &ProjectId) -> joinerror::Result<()> {
        todo!()
    }

    pub async fn unarchive_project(&self, id: &ProjectId) -> joinerror::Result<()> {
        todo!()
    }

    pub async fn import_project(&self) -> joinerror::Result<()> {
        todo!()
    }

    pub async fn export_project(&self, id: &ProjectId) -> joinerror::Result<()> {
        todo!()
    }

    pub async fn projects(&self, ctx: &dyn AnyAsyncContext) -> joinerror::Result<Vec<ProjectItem>> {
        let mut projects = Vec::new();

        let metadata = self
            .storage
            .get_batch_by_prefix(
                ctx,
                StorageScope::Workspace(self.workspace_id.inner()),
                KEY_PROJECT_PREFIX,
            )
            .await
            .unwrap_or_else(|e| {
                tracing::warn!(
                    "failed to fetch metadata from database when listing projects: {}",
                    e
                );
                Vec::new()
            })
            .into_iter()
            .collect::<FxHashMap<_, _>>();

        let mut read_dir = self
            .fs
            .read_dir(ctx, &self.abs_path)
            .await
            .join_err_with::<()>(|| {
                format!("failed to read directory `{}`", self.abs_path.display())
            })?;

        while let Some(entry) = read_dir.next_entry().await? {
            if !entry.file_type().await?.is_dir() {
                continue;
            }

            let id_str = entry.file_name().to_string_lossy().to_string();
            let id: ProjectId = id_str.clone().into();

            let manifest = self
                .backend
                .create_project_manifest(ctx, &entry.path())
                .await
                .join_err::<()>("failed to read manifest")?;
            let config = self
                .backend
                .read_project_config(ctx, &entry.path())
                .await
                .join_err::<()>("failed to read project config")?;

            projects.push(ProjectItem {
                id: id.clone(),
                abs_path: entry.path().to_owned(),
                manifest,
                config,
                order: metadata
                    .get(&key_project_order(&id))
                    .and_then(|v| serde_json::from_value(v.clone()).ok()),
            });
        }

        Ok(projects)
    }
}
