use moss_common::api::{OperationOptionExt, OperationResult};
use moss_fs::RemoveOptions;
use moss_storage::storage::operations::RemoveItem;
use tauri::Runtime as TauriRuntime;

use crate::{
    app::App,
    context::{AnyAppContext, ctxkeys},
    models::operations::DeleteWorkspaceInput,
    services::workspace_service::WorkspaceService,
    storage::segments::WORKSPACE_SEGKEY,
};

impl<R: TauriRuntime> App<R> {
    pub async fn delete_workspace<C: AnyAppContext<R>>(
        &self,
        ctx: &C,
        input: &DeleteWorkspaceInput,
    ) -> OperationResult<()> {
        let workspace_service = self.service::<WorkspaceService<R>>();
        let workspaces = workspace_service.workspaces().await?;

        let workspace_desc = workspaces
            .read()
            .await
            .get(&input.id)
            .cloned()
            .map_err_as_not_found("Failed to delete the workspace")?;

        if workspace_desc.abs_path.exists() {
            self.fs
                .remove_dir(
                    &workspace_desc.abs_path,
                    RemoveOptions {
                        recursive: true,
                        ignore_if_not_exists: true,
                    },
                )
                .await?;
        }

        {
            let item_store = self.global_storage.item_store();
            let segkey = WORKSPACE_SEGKEY.join(input.id.to_string());
            // Only try to remove from database if it exists (ignore error if not found)
            let _ = RemoveItem::remove(item_store.as_ref(), segkey);
        }

        {
            let mut workspaces_lock = workspaces.write().await;
            workspaces_lock.remove(&workspace_desc.id);
        }

        if let Some(active_workspace_id) = ctx.value::<ctxkeys::WorkspaceId>().map(|id| **id) {
            if active_workspace_id == input.id {
                workspace_service.deactivate_workspace(ctx).await;
            }
        }

        Ok(())
    }
}
