use moss_applib::AppRuntime;
use moss_common::api::OperationResult;
use validator::Validate;

use crate::{
    app::App, models::operations::UpdateWorkspaceInput,
    services::workspace_service::WorkspaceItemUpdateParams,
};

impl<R: AppRuntime> App<R> {
    pub async fn update_workspace(
        &self,
        _ctx: &R::AsyncContext,
        input: &UpdateWorkspaceInput,
    ) -> OperationResult<()> {
        input.validate()?;

        self.workspace_service
            .update_workspace(WorkspaceItemUpdateParams {
                name: input.name.to_owned(),
            })
            .await?;

        Ok(())
    }
}
