use anyhow::Context as _;
use moss_common::api::OperationResult;
use moss_fs::RemoveOptions;
use tauri::Runtime as TauriRuntime;

use crate::{models::operations::DeleteWorkspaceInput, workspace_manager::WorkspaceManager};

impl<R: TauriRuntime> WorkspaceManager<R> {
    pub async fn delete_workspace(&self, input: &DeleteWorkspaceInput) -> OperationResult<()> {
        let known_workspaces = self.known_workspaces().await?;

        let mut workspaces_lock = known_workspaces.write().await;
        let workspace_info = workspaces_lock
            .remove(input.key)
            .context("Failed to remove the workspace")?;

        let workspace_path = workspace_info.path;

        // TODO: If any of the following operations fail, we should place the task
        // in the dead queue and attempt the deletion later.

        let workspace_storage = self.global_storage.workspaces_store();
        let mut txn = self.global_storage.begin_write().await?;
        workspace_storage.delete_workspace(&mut txn, workspace_info.name)?;

        // TODO: logging if the folder has already been removed from the filesystem
        self.fs
            .remove_dir(
                &workspace_path,
                RemoveOptions {
                    recursive: true,
                    ignore_if_not_exists: true,
                },
            )
            .await?;

        txn.commit()?;

        // Deleting a workspace will remove it from current workspace if it is
        let current_entry = self.current_workspace.swap(None);

        if let Some(entry) = current_entry {
            if entry.0 != input.key {
                self.current_workspace.store(Some(entry))
            }
        }

        Ok(())
    }
}
