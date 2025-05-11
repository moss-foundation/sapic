use moss_common::api::{OperationError, OperationResult, OperationResultExt};
use moss_fs::{RenameOptions, utils::encode_name};
use moss_workspace::workspace::Workspace;
use std::{path::Path, sync::Arc};
use tauri::Runtime as TauriRuntime;
use validator::Validate;

use crate::{
    models::operations::UpdateWorkspaceInput,
    workbench::{Workbench, WorkspaceInfoEntry},
};

impl<R: TauriRuntime> Workbench<R> {
    pub async fn update_workspace(&self, input: UpdateWorkspaceInput) -> OperationResult<()> {
        input.validate()?;

        let workspaces = self.known_workspaces().await?;
        let workspace_info_entry = workspaces
            .read()
            .await
            .get(&input.id)
            .ok_or(OperationError::NotFound {
                name: input.id.to_string(),
                path: self.absolutize(&input.id.to_string()),
            })?
            .clone();

        if let Some(new_name) = input.name {
            self.rename_workspace(&workspace_info_entry, new_name)
                .await?;
        }

        Ok(())
    }

    async fn rename_workspace(
        &self,
        workspace_info: &Arc<WorkspaceInfoEntry>,
        new_name: String,
    ) -> OperationResult<()> {
        if workspace_info.name == new_name {
            return Ok(());
        }

        let new_encoded_name = encode_name(&new_name);
        let new_abs_path: Arc<Path> = self.absolutize(&new_encoded_name).into();
        if new_abs_path.exists() {
            return Err(OperationError::AlreadyExists {
                name: new_encoded_name,
                path: new_abs_path.to_path_buf(),
            });
        }

        // An opened workspace db will prevent its parent folder from being renamed
        // If we are renaming the current workspace, we need to call the reset method

        if self
            .active_workspace()
            .map(|active_workspace| active_workspace.id == workspace_info.id)
            .unwrap_or(false)
        {
            // FIXME: This is probably not the best approach
            // If the current workspace needs to be renamed
            // We will first drop the workspace, do fs renaming, and reload it
            let transit_workspace_entry = self.active_workspace.swap(None).unwrap(); // This is safe because we just checked that the active workspace is the one we want to rename
            drop(transit_workspace_entry);

            self.fs
                .rename(
                    &workspace_info.abs_path,
                    &new_abs_path,
                    RenameOptions::default(),
                )
                .await
                .map_err_as_internal()?;

            self.set_active_workspace(
                workspace_info.id,
                Workspace::new(
                    self.app_handle.clone(),
                    Arc::clone(&new_abs_path),
                    Arc::clone(&self.fs),
                    self.activity_indicator.clone(),
                )?,
            );
        } else {
            self.fs
                .rename(
                    &workspace_info.abs_path,
                    &new_abs_path,
                    RenameOptions::default(),
                )
                .await
                .map_err_as_internal()?;
        }

        {
            let mut txn = self.global_storage.begin_write().await?;
            self.global_storage.workspaces_store().rekey_workspace(
                &mut txn,
                workspace_info.name.clone(),
                new_encoded_name.clone(),
            )?;
            txn.commit()?;
        }

        dbg!(&new_name);

        {
            let mut workspaces_lock = self.known_workspaces().await?.write().await;
            workspaces_lock.insert(
                workspace_info.id,
                Arc::new(WorkspaceInfoEntry {
                    id: workspace_info.id,
                    name: new_encoded_name,
                    display_name: new_name,
                    abs_path: new_abs_path,
                    last_opened_at: workspace_info.last_opened_at,
                }),
            );
        }

        Ok(())
    }
}
