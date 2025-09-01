use moss_applib::{AppRuntime, errors::ValidationResultExt};
use validator::Validate;

use crate::{
    app::App,
    models::{
        operations::{CreateWorkspaceInput, CreateWorkspaceOutput},
        primitives::WorkspaceId,
    },
    services::workspace_service::WorkspaceItemCreateParams,
};

impl<R: AppRuntime> App<R> {
    pub async fn create_workspace(
        &self,
        ctx: &R::AsyncContext,
        input: &CreateWorkspaceInput,
    ) -> joinerror::Result<CreateWorkspaceOutput> {
        input.validate().join_err_bare()?;

        let active_profile = self.profile_service.active_profile();

        let id = WorkspaceId::new();
        let item = self
            .workspace_service
            .create_workspace(
                &id,
                WorkspaceItemCreateParams {
                    name: input.name.to_owned(),
                },
            )
            .await?;

        if input.open_on_creation {
            self.workspace_service
                .activate_workspace(ctx, &id, active_profile, self.broadcaster.clone())
                .await?;
        }

        Ok(CreateWorkspaceOutput {
            id: item.id,
            active: input.open_on_creation,
            abs_path: item.abs_path.clone(),
        })
    }
}
