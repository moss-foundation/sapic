use moss_common::api::OperationResult;
use tauri::Runtime as TauriRuntime;

use crate::{
    app::App,
    context::AnyAppContext,
    models::operations::{OpenWorkspaceInput, OpenWorkspaceOutput},
    services::workspace_service::WorkspaceService,
};

impl<R: TauriRuntime> App<R> {
    pub async fn open_workspace<C: AnyAppContext<R>>(
        &self,
        ctx: &C,
        input: &OpenWorkspaceInput,
    ) -> OperationResult<OpenWorkspaceOutput> {
        let workspace_service = self.services.get::<WorkspaceService<R>>();
        let desc = workspace_service
            .activate_workspace(ctx, &input.id, self.activity_indicator.clone())
            .await?;

        Ok(OpenWorkspaceOutput {
            id: desc.id,
            abs_path: desc.abs_path.to_owned(),
        })
    }
}
