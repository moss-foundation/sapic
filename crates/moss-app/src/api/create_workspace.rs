use crate::{
    app::App,
    context::AnyAppContext,
    models::operations::{CreateWorkspaceInput, CreateWorkspaceOutput},
    services::workspace_service::{WorkspaceItemCreateParams, WorkspaceService},
};
use moss_common::{api::OperationResult, new_nanoid};
use tauri::Runtime as TauriRuntime;
use validator::Validate;

impl<R: TauriRuntime> App<R> {
    pub async fn create_workspace<C: AnyAppContext<R>>(
        &self,
        ctx: &C,
        input: &CreateWorkspaceInput,
    ) -> OperationResult<CreateWorkspaceOutput> {
        input.validate()?;

        let workspace_service = self.services.get::<WorkspaceService<R>>();

        let id = new_nanoid();
        let item = workspace_service
            .create_workspace(
                &id,
                WorkspaceItemCreateParams {
                    name: input.name.to_owned(),
                },
            )
            .await?;

        if input.open_on_creation {
            workspace_service
                .activate_workspace(ctx, &id, self.activity_indicator.clone())
                .await?;
        }

        Ok(CreateWorkspaceOutput {
            id: item.id.to_string(),
            active: input.open_on_creation,
            abs_path: item.abs_path.clone(),
        })
    }
}
