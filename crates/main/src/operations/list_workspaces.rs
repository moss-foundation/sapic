use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;
use sapic_window::models::{operations::ListWorkspacesOutput, types::WorkspaceInfo};

use crate::MainWindow;

impl<R: AppRuntime> MainWindow<R> {
    pub async fn list_workspaces(
        &self,
        delegate: &AppDelegate<R>,
    ) -> joinerror::Result<ListWorkspacesOutput> {
        let workspaces = self
            .workspace_ops
            .list_workspaces(delegate)
            .await?
            .into_iter()
            .map(|item| WorkspaceInfo {
                id: item.id,
                name: item.name.clone(),
                last_opened_at: item.last_opened_at,
                abs_path: item.abs_path,
            })
            .collect();

        Ok(ListWorkspacesOutput(workspaces))
    }
}
