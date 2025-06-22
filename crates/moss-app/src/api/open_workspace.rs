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
        let workspace_service = self.service::<WorkspaceService<R>>();
        let (active, workspace, descriptor) = workspace_service
            .load_workspace(input.id, self.activity_indicator.clone())
            .await?;

        if active {
            return Ok(OpenWorkspaceOutput {
                id: descriptor.id,
                abs_path: descriptor.abs_path.to_owned(),
            });
        }

        workspace_service
            .activate_workspace(ctx, input.id, workspace)
            .await?;

        Ok(OpenWorkspaceOutput {
            id: descriptor.id,
            abs_path: descriptor.abs_path.to_owned(),
        })
    }
}
