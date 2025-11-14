use moss_applib::{AppRuntime, errors::ValidationResultExt};
use validator::Validate;

use crate::{
    models::operations::UpdateWorkspaceInput, window::Window, workspace::WorkspaceItemUpdateParams,
};

impl<R: AppRuntime> Window<R> {
    // TODO: Support renaming non-active workspace?
    pub async fn update_workspace(
        &self,
        _ctx: &R::AsyncContext,
        input: &UpdateWorkspaceInput,
    ) -> joinerror::Result<()> {
        input.validate().join_err_bare()?;

        self.workspace_service
            .update_workspace(WorkspaceItemUpdateParams {
                name: input.name.to_owned(),
            })
            .await?;

        Ok(())
    }
}
