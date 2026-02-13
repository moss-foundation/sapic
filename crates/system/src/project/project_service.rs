use joinerror::ResultExt;
use moss_fs::FileSystem;
use moss_git::{repository::Repository, url::GitUrl};
use sapic_base::{
    other::GitProviderKind,
    project::{config::ProjectConfig, manifest::ProjectManifest, types::primitives::ProjectId},
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
        ExportArchiveParams, ImportArchivedProjectParams, ImportExternalProjectParams,
        ProjectServiceFs,
    },
    user::account::Account,
};

pub struct ProjectItem {
    pub id: ProjectId,
    pub internal_abs_path: PathBuf,
    pub manifest: ProjectManifest,
    pub config: ProjectConfig,
}

pub struct ProjectService {
    #[allow(unused)]
    workspace_id: WorkspaceId,
    backend: Arc<dyn ProjectServiceFs>,
    _fs: Arc<dyn FileSystem>,
}

impl ProjectService {
    pub fn new(
        workspace_id: WorkspaceId,
        backend: Arc<dyn ProjectServiceFs>,
        fs: Arc<dyn FileSystem>,
    ) -> Self {
        Self {
            workspace_id,
            backend,
            _fs: fs,
        }
    }

    pub async fn create_project(
        &self,
        ctx: &dyn AnyAsyncContext,
        name: String,
        external_abs_path: Option<PathBuf>,
        git_params: Option<CreateProjectGitParams>,
        icon_path: Option<PathBuf>,
    ) -> joinerror::Result<ProjectItem> {
        let id = ProjectId::new();
        let internal_abs_path = self
            .backend
            .create_project(
                ctx,
                &id,
                CreateProjectParams {
                    name: Some(name),
                    external_abs_path,
                    git_params,
                    icon_path: icon_path.map(|p| p.into()),
                },
            )
            .await
            .join_err::<()>("failed to create project")?;

        let manifest = self.backend.read_project_manifest(ctx, &id).await?;
        let config = self.backend.read_project_config(ctx, &id).await?;

        Ok(ProjectItem {
            id,
            internal_abs_path,
            manifest,
            config,
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
        let (repository, internal_abs_path) = self
            .backend
            .clone_project(
                ctx,
                &id,
                account,
                CloneProjectParams {
                    git_params: CloneProjectGitParams {
                        provider_kind: git_provider_kind,
                        repository_url: repo_url,
                        branch_name: branch,
                    },
                },
            )
            .await?;

        let manifest = self.backend.read_project_manifest(ctx, &id).await?;

        let config = self.backend.read_project_config(ctx, &id).await?;

        let project_item = ProjectItem {
            id,
            internal_abs_path,
            manifest,
            config,
        };

        Ok((project_item, repository))
    }

    pub async fn import_archived_project(
        &self,
        ctx: &dyn AnyAsyncContext,
        archive_path: &Path,
    ) -> joinerror::Result<ProjectItem> {
        let id = ProjectId::new();

        let internal_abs_path = self
            .backend
            .import_archived_project(
                ctx,
                &id,
                ImportArchivedProjectParams {
                    archive_path: archive_path.to_path_buf(),
                },
            )
            .await?;

        let manifest = self.backend.read_project_manifest(ctx, &id).await?;

        let config = self.backend.read_project_config(ctx, &id).await?;

        let project_item = ProjectItem {
            id,
            internal_abs_path,
            manifest,
            config,
        };

        Ok(project_item)
    }

    pub async fn import_external_project(
        &self,
        ctx: &dyn AnyAsyncContext,
        external_abs_path: &Path,
    ) -> joinerror::Result<ProjectItem> {
        let id = ProjectId::new();

        let internal_abs_path = self
            .backend
            .import_external_project(
                ctx,
                &id,
                ImportExternalProjectParams {
                    external_abs_path: external_abs_path.to_path_buf(),
                },
            )
            .await?;

        let manifest = self.backend.read_project_manifest(ctx, &id).await?;

        let config = self.backend.read_project_config(ctx, &id).await?;

        let project_item = ProjectItem {
            id,
            internal_abs_path,
            manifest,
            config,
        };

        Ok(project_item)
    }

    pub async fn delete_project(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &ProjectId,
    ) -> joinerror::Result<Option<PathBuf>> {
        self.backend.delete_project(ctx, id).await
    }

    pub async fn export_archive(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &ProjectId,
        destination: &Path,
    ) -> joinerror::Result<PathBuf> {
        let archive_path = destination.join(format!("{}.zip", id.to_string()));
        self.backend
            .export_archive(
                ctx,
                id,
                ExportArchiveParams {
                    archive_path: archive_path.clone(),
                },
            )
            .await?;

        Ok(archive_path)
    }

    // FIXME: I'm not sure why ProjectItem requires Manifest and Config
    // In WorkspaceService::workspaces we don't need them
    // I'll keep them for now and if needed we can change it
    pub async fn projects(&self, ctx: &dyn AnyAsyncContext) -> joinerror::Result<Vec<ProjectItem>> {
        let discovered_projects = self
            .backend
            .lookup_projects(ctx)
            .await
            .join_err::<()>("failed to lookup projects")?;

        let projects = discovered_projects
            .into_iter()
            .map(|discovered| ProjectItem {
                id: discovered.id.clone(),
                internal_abs_path: discovered.abs_path,
                manifest: discovered.manifest,
                config: discovered.config,
            })
            .collect();

        Ok(projects)
    }
}
