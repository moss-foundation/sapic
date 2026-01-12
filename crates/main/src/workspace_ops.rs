use sapic_core::context::AnyAsyncContext;
use sapic_system::{
    environment::EnvironmentInitializeOp,
    workspace::{CreatedWorkspace, WorkspaceCreateOp},
};
use std::sync::Arc;

#[derive(Clone)]
pub struct MainWindowWorkspaceOps {
    create_workspace: Arc<dyn WorkspaceCreateOp>,
}

impl MainWindowWorkspaceOps {
    pub fn new(create_workspace: Arc<dyn WorkspaceCreateOp>) -> Self {
        Self { create_workspace }
    }

    pub async fn create(
        &self,
        ctx: &dyn AnyAsyncContext,
        name: String,
    ) -> joinerror::Result<CreatedWorkspace> {
        self.create_workspace.create(ctx, name).await
    }
}
