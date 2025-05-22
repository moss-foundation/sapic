use anyhow::Context as _;
use moss_common::api::{OperationError, OperationResult, OperationResultExt};
use moss_fs::RemoveOptions;
use moss_storage::storage::operations::RemoveItem;
use tauri::Runtime as TauriRuntime;

use crate::{
    models::operations::DeleteWorkspaceInput, storage::segments::WORKSPACE_SEGKEY,
    workbench::Workbench,
};

impl<R: TauriRuntime> Workbench<R> {
    pub async fn delete_workspace(&self, input: &DeleteWorkspaceInput) -> OperationResult<()> {
        let workspaces = self.workspaces().await?;

        let workspace_entry = workspaces
            .read()
            .await
            .get(&input.id)
            .cloned()
            .context("Failed to remove the workspace")
            .map_err_as_not_found()?;

        if !workspace_entry.abs_path.exists() {
            // TODO: if a path is not found, we also need to remove the workspace from the database and clean up other caches
            return Err(OperationError::NotFound(
                workspace_entry.abs_path.to_string_lossy().to_string(),
            ));
        }

        self.fs
            .remove_dir(
                &workspace_entry.abs_path,
                RemoveOptions {
                    recursive: true,
                    ignore_if_not_exists: true,
                },
            )
            .await?;

        {
            let item_store = self.global_storage.item_store();
            let segkey = WORKSPACE_SEGKEY.join(input.id.to_string());
            RemoveItem::remove(item_store.as_ref(), segkey)?;
        }

        {
            let mut workspaces_lock = workspaces.write().await;
            workspaces_lock.remove(&workspace_entry.id);
        }

        if let Some(active_workspace) = self.active_workspace.load().as_ref() {
            if active_workspace.id == input.id {
                self.active_workspace.store(None);
            }
        }

        Ok(())
    }
}
