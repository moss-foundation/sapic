use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;

use crate::{
    app::App,
    models::operations::{OpenWorkspaceInput, OpenWorkspaceOutput},
};

// FIXME: Allow the workspace to be opened even if it encounters invalid collection
// Ticket: https://mossland.atlassian.net/browse/SAPIC-514

impl<R: AppRuntime> App<R> {
    pub async fn open_workspace(
        &self,
        ctx: &R::AsyncContext,
        app_delegate: &AppDelegate<R>,
        input: &OpenWorkspaceInput,
    ) -> joinerror::Result<OpenWorkspaceOutput> {
        let active_profile = self.profile_service.active_profile().await;

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
