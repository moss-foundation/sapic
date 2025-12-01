use moss_applib::AppRuntime;

use sapic_ipc::{
    ValidationResultExt,
    contracts::main::workspace::{UpdateWorkspaceInput, UpdateWorkspaceOutput},
};
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
            .load()
            .edit(WorkspaceEditParams {
                name: input.name.clone(),
            })
            .await?;

        Ok(UpdateWorkspaceOutput {})
    }
}
