use async_trait::async_trait;
use joinerror::ResultExt;
use moss_fs::FileSystem;
use moss_project::{Project, ProjectBuilder, builder::ProjectLoadParams};
use moss_storage2::KvStorage;
use rustc_hash::FxHashMap;
use sapic_base::{
    project::types::primitives::ProjectId, workspace::types::primitives::WorkspaceId,
};
use sapic_core::context::AnyAsyncContext;
use sapic_system::{
    project::project_service::ProjectService,
    workspace::{WorkspaceEditOp, WorkspaceEditParams},
};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::{OnceCell, RwLock};

#[derive(Clone)]
pub struct RuntimeProject {
    pub id: ProjectId,
    pub handle: Arc<Project>,
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

    async fn projects(&self, ctx: &dyn AnyAsyncContext) -> joinerror::Result<Vec<RuntimeProject>>;
}

pub struct RuntimeWorkspace {
    id: WorkspaceId,
    abs_path: PathBuf,
    fs: Arc<dyn FileSystem>,
    storage: Arc<dyn KvStorage>,
    edit: Arc<dyn WorkspaceEditOp>,
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
        project_service: ProjectService,
    ) -> Self {
        Self {
            id,
            abs_path,
            fs,
            storage,
            edit: edit.clone(),
            project_service,
            projects: OnceCell::new(),
        }
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

    async fn projects(&self, ctx: &dyn AnyAsyncContext) -> joinerror::Result<Vec<RuntimeProject>> {
        let projects = self
            .projects
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
                            order: project.order,
                        },
                    );
                }

                Ok::<_, joinerror::Error>(RwLock::new(result))
            })
            .await
            .join_err::<()>("failed to get projects")?;

        Ok(projects.read().await.clone().into_values().collect())
    }
}
