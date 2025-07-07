use moss_applib::context::Context;
use moss_common::api::OperationResult;
use tauri::Runtime as TauriRuntime;

use crate::{
    app::App,
    models::{operations::ListWorkspacesOutput, types::WorkspaceInfo},
    services::workspace_service::WorkspaceService,
};

impl<R: TauriRuntime> App<R> {
    pub async fn list_workspaces<C: Context<R>>(
        &self,
        _ctx: &C,
    ) -> OperationResult<ListWorkspacesOutput> {
        let workspace_service = self.services.get::<WorkspaceService<R>>();
        let workspaces = workspace_service.list_workspaces().await?;
        let workspaces = workspaces
            .into_iter()
            .map(|item| WorkspaceInfo {
                id: item.id.to_string(),
                name: item.name.clone(),
                last_opened_at: item.last_opened_at,
                active: item.active,
                abs_path: item.abs_path,
            })
            .collect();

        Ok(ListWorkspacesOutput(workspaces))
    }
}
