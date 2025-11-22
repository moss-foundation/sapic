use moss_applib::{AppRuntime, errors::ValidationResultExt};
use sapic_ipc::contracts::workspace::{UpdateWorkspaceInput, UpdateWorkspaceOutput};
use sapic_system::workspace::{WorkspaceEditOp, WorkspaceEditParams};
use validator::Validate;

use crate::App;

impl<R: AppRuntime> App<R> {
    pub async fn update_workspace(
        &self,
        _ctx: &R::AsyncContext,
        input: &UpdateWorkspaceInput,
    ) -> joinerror::Result<UpdateWorkspaceOutput> {
        input.validate().join_err_bare()?;

        self.services
            .workspace_edit_service
            .edit(
                &input.id,
                WorkspaceEditParams {
                    name: input.name.clone(),
                },
            )
            .await?;

        todo!()
    }
}
