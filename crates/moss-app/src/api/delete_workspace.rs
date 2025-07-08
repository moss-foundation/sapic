use moss_common::api::OperationResult;
use tauri::Runtime as TauriRuntime;

use crate::{
    app::App, context::AnyAppContext, models::operations::DeleteWorkspaceInput,
    services::workspace_service::WorkspaceService,
};

impl<R: TauriRuntime> App<R> {
    pub async fn delete_workspace<C: AnyAppContext<R>>(
        &self,
        ctx: &C,
        input: &DeleteWorkspaceInput,
    ) -> OperationResult<()> {
        let workspace_service = self.services.get::<WorkspaceService<R>>();
        workspace_service.delete_workspace(ctx, &input.id).await?;

        Ok(())
    }
}
