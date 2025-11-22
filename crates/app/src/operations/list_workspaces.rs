use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;
use sapic_base::workspace::types::WorkspaceInfo;
use sapic_ipc::contracts::workspace::ListWorkspacesOutput;

use crate::App;

impl<R: AppRuntime> App<R> {
    pub async fn list_workspaces(
        &self,
        _ctx: &R::AsyncContext,
        _delegate: &AppDelegate<R>,
    ) -> joinerror::Result<ListWorkspacesOutput> {
        let workspaces = self
            .services
            .workspace_service
            .restore_workspaces()
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
