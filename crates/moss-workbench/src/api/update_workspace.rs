use anyhow::Context as _;
use moss_applib::context::Context;
use moss_common::api::{OperationError, OperationResult, OperationResultExt};
use moss_workspace::workspace;
use tauri::Runtime as TauriRuntime;
use validator::Validate;

use crate::{models::operations::UpdateWorkspaceInput, workbench::Workbench};

impl<R: TauriRuntime> Workbench<R> {
    pub async fn update_workspace<C: Context<R>>(
        &self,
        ctx: &C,
        input: &UpdateWorkspaceInput,
    ) -> OperationResult<()> {
        input.validate()?;

        let workspaces = self.workspaces(ctx).await?;
        let mut workspace_guard = self.active_workspace_mut().await;
        let workspace = workspace_guard
            .as_mut()
            .context("No active workspace")
            .map_err_as_failed_precondition()?;

        let mut descriptor = {
            let workspaces_lock = workspaces.read().await;

            workspaces_lock
                .get(&workspace.id)
                .context("Workspace not found")
                .map_err_as_internal()? // This should never happen, if it does, there is a bug
                .as_ref()
                .clone()
        };

        if let Some(new_name) = input.name.as_ref().and_then(|new_name| {
            if new_name != &descriptor.name {
                Some(new_name)
            } else {
                None
            }
        }) {
            workspace
                .modify(workspace::ModifyParams {
                    name: Some(new_name.clone()),
                })
                .await?;
            descriptor.name = new_name.to_owned();
        }

        {
            let mut workspaces_lock = workspaces.write().await;
            workspaces_lock.insert(workspace.id, descriptor.into());
        }

        Ok(())
    }
}
