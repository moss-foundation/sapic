use async_trait::async_trait;
use sapic_base::workspace::types::primitives::WorkspaceId;
use sapic_system::workspace::{WorkspaceEditOp, WorkspaceEditParams};
use std::{path::Path, sync::Arc};

#[async_trait]
pub trait Workspace: Send + Sync {
    fn id(&self) -> WorkspaceId;
    fn abs_path(&self) -> Arc<Path>;

    async fn dispose(&self) -> joinerror::Result<()>;
    async fn edit(&self, params: WorkspaceEditParams) -> joinerror::Result<()>;
}

pub struct RuntimeWorkspace {
    id: WorkspaceId,
    abs_path: Arc<Path>,
    edit: Arc<dyn WorkspaceEditOp>,
    // project_service
    // environment_service

    // projects: FxHashMap<ProjectId, Project>
    // environments: FxHashMap<EnvironmentId, Environment>
}

impl RuntimeWorkspace {
    pub fn new(id: WorkspaceId, abs_path: Arc<Path>, edit: Arc<dyn WorkspaceEditOp>) -> Self {
        Self { id, abs_path, edit }
    }
}

#[async_trait]
impl Workspace for RuntimeWorkspace {
    fn id(&self) -> WorkspaceId {
        self.id.clone()
    }
    fn abs_path(&self) -> Arc<Path> {
        self.abs_path.clone()
    }

    async fn edit(&self, params: WorkspaceEditParams) -> joinerror::Result<()> {
        self.edit.edit(&self.id, params).await
    }

    async fn dispose(&self) -> joinerror::Result<()> {
        Ok(())
    }
}
