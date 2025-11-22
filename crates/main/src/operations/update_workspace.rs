use moss_applib::{AppRuntime, errors::ValidationResultExt};
use sapic_ipc::contracts::main::workspace::{UpdateWorkspaceInput, UpdateWorkspaceOutput};
use sapic_system::workspace::WorkspaceEditParams;
use validator::Validate;

use crate::MainWindow;

impl<R: AppRuntime> MainWindow<R> {
    pub async fn update_workspace(
        &self,
        _ctx: &R::AsyncContext,
        input: &UpdateWorkspaceInput,
    ) -> joinerror::Result<UpdateWorkspaceOutput> {
        input.validate().join_err_bare()?;

        self.workspace
            .edit(WorkspaceEditParams {
                name: input.name.clone(),
            })
            .await?;

        Ok(UpdateWorkspaceOutput {})
    }
}
