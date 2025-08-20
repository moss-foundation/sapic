use joinerror::OptionExt;
use moss_applib::AppRuntime;

use crate::{
    app::App,
    models::operations::{CloseWorkspaceInput, CloseWorkspaceOutput},
};

impl<R: AppRuntime> App<R> {
    pub async fn close_workspace(
        &self,
        ctx: &R::AsyncContext,
        input: &CloseWorkspaceInput,
    ) -> joinerror::Result<CloseWorkspaceOutput> {
        let workspace_id = self
            .workspace()
            .await
            .map(|w| w.id())
            .ok_or_join_err::<()>("no active workspace to close")?;

        if workspace_id != input.id {
            return Err(joinerror::Error::new::<()>(format!(
                "Workspace {} is not currently active",
                input.id
            )));
        }

        self.workspace_service.deactivate_workspace(ctx).await?;

        Ok(CloseWorkspaceOutput { id: workspace_id })
    }
}
