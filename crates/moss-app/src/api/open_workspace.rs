use moss_applib::AppRuntime;
use moss_common::api::OperationResult;

use crate::{
    app::App,
    models::operations::{OpenWorkspaceInput, OpenWorkspaceOutput},
    services::workspace_service::WorkspaceService,
};

impl<R: AppRuntime> App<R> {
    pub async fn open_workspace(
        &self,
        ctx: &R::AsyncContext,
        input: &OpenWorkspaceInput,
    ) -> OperationResult<OpenWorkspaceOutput> {
        let workspace_service = self.services.get::<WorkspaceService<R>>();
        let desc = workspace_service
            .activate_workspace(ctx, &input.id, self.activity_indicator.clone())
            .await?;

        Ok(OpenWorkspaceOutput {
            id: desc.id,
            abs_path: desc.abs_path.to_owned(),
        })
    }
}
