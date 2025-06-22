use moss_common::api::{OperationOptionExt, OperationResult};
use moss_workspace::workspace;
use tauri::Runtime as TauriRuntime;
use validator::Validate;

use crate::{
    app::App,
    context::{AnyAppContext, ctxkeys},
    models::operations::UpdateWorkspaceInput,
    services::workspace_service::WorkspaceService,
};

impl<R: TauriRuntime> App<R> {
    pub async fn update_workspace<C: AnyAppContext<R>>(
        &self,
        ctx: &C,
        input: &UpdateWorkspaceInput,
    ) -> OperationResult<()> {
        input.validate()?;

        let workspace_service = self.service::<WorkspaceService<R>>();
        // let workspaces = workspace_service.workspaces().await?;
        // let mut workspace_guard = workspace_service.workspace_mut().await;
        // let workspace = workspace_guard
        //     .as_mut()
        //     .map_err_as_failed_precondition("No active workspace")?;

        // let workspace_id = ctx
        //     .value::<ctxkeys::WorkspaceId>()
        //     .map(|id| **id)
        //     .map_err_as_internal("The required context value is not provided")?;

        // let mut descriptor = {
        //     let workspaces_lock = workspaces.read().await;

        //     workspaces_lock
        //         .get(&workspace_id)
        //         .map_err_as_internal("Workspace not found")? // This should never happen, if it does, there is a bug
        //         .as_ref()
        //         .clone()
        // };

        // if let Some(new_name) = input.name.as_ref().and_then(|new_name| {
        //     if new_name != &descriptor.name {
        //         Some(new_name)
        //     } else {
        //         None
        //     }
        // }) {
        //     workspace
        //         .modify(workspace::ModifyParams {
        //             name: Some(new_name.clone()),
        //         })
        //         .await?;
        //     descriptor.name = new_name.to_owned();
        // }

        // {
        //     let mut workspaces_lock = workspaces.write().await;
        //     workspaces_lock.insert(workspace_id, descriptor.into());
        // }

        let params = workspace::ModifyParams {
            name: input.name.clone(),
        };

        workspace_service.update_workspace(params).await?;

        Ok(())
    }
}
