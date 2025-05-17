use moss_common::api::OperationResult;
use tauri::Runtime as TauriRuntime;

use crate::{
    models::{operations::ListWorkspacesOutput, types::WorkspaceInfo},
    workbench::Workbench,
};

impl<R: TauriRuntime> Workbench<R> {
    pub async fn list_workspaces(&self) -> OperationResult<ListWorkspacesOutput> {
        let workspaces = self.workspaces().await?;
        let workspaces_lock = workspaces.read().await;

        Ok(ListWorkspacesOutput(
            workspaces_lock
                .iter()
                .map(|(_, entry)| WorkspaceInfo {
                    id: entry.id,
                    display_name: entry.display_name.clone(),
                    last_opened_at: entry.last_opened_at,
                })
                .collect(),
        ))
    }
}
