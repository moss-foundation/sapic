use anyhow::Context as _;
use moss_common::api::{OperationError, OperationResult, OperationResultExt};
use moss_fs::RemoveOptions;
use tauri::Runtime as TauriRuntime;

use crate::{models::operations::DeleteWorkspaceInput, workbench::Workbench};

impl<R: TauriRuntime> Workbench<R> {
    pub async fn delete_workspace(&self, input: &DeleteWorkspaceInput) -> OperationResult<()> {
        let workspaces = self.known_workspaces().await?;

        let workspace_entry = workspaces
            .read()
            .await
            .get(&input.id)
            .cloned()
            .context("Failed to remove the workspace")
            .map_err_as_not_found()?;

        if !workspace_entry.abs_path.exists() {
            return Err(OperationError::NotFound {
                name: workspace_entry.name.clone(),
                path: workspace_entry.abs_path.to_path_buf(),
            });
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
            let workspace_storage = self.global_storage.workspaces_store();
            let mut txn = self.global_storage.begin_write().await?;
            workspace_storage.delete_workspace(&mut txn, workspace_entry.name.to_owned())?;
            txn.commit()?;
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
