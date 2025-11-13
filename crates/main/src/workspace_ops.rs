use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;
use sapic_system::workspace::workspace_service::{WorkspaceItem, WorkspaceService};
use std::sync::Arc;

pub struct MainWorkspaceOps {
    workspace_service: Arc<WorkspaceService>,
}

impl MainWorkspaceOps {
    pub fn new(workspace_service: Arc<WorkspaceService>) -> Self {
        Self { workspace_service }
    }

    pub async fn list_workspaces<R: AppRuntime>(
        &self,
        _delegate: &AppDelegate<R>,
    ) -> joinerror::Result<Vec<WorkspaceItem>> {
        self.workspace_service.known_workspaces().await
    }
}
