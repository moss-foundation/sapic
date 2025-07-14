use moss_applib::AppRuntime;
use moss_common::api::OperationResult;
use validator::Validate;

use crate::{
    app::App,
    models::{
        operations::{CreateWorkspaceInput, CreateWorkspaceOutput},
        primitives::WorkspaceId,
    },
    services::workspace_service::{WorkspaceItemCreateParams, WorkspaceService},
};

impl<R: AppRuntime> App<R> {
    pub async fn create_workspace(
        &self,
        ctx: &R::AsyncContext,
        input: &CreateWorkspaceInput,
    ) -> OperationResult<CreateWorkspaceOutput> {
        input.validate()?;

        let workspace_service = self.services.get::<WorkspaceService<R>>();

        let id = WorkspaceId::new();
        let item = workspace_service
            .create_workspace(
                &id,
                WorkspaceItemCreateParams {
                    name: input.name.to_owned(),
                },
            )
            .await?;

        if input.open_on_creation {
            workspace_service
                .activate_workspace(ctx, &id, self.activity_indicator.clone())
                .await?;
        }

        Ok(CreateWorkspaceOutput {
            id: item.id,
            active: input.open_on_creation,
            abs_path: item.abs_path.clone(),
        })
    }
}
