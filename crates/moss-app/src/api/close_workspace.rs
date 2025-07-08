use moss_common::api::{OperationError, OperationOptionExt, OperationResult};
use tauri::Runtime as TauriRuntime;

use crate::{
    app::App,
    context::{AnyAppContext, ctxkeys},
    models::operations::{CloseWorkspaceInput, CloseWorkspaceOutput},
    services::workspace_service::WorkspaceService,
};

impl<R: TauriRuntime> App<R> {
    pub async fn close_workspace<C: AnyAppContext<R>>(
        &self,
        ctx: &C,
        input: &CloseWorkspaceInput,
    ) -> OperationResult<CloseWorkspaceOutput> {
        let workspace_service = self.services.get::<WorkspaceService<R>>();
        let workspace_id = ctx
            .value::<ctxkeys::ActiveWorkspaceId>()
            .map(|id| (*id).clone())
            .map_err_as_failed_precondition("No active workspace to close")?;

        if workspace_id != input.id {
            return Err(OperationError::InvalidInput(format!(
                "Workspace {} is not currently active",
                input.id
            )));
        }

        let _ = workspace_service.deactivate_workspace(ctx).await;

        Ok(CloseWorkspaceOutput { id: workspace_id })
    }
}
