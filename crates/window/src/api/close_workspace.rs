use joinerror::{Error, OptionExt};
use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;

use crate::{
    models::operations::{CloseWorkspaceInput, CloseWorkspaceOutput},
    window::Window,
};

impl<R: AppRuntime> Window<R> {
    pub async fn close_workspace(
        &self,
        ctx: &R::AsyncContext,
        app_delegate: &AppDelegate<R>,
        input: &CloseWorkspaceInput,
    ) -> joinerror::Result<CloseWorkspaceOutput> {
        let workspace_id = self
            .workspace()
            .await
            .map(|w| w.id())
            .ok_or_join_err::<()>("no active workspace to close")?;

        if workspace_id != input.id {
            return Err(Error::new::<()>(format!(
                "Workspace {} is not currently active",
                input.id
            )));
        }

        self.workspace_service
            .deactivate_workspace(ctx, app_delegate)
            .await?;

        Ok(CloseWorkspaceOutput { id: workspace_id })
    }
}
