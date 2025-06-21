use moss_common::api::{OperationError, OperationOptionExt, OperationResult};
use tauri::Runtime as TauriRuntime;

use crate::{
    app::App,
    models::operations::{CloseWorkspaceInput, CloseWorkspaceOutput},
};

impl<R: TauriRuntime> App<R> {
    pub async fn close_workspace(
        &self,
        input: &CloseWorkspaceInput,
    ) -> OperationResult<CloseWorkspaceOutput> {
        let active_workspace_id = self
            .active_workspace_id()
            .await
            .map_err_as_failed_precondition("No active workspace to close")?;

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
