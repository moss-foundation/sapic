use moss_applib::AppRuntime;
use sapic_ipc::{
    ValidationResultExt,
    contracts::welcome::workspace::{UpdateWorkspaceInput, UpdateWorkspaceOutput},
};
use sapic_system::workspace::WorkspaceEditParams;
use validator::Validate;

use crate::WelcomeWindow;

impl<R: AppRuntime> WelcomeWindow<R> {
    pub async fn update_workspace(
        &self,
        ctx: &R::AsyncContext,
        input: &UpdateWorkspaceInput,
    ) -> joinerror::Result<UpdateWorkspaceOutput> {
        input.validate().join_err_bare()?;

        self.workspace_ops
            .update_workspace(
                ctx,
                &input.id,
                WorkspaceEditParams {
                    name: input.name.clone(),
                },
            )
            .await?;

        Ok(UpdateWorkspaceOutput {})
    }
}
