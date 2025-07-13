use moss_applib::AppRuntime;
use moss_common::api::{OperationError, OperationOptionExt, OperationResult};

use crate::{
    app::App,
    models::operations::{CloseWorkspaceInput, CloseWorkspaceOutput},
    services::workspace_service::WorkspaceService,
};

impl<R: AppRuntime> App<R> {
    pub async fn close_workspace(
        &self,
        ctx: &R::AsyncContext,
        input: &CloseWorkspaceInput,
    ) -> OperationResult<CloseWorkspaceOutput> {
        let workspace_service = self.services.get::<WorkspaceService<R>>();
        let workspace_id = workspace_service
            .workspace()
            .await
            .map(|w| w.id())
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
