use anyhow::{Context, Result};
use moss_fs::utils::encode_directory_name;
use moss_fs::RenameOptions;
use std::sync::Arc;
use tauri::Runtime as TauriRuntime;
use validator::Validate;

use crate::models::operations::RenameWorkspaceInput;
use crate::workspace::Workspace;
use crate::workspace_manager::{OperationError, WorkspaceManager};

impl<R: TauriRuntime> WorkspaceManager<R> {
    pub async fn rename_workspace(
        &self,
        input: RenameWorkspaceInput,
    ) -> Result<(), OperationError> {
        input.validate()?;

        let workspaces = self
            .known_workspaces()
            .await
            .context("Failed to get known workspaces")?;

        let mut workspaces_lock = workspaces.write().await;
        let workspace_info = workspaces_lock
            .get_mut(input.key)
            .context("Failed to lease the workspace")?;

        if workspace_info.name == input.new_name {
            return Ok(());
        }

        let old_path = workspace_info.path.clone();
        if !old_path.exists() {
            return Err(OperationError::NotFound {
                name: workspace_info.name.clone(),
                path: old_path,
            });
        }

        let new_path = old_path
            .parent()
            .context("Parent directory not found")?
            .join(encode_directory_name(&input.new_name));
        if new_path.exists() {
            return Err(OperationError::AlreadyExists {
                name: input.new_name,
                path: new_path,
            });
        }

        // An opened workspace db will prevent its parent folder from being renamed
        // If we are renaming the current workspace, we need to call the reset method

        let current_entry = self.current_workspace.swap(None);

        // FIXME: This is probably not the best approach
        // If the current workspace needs to be renamed
        // We will first drop the workspace, do fs renaming, and reload it
        if let Some(mut entry) = current_entry {
            if entry.0 == input.key {
                std::mem::drop(entry);
                self.fs
                    .rename(&old_path, &new_path, RenameOptions::default())
                    .await?;
                entry = Arc::new((
                    input.key,
                    Workspace::new(
                        self.app_handle.clone(),
                        new_path.clone(),
                        self.fs.clone(),
                        self.activity_indicator.clone(),
                    )?,
                ))
            } else {
                self.fs
                    .rename(&old_path, &new_path, RenameOptions::default())
                    .await?;
            }
            self.current_workspace.store(Some(entry))
        } else {
            self.fs
                .rename(&old_path, &new_path, RenameOptions::default())
                .await?;
        }

        workspace_info.name = input.new_name;
        workspace_info.path = new_path;

        Ok(())
    }
}
