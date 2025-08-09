use moss_applib::AppRuntime;
use moss_common::api::OperationResult;

use crate::{app::App, models::operations::DeleteWorkspaceInput};

impl<R: AppRuntime> App<R> {
    pub async fn delete_workspace(
        &self,
        ctx: &R::AsyncContext,
        input: &DeleteWorkspaceInput,
    ) -> OperationResult<()> {
        self.workspace_service
            .delete_workspace(ctx, &input.id)
            .await?;

        Ok(())
    }
}
