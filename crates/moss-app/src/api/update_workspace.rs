use moss_applib::ctx::Context;
use moss_common::api::OperationResult;
use tauri::Runtime as TauriRuntime;
use validator::Validate;

use crate::{
    app::App,
    models::operations::UpdateWorkspaceInput,
    services::workspace_service::{WorkspaceItemUpdateParams, WorkspaceService},
};

impl<R: TauriRuntime> App<R> {
    pub async fn update_workspace<C: Context>(
        &self,
        _ctx: &C,
        input: &UpdateWorkspaceInput,
    ) -> OperationResult<()> {
        input.validate()?;

        let workspace_service = self.services.get::<WorkspaceService<R>>();
        workspace_service
            .update_workspace(WorkspaceItemUpdateParams {
                name: input.name.to_owned(),
            })
            .await?;

        Ok(())
    }
}
