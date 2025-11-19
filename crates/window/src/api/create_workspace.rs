use joinerror::OptionExt;
use moss_app_delegate::AppDelegate;
use moss_applib::{
    AppRuntime,
    errors::{FailedPrecondition, ValidationResultExt},
};
use sapic_base::workspace::types::primitives::WorkspaceId;
use validator::Validate;

use crate::{
    models::operations::{CreateWorkspaceInput, CreateWorkspaceOutput},
    window::Window,
    workspace::WorkspaceItemCreateParams,
};

impl<R: AppRuntime> Window<R> {
    pub async fn create_workspace(
        &self,
        ctx: &R::AsyncContext,
        app_delegate: &AppDelegate<R>,
        input: &CreateWorkspaceInput,
    ) -> joinerror::Result<CreateWorkspaceOutput> {
        input.validate().join_err_bare()?;

        let active_profile = self
            .profile_service
            .active_profile()
            .await
            .ok_or_join_err::<FailedPrecondition>("no active profile to create a workspace")?;

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
                .activate_workspace(ctx, app_delegate, &id, active_profile)
                .await?;
        }

        Ok(CreateWorkspaceOutput {
            id: item.id,
            active: input.open_on_creation,
            abs_path: item.abs_path.clone(),
        })
    }
}
