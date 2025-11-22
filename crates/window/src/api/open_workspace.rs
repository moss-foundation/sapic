use joinerror::OptionExt;
use moss_app_delegate::AppDelegate;
use moss_applib::{AppRuntime, errors::FailedPrecondition};

use crate::{
    models::operations::{OpenWorkspaceInput, OpenWorkspaceOutput},
    window::OldSapicWindow,
};

impl<R: AppRuntime> OldSapicWindow<R> {
    pub async fn open_workspace(
        &self,
        ctx: &R::AsyncContext,
        app_delegate: &AppDelegate<R>,
        input: &OpenWorkspaceInput,
    ) -> joinerror::Result<OpenWorkspaceOutput> {
        let active_profile = self
            .profile_service
            .active_profile()
            .await
            .ok_or_join_err::<FailedPrecondition>("no active profile to open a workspace")?;

        let desc = self
            .workspace_service
            .activate_workspace(ctx, app_delegate, &input.id, active_profile)
            .await?;

        Ok(OpenWorkspaceOutput {
            id: desc.id,
            abs_path: desc.abs_path.to_owned(),
        })
    }
}
