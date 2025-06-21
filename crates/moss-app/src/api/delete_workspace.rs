use moss_applib::context::Context;
use moss_common::api::{OperationError, OperationOptionExt, OperationResult};
use moss_fs::{FileSystem, RemoveOptions};
use moss_storage::storage::operations::RemoveItem;
use tauri::Runtime as TauriRuntime;

use crate::{
    app::App, models::operations::DeleteWorkspaceInput, storage::segments::WORKSPACE_SEGKEY,
};

impl<R: TauriRuntime> App<R> {
    pub async fn delete_workspace<C: Context<R>>(
        &self,
        ctx: &C,
        input: &DeleteWorkspaceInput,
    ) -> OperationResult<()> {
        let fs = <dyn FileSystem>::global::<R, C>(ctx);
        let workspaces = self.workspaces(ctx).await?;

        let workspace_desc = workspaces
            .read()
            .await
            .get(&input.id)
            .cloned()
            .map_err_as_not_found("Failed to delete the workspace")?;

        // Remove directory if it exists
        if workspace_desc.abs_path.exists() {
            fs.remove_dir(
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

        if let Some(active_workspace_id) = self.active_workspace_id().await {
            if active_workspace_id == input.id {
                self.deactivate_workspace().await;
            }
        }

        Ok(())
    }
}
