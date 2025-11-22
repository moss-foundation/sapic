use sapic_system::workspace::{WorkspaceCreateOp, workspace_service::CreatedWorkspace};
use std::sync::Arc;

#[derive(Clone)]
pub struct WelcomeWindowWorkspaceOps {
    create_workspace: Arc<dyn WorkspaceCreateOp>,
}

impl WelcomeWindowWorkspaceOps {
    pub fn new(create_workspace: Arc<dyn WorkspaceCreateOp>) -> Self {
        Self { create_workspace }
    }

    pub async fn create(&self, name: String) -> joinerror::Result<CreatedWorkspace> {
        self.create_workspace.create(name).await
    }
}
