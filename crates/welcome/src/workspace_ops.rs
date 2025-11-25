use sapic_base::workspace::types::primitives::WorkspaceId;
use sapic_system::workspace::{
    CreatedWorkspace, WorkspaceCreateOp, WorkspaceEditOp, WorkspaceEditParams,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct WelcomeWindowWorkspaceOps {
    create_workspace: Arc<dyn WorkspaceCreateOp>,
    edit_workspace: Arc<dyn WorkspaceEditOp>,
}

impl WelcomeWindowWorkspaceOps {
    pub fn new(
        create_workspace: Arc<dyn WorkspaceCreateOp>,
        edit_workspace: Arc<dyn WorkspaceEditOp>,
    ) -> Self {
        Self {
            create_workspace,
            edit_workspace,
        }
    }

    pub async fn create_workspace(&self, name: String) -> joinerror::Result<CreatedWorkspace> {
        self.create_workspace.create(name).await
    }

    pub async fn update_workspace(
        &self,
        id: &WorkspaceId,
        params: WorkspaceEditParams,
    ) -> joinerror::Result<()> {
        self.edit_workspace.edit(id, params).await
    }
}
