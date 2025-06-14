use moss_applib::context::Context;
use moss_common::api::OperationResult;
use tauri::Runtime as TauriRuntime;

use crate::{
    models::{operations::ListWorkspacesOutput, types::WorkspaceInfo},
    workbench::Workbench,
};

impl<R: TauriRuntime> Workbench<R> {
    pub async fn list_workspaces<C: Context<R>>(
        &self,
        ctx: &C,
    ) -> OperationResult<ListWorkspacesOutput> {
        let workspaces = self.workspaces(ctx).await?;
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
