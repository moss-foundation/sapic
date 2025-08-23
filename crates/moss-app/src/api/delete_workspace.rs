use moss_applib::AppRuntime;

use crate::{app::App, models::operations::DeleteWorkspaceInput};

impl<R: AppRuntime> App<R> {
    pub async fn delete_workspace(
        &self,
        ctx: &R::AsyncContext,
        input: &DeleteWorkspaceInput,
    ) -> joinerror::Result<()> {
        self.workspace_service
            .delete_workspace(ctx, &input.id)
            .await?;

        Ok(())
    }
}
