use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;
use std::sync::Arc;

use crate::workspace::service::{WorkspaceItem, WorkspaceService};

pub struct WelcomeWorkspaceOps {
    workspace_service: Arc<WorkspaceService>,
}

impl WelcomeWorkspaceOps {
    pub fn new(workspace_service: Arc<WorkspaceService>) -> Self {
        Self { workspace_service }
    }

    pub async fn list_workspaces<R: AppRuntime>(
        &self,
        delegate: &AppDelegate<R>,
    ) -> joinerror::Result<Vec<WorkspaceItem>> {
        self.workspace_service.known_workspaces(delegate).await
    }
}
