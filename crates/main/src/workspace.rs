use async_trait::async_trait;
use joinerror::{OptionExt, ResultExt};
use moss_fs::FileSystem;
use moss_git::url::GitUrl;
use moss_logging::session;
use moss_project::{
    Project, ProjectBuilder,
    builder::{ProjectCreateParams, ProjectLoadParams},
    git::GitClient,
};
use moss_storage2::{KvStorage, models::primitives::StorageScope};
use moss_workspace::storage::key_project;
use rustc_hash::FxHashMap;
use sapic_base::{
    other::GitProviderKind, project::types::primitives::ProjectId,
    workspace::types::primitives::WorkspaceId,
};
use sapic_core::context::AnyAsyncContext;
use sapic_ipc::contracts::main::project::{CreateProjectParams, UpdateProjectParams};
use sapic_platform::project::project_edit_backend::ProjectFsEditBackend;
use sapic_system::{
    ports::{github_api::GitHubApiClient, gitlab_api::GitLabApiClient},
    project::{
        CreateProjectGitParams, ProjectEditParams, project_edit_service::ProjectEditService,
        project_service::ProjectService,
    },
    user::User,
    workspace::{WorkspaceEditOp, WorkspaceEditParams},
};
use std::{path::PathBuf, sync::Arc};
use tokio::sync::{OnceCell, RwLock};

#[derive(Clone)]
pub struct RuntimeProject {
    pub id: ProjectId,
    pub handle: Arc<Project>,

    pub(crate) edit: ProjectEditService,
    // edit: ProjectEditService
    pub order: Option<isize>,
}

#[async_trait]
pub trait Workspace: Send + Sync {
    fn id(&self) -> WorkspaceId;
    fn abs_path(&self) -> PathBuf;

    async fn dispose(&self) -> joinerror::Result<()>;
    async fn edit(
        &self,
        ctx: &dyn AnyAsyncContext,
        params: WorkspaceEditParams,
    ) -> joinerror::Result<()>;

    async fn create_project(
        &self,
        ctx: &dyn AnyAsyncContext,
        params: CreateProjectParams,
    ) -> joinerror::Result<RuntimeProject>;

    async fn delete_project(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &ProjectId,
    ) -> joinerror::Result<Option<PathBuf>>;

    async fn update_project(
        &self,
        ctx: &dyn AnyAsyncContext,
        params: UpdateProjectParams,
    ) -> joinerror::Result<()>;

    async fn project(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &ProjectId,
    ) -> joinerror::Result<RuntimeProject>;

    async fn projects(&self, ctx: &dyn AnyAsyncContext) -> joinerror::Result<Vec<RuntimeProject>>;
}

pub struct RuntimeWorkspace {
    id: WorkspaceId,
    abs_path: PathBuf,
    fs: Arc<dyn FileSystem>,
    storage: Arc<dyn KvStorage>,
    edit: Arc<dyn WorkspaceEditOp>,

    user: Arc<dyn User>,

    global_github_api: Arc<dyn GitHubApiClient>,
    global_gitlab_api: Arc<dyn GitLabApiClient>,

    project_service: ProjectService,

    // environment_service
    projects: OnceCell<RwLock<FxHashMap<ProjectId, RuntimeProject>>>,
    // environments: FxHashMap<EnvironmentId, Environment>
}

impl RuntimeWorkspace {
    pub fn new(
        id: WorkspaceId,
        abs_path: PathBuf,
        fs: Arc<dyn FileSystem>,
        storage: Arc<dyn KvStorage>,
        edit: Arc<dyn WorkspaceEditOp>,
        user: Arc<dyn User>,
        global_github_api: Arc<dyn GitHubApiClient>,
        global_gitlab_api: Arc<dyn GitLabApiClient>,
        project_service: ProjectService,
    ) -> Self {
        Self {
            id,
            abs_path,
            fs,
            storage,
            edit: edit.clone(),
            user,
            global_github_api,
            global_gitlab_api,
            project_service,
            projects: OnceCell::new(),
        }
    }

    async fn projects_internal(
        &self,
        ctx: &dyn AnyAsyncContext,
    ) -> joinerror::Result<&RwLock<FxHashMap<ProjectId, RuntimeProject>>> {
        self.projects
            .get_or_try_init(|| async {
                let projects = self.project_service.projects(ctx).await?;

                let mut result = FxHashMap::default();
                for project in projects {
                    let builder = ProjectBuilder::new(
                        self.fs.clone(),
                        self.storage.clone(),
                        project.id.clone(),
                    )
                    .await;
                    let handle = builder
                        .load(
                            ctx,
                            ProjectLoadParams {
                                internal_abs_path: project.abs_path.clone().into(),
                            },
                        )
                        .await?;

                    result.insert(
                        project.id.clone(),
                        RuntimeProject {
                            id: project.id.clone(),
                            handle: handle.into(),
                            edit: ProjectEditService::new(ProjectFsEditBackend::new(
                                self.fs.clone(),
                                self.abs_path.join("projects"),
                            )),
                            order: project.order,
                        },
                    );
                }

                Ok::<_, joinerror::Error>(RwLock::new(result))
            })
            .await
            .join_err::<()>("failed to get projects")
    }
}

#[async_trait]
impl Workspace for RuntimeWorkspace {
    fn id(&self) -> WorkspaceId {
        self.id.clone()
    }
    fn abs_path(&self) -> PathBuf {
        self.abs_path.clone()
    }

    async fn edit(
        &self,
        ctx: &dyn AnyAsyncContext,
        params: WorkspaceEditParams,
    ) -> joinerror::Result<()> {
        self.edit.edit(ctx, &self.id, params).await
    }

    async fn dispose(&self) -> joinerror::Result<()> {
        Ok(())
    }

    async fn project(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &ProjectId,
    ) -> joinerror::Result<RuntimeProject> {
        let projects = self.projects_internal(ctx).await?;
        let project = projects
            .read()
            .await
            .get(id)
            .cloned()
            .ok_or_join_err::<()>("project not found")?;

        Ok(project)
    }

    async fn create_project(
        &self,
        ctx: &dyn AnyAsyncContext,
        params: CreateProjectParams,
    ) -> joinerror::Result<RuntimeProject> {
        let git_params = if let Some(p) = params.git_params.clone() {
            let repository_url = GitUrl::parse(&p.repository_url_string())
                .join_err::<()>("failed to parse repository url")?;
            Some(CreateProjectGitParams {
                provider_kind: p.provider_kind(),
                repository_url,
                branch_name: p.branch_name(),
            })
        } else {
            None
        };

        let project_item = self
            .project_service
            .create_project(
                ctx,
                params.name.clone(),
                params.order,
                params.external_path,
                git_params.clone(),
                params.icon_path.clone(),
            )
            .await?;

        let builder = ProjectBuilder::new(
            self.fs.clone(),
            self.storage.clone(),
            project_item.id.clone(),
        )
        .await;

        let handle = builder
            .create(
                ctx,
                ProjectCreateParams {
                    name: Some(params.name),
                    abs_path: project_item.abs_path.clone(),
                    config: project_item.config.clone(),
                    icon_path: params.icon_path,
                },
            )
            .await?;

        let project = RuntimeProject {
            id: project_item.id.clone(),
            handle: handle.into(),
            edit: ProjectEditService::new(ProjectFsEditBackend::new(
                self.fs.clone(),
                self.abs_path.join("projects"),
            )),
            order: Some(params.order),
        };

        let account = if let Some(git_params) = params.git_params.map(|p| p.account_id()) {
            self.user.account(&git_params).await
        } else {
            None
        };

        if let (Some(git_params), Some(account)) = (git_params, account) {
            let client = match git_params.provider_kind {
                GitProviderKind::GitHub => GitClient::GitHub {
                    account: account,
                    api: self.global_github_api.clone(),
                },
                GitProviderKind::GitLab => GitClient::GitLab {
                    account: account,
                    api: self.global_gitlab_api.clone(),
                },
            };

            if let Err(e) = project
                .handle
                .init_vcs(
                    ctx,
                    client,
                    git_params.repository_url,
                    git_params.branch_name,
                )
                .await
            {
                tracing::warn!("failed to init vcs: {}", e.to_string());
                // app_delegate.emit_oneshot(ToLocation::Toast {
                //     activity_id: "create_collection_init_vcs_failure",
                //     title: localize!(NO_TRANSLATE_KEY, "Failed to initialized collection vcs"),
                //     detail: Some(localize!(
                //         NO_TRANSLATE_KEY,
                //         "Failed to initialize collection vcs, creating a local only collection"
                //     )),
                // })?;
            }
        }

        let projects = self.projects_internal(ctx).await?;
        projects
            .write()
            .await
            .insert(project_item.id.clone(), project.clone());

        if let Err(e) = self
            .storage
            .add_project(self.id.inner(), project_item.id.inner())
            .await
        {
            return Err(joinerror::Error::new::<()>(format!(
                "failed to add project storage: {}",
                e
            )));
        }

        Ok(project)
    }

    async fn delete_project(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &ProjectId,
    ) -> joinerror::Result<Option<PathBuf>> {
        let project = self.projects_internal(ctx).await?.write().await.remove(id);
        if project.is_none() {
            return Ok(None);
        }

        // Dropping repo and database handle to prevent lock when deleting the folder
        let project = project.unwrap();
        project.handle.dispose(ctx).await?;

        self.storage
            .remove_project(self.id.inner(), project.id.inner())
            .await?;

        let path = self.project_service.delete_project(ctx, id).await?;

        if let Err(e) = self
            .storage
            .remove_batch_by_prefix(
                ctx,
                StorageScope::Workspace(self.id.inner()),
                &key_project(id),
            )
            .await
        {
            tracing::warn!("failed to remove project `{}` from storage: {}", id, e);
        }

        Ok(path)
    }

    async fn update_project(
        &self,
        ctx: &dyn AnyAsyncContext,
        params: UpdateProjectParams,
    ) -> joinerror::Result<()> {
        let project = self.project(ctx, &params.id).await?;

        project
            .edit
            .edit(
                ctx,
                &params.id,
                ProjectEditParams {
                    name: params.name,
                    repository: params.repository,
                },
            )
            .await?;

        // TODO: Migrate icon update logic or remove it?
        Ok(())
    }

    async fn projects(&self, ctx: &dyn AnyAsyncContext) -> joinerror::Result<Vec<RuntimeProject>> {
        let projects = self.projects_internal(ctx).await?;

        Ok(projects.read().await.clone().into_values().collect())
    }
}
