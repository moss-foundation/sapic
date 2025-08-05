use moss_applib::AppRuntime;
use moss_common::api::OperationResult;

use crate::{
    app::App,
    models::operations::{OpenWorkspaceInput, OpenWorkspaceOutput},
};

impl<R: AppRuntime> App<R> {
    pub async fn open_workspace(
        &self,
        ctx: &R::AsyncContext,
        input: &OpenWorkspaceInput,
    ) -> OperationResult<OpenWorkspaceOutput> {
        let desc = self
            .workspace_service
            .activate_workspace(
                ctx,
                &input.id,
                self.models.clone(),
                self.activity_indicator.clone(),
                self.github_client.clone(),
                self.gitlab_client.clone(),
            )
            .await?;

        Ok(OpenWorkspaceOutput {
            id: desc.id,
            abs_path: desc.abs_path.to_owned(),
        })
    }
}
