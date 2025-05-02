use moss_common::api::OperationResult;
use tauri::Runtime as TauriRuntime;

use crate::{models::operations::ListWorkspacesOutput, workspace_manager::WorkspaceManager};

impl<R: TauriRuntime> WorkspaceManager<R> {
    // TODO: (How) Should we write tests for this function?
    pub async fn list_workspaces(&self) -> OperationResult<ListWorkspacesOutput> {
        let workspaces = self.known_workspaces().await?;
        let workspaces_lock = workspaces.read().await;

        Ok(ListWorkspacesOutput(
            workspaces_lock
                .iter()
                .filter(|(_, iter_slot)| !iter_slot.is_leased())
                .map(|(_, iter_slot)| iter_slot.value().clone())
                .collect(),
        ))
    }
}
