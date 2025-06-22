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
        let workspace_service = self.service::<WorkspaceService<R>>();
        let workspaces = workspace_service
            .map_known_workspaces_to_vec(|id, descriptor| WorkspaceInfo {
                id,
                display_name: descriptor.name.clone(),
                last_opened_at: descriptor.last_opened_at,
            })
            .await?;

        Ok(ListWorkspacesOutput(workspaces))
    }
}
