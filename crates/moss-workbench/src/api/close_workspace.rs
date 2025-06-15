use moss_common::api::{OperationError, OperationResult};
use tauri::Runtime as TauriRuntime;

use crate::{
    models::operations::{CloseWorkspaceInput, CloseWorkspaceOutput},
    workbench::Workbench,
};

impl<R: TauriRuntime> Workbench<R> {
    pub async fn close_workspace(
        &self,
        input: &CloseWorkspaceInput,
    ) -> OperationResult<CloseWorkspaceOutput> {
        let active_workspace_id = {
            let active_workspace = self.active_workspace().await;
            if let Some(workspace) = active_workspace.as_ref() {
                workspace.id
            } else {
                return Err(OperationError::InvalidInput(
                    "No active workspace to close".to_string(),
                ));
            }
        };

        if active_workspace_id != input.id {
            return Err(OperationError::InvalidInput(format!(
                "Workspace {} is not currently active",
                input.id
            )));
        }

        self.deactivate_workspace().await;

        Ok(CloseWorkspaceOutput {
            id: active_workspace_id,
        })
    }
}
