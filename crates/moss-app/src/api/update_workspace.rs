use moss_applib::AppRuntime;
use moss_common::api::OperationResult;
use validator::Validate;

use crate::{
    app::App,
    models::operations::UpdateWorkspaceInput,
    services::workspace_service::{WorkspaceItemUpdateParams, WorkspaceService},
};

impl<R: AppRuntime> App<R> {
    pub async fn update_workspace(
        &self,
        _ctx: &R::AsyncContext,
        input: &UpdateWorkspaceInput,
    ) -> OperationResult<()> {
        input.validate()?;

        let workspace_service = self.services.get::<WorkspaceService<R>>();
        workspace_service
            .update_workspace(WorkspaceItemUpdateParams {
                name: input.name.to_owned(),
            })
            .await?;

        Ok(())
    }
}
