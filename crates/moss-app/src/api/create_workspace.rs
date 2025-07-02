use moss_common::api::OperationResult;
use tauri::Runtime as TauriRuntime;
use validator::Validate;

use crate::{
    app::App,
    context::AnyAppContext,
    models::operations::{CreateWorkspaceInput, CreateWorkspaceOutput},
    services::workspace_service::WorkspaceService,
};

impl<R: TauriRuntime> App<R> {
    pub async fn create_workspace<C: AnyAppContext<R>>(
        &self,
        ctx: &C,
        input: &CreateWorkspaceInput,
    ) -> OperationResult<CreateWorkspaceOutput> {
        input.validate()?;

        let workspace_service = self.service::<WorkspaceService<R>>();
        let (workspace, descriptor) = workspace_service
            .create_workspace(input.name.as_str(), self.activity_indicator.clone())
            .await?;

        if input.open_on_creation {
            workspace_service
                .activate_workspace(ctx, &descriptor.id, workspace)
                .await?;
        }

        Ok(CreateWorkspaceOutput {
            id: descriptor.id.clone(),
            active: input.open_on_creation,
            abs_path: descriptor.abs_path.clone(),
        })
    }
}
