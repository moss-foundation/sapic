use moss_common::{
    api::{OperationError, OperationResult, OperationResultExt},
    sanitized::sanitize,
};
use moss_fs::RenameOptions;
use moss_storage::storage::operations::RekeyItem;
use moss_workspace::Workspace;
use std::{path::Path, sync::Arc};
use tauri::Runtime as TauriRuntime;
use validator::Validate;

use crate::{
    models::operations::UpdateWorkspaceInput,
    storage::segments::WORKSPACE_SEGKEY,
    workbench::{Workbench, WorkspaceInfoEntry},
};

impl<R: TauriRuntime> Workbench<R> {
    pub async fn update_workspace(&self, input: UpdateWorkspaceInput) -> OperationResult<()> {
        input.validate()?;

        let workspaces = self.workspaces().await?;
        let workspace_info_entry = workspaces
            .read()
            .await
            .get(&input.id)
            .ok_or(OperationError::NotFound(format!(
                "workspace with id {}",
                input.id
            )))?
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

        let new_encoded_name = sanitize(&new_name);
        let new_abs_path: Arc<Path> = self.absolutize(&new_encoded_name).into();
        if new_abs_path.exists() {
            return Err(OperationError::AlreadyExists(
                new_abs_path.to_string_lossy().to_string(),
            ));
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
            let item_store = self.global_storage.item_store();
            let old_segkey = WORKSPACE_SEGKEY.join(workspace_info.name.clone());
            let new_segkey = WORKSPACE_SEGKEY.join(new_encoded_name.clone());
            RekeyItem::rekey(item_store.as_ref(), old_segkey, new_segkey)?;
        }

        {
            let mut workspaces_lock = self.workspaces().await?.write().await;
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
