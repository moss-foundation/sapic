use moss_applib::AppRuntime;

use crate::{
    app::App,
    models::operations::{OpenWorkspaceInput, OpenWorkspaceOutput},
};

// FIXME: Allow the workspace to be opened even if it encounters invalid collection
impl<R: AppRuntime> App<R> {
    pub async fn open_workspace(
        &self,
        ctx: &R::AsyncContext,
        input: &OpenWorkspaceInput,
    ) -> joinerror::Result<OpenWorkspaceOutput> {
        let desc = self
            .workspace_service
            .activate_workspace(ctx, &input.id, self.broadcaster.clone())
            .await?;

        Ok(OpenWorkspaceOutput {
            id: desc.id,
            abs_path: desc.abs_path.to_owned(),
        })
    }
}
