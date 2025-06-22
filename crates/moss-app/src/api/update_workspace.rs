use moss_common::api::OperationResult;
use moss_workspace::workspace;
use tauri::Runtime as TauriRuntime;
use validator::Validate;

use crate::{
    app::App, context::AnyAppContext, models::operations::UpdateWorkspaceInput,
    services::workspace_service::WorkspaceService,
};

impl<R: TauriRuntime> App<R> {
    pub async fn update_workspace<C: AnyAppContext<R>>(
        &self,
        _ctx: &C,
        input: &UpdateWorkspaceInput,
    ) -> OperationResult<()> {
        input.validate()?;

        let workspace_service = self.service::<WorkspaceService<R>>();
        let params = workspace::ModifyParams {
            name: input.name.to_owned(),
        };

        workspace_service.update_workspace(params).await?;

        Ok(())
    }
}
