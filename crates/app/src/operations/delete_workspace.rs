use joinerror::ResultExt;
use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;

use sapic_ipc::{
    ValidationResultExt,
    contracts::workspace::{DeleteWorkspaceInput, DeleteWorkspaceOutput},
};
use validator::Validate;

use crate::App;

impl<R: AppRuntime> App<R> {
    pub async fn delete_workspace(
        &self,
        _ctx: &R::AsyncContext,
        delegate: &AppDelegate<R>,
        input: &DeleteWorkspaceInput,
    ) -> joinerror::Result<DeleteWorkspaceOutput> {
        input.validate().join_err_bare()?;

        let maybe_window = self.windows.main_window_by_workspace_id(&input.id).await;
        if let Some(window) = maybe_window {
            self.windows
                .close_main_window(delegate, window.label())
                .await?;
        }

        let maybe_abs_path = self
            .services
            .workspace_service
            .delete_workspace(&input.id)
            .await
            .join_err_with::<()>(|| format!("failed to delete workspace `{}`", input.id))?;

        Ok(DeleteWorkspaceOutput {
            id: input.id.clone(),
            abs_path: maybe_abs_path,
        })
    }
}
