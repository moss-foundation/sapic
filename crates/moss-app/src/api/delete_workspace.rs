use moss_applib::ctx::Context;
use moss_common::api::OperationResult;
use tauri::Runtime as TauriRuntime;

use crate::{
    app::App, models::operations::DeleteWorkspaceInput,
    services::workspace_service::WorkspaceService,
};

impl<R: TauriRuntime> App<R> {
    pub async fn delete_workspace<C: Context>(
        &self,
        ctx: &C,
        input: &DeleteWorkspaceInput,
    ) -> OperationResult<()> {
        let workspace_service = self.services.get::<WorkspaceService<R>>();
        workspace_service.delete_workspace(ctx, &input.id).await?;

        Ok(())
    }
}
