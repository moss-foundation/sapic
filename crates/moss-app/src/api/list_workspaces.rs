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
        let workspaces = workspace_service.workspaces().await?;
        let workspaces_lock = workspaces.read().await;

        Ok(ListWorkspacesOutput(
            workspaces_lock
                .iter()
                .map(|(_, entry)| WorkspaceInfo {
                    id: entry.id,
                    display_name: entry.name.clone(),
                    last_opened_at: entry.last_opened_at,
                })
                .collect(),
        ))
    }
}
