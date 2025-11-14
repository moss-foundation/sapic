use moss_applib::AppRuntime;

use crate::{
    models::{operations::ListWorkspacesOutput, types::WorkspaceInfo},
    window::Window,
};

impl<R: AppRuntime> Window<R> {
    pub async fn list_workspaces(
        &self,
        _ctx: &R::AsyncContext,
    ) -> joinerror::Result<ListWorkspacesOutput> {
        let workspaces = self.workspace_service.list_workspaces().await?;
        let workspaces = workspaces
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
