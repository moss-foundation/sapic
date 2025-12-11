use sapic_base::workspace::types::primitives::WorkspaceId;
use sapic_system::workspace::{
    CreatedWorkspace, WorkspaceCreateOp, WorkspaceEditOp, WorkspaceEditParams,
};
use std::sync::Arc;

#[cfg(feature = "integration-tests")]
use sapic_system::workspace::{WorkspaceListOp, types::WorkspaceItem};

#[derive(Clone)]
pub struct WelcomeWindowWorkspaceOps {
    create_workspace: Arc<dyn WorkspaceCreateOp>,
    edit_workspace: Arc<dyn WorkspaceEditOp>,

    // In tests we need to verify the results of create and edit workspace
    // But we can't create a full app controller, which would result in circular dependency
    #[cfg(feature = "integration-tests")]
    list_workspaces: Arc<dyn WorkspaceListOp>,
}

impl WelcomeWindowWorkspaceOps {
    #[cfg(not(feature = "integration-tests"))]
    pub fn new(
        create_workspace: Arc<dyn WorkspaceCreateOp>,
        edit_workspace: Arc<dyn WorkspaceEditOp>,
    ) -> Self {
        Self {
            create_workspace,
            edit_workspace,
        }
    }

    #[cfg(feature = "integration-tests")]
    pub fn new(
        create_workspace: Arc<dyn WorkspaceCreateOp>,
        edit_workspace: Arc<dyn WorkspaceEditOp>,
        list_workspaces: Arc<dyn WorkspaceListOp>,
    ) -> Self {
        Self {
            create_workspace,
            edit_workspace,
            list_workspaces,
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

    #[cfg(feature = "integration-tests")]
    pub async fn list_workspaces(&self) -> joinerror::Result<Vec<WorkspaceItem>> {
        self.list_workspaces.list().await
    }
}
