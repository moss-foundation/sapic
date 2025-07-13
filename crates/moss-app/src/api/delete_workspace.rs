use moss_applib::AppRuntime;
use moss_common::api::OperationResult;

use crate::{
    app::App, models::operations::DeleteWorkspaceInput,
    services::workspace_service::WorkspaceService,
};

impl<R: AppRuntime> App<R> {
    pub async fn delete_workspace(
        &self,
        ctx: &R::AsyncContext,
        input: &DeleteWorkspaceInput,
    ) -> OperationResult<()> {
        let workspace_service = self.services.get::<WorkspaceService<R>>();
        workspace_service.delete_workspace(ctx, &input.id).await?;

        Ok(())
    }
}
