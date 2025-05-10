use chrono::Utc;
use moss_common::api::{OperationError, OperationResult};
use moss_storage::global_storage::entities::WorkspaceInfoEntity;
use moss_workspace::workspace::Workspace;
use std::sync::Arc;
use tauri::Runtime as TauriRuntime;

use crate::{
    models::operations::{OpenWorkspaceInput, OpenWorkspaceOutput},
    workbench::{Workbench, WorkspaceInfoEntry},
};

impl<R: TauriRuntime> Workbench<R> {
    pub async fn open_workspace(
        &self,
        input: &OpenWorkspaceInput,
    ) -> OperationResult<OpenWorkspaceOutput> {
        let target_workspace_entry =
            if let Some(workspace) = self.workspace_by_name(&input.name).await? {
                workspace
            } else {
                return Err(OperationError::NotFound {
                    name: input.name.clone(),
                    path: self.absolutize(&input.name),
                });
            };

        if !target_workspace_entry.abs_path.exists() {
            return Err(OperationError::NotFound {
                name: target_workspace_entry.name.clone(),
                path: target_workspace_entry.abs_path.to_path_buf(),
            });
        }

        // Check if the workspace is already active
        if self.active_workspace()?.id == target_workspace_entry.id {
            return Ok(OpenWorkspaceOutput {
                id: target_workspace_entry.id,
                abs_path: Arc::clone(&target_workspace_entry.abs_path),
            });
        }

        let workspace = Workspace::new(
            self.app_handle.clone(),
            Arc::clone(&target_workspace_entry.abs_path),
            Arc::clone(&self.fs),
            self.activity_indicator.clone(),
        )?;

        let last_opened_at = Utc::now().timestamp();

        // Update the workspace entry in the known workspaces map
        {
            let updated_workspace_entry = WorkspaceInfoEntry {
                id: target_workspace_entry.id,
                name: target_workspace_entry.name.to_owned(),
                display_name: target_workspace_entry.display_name.to_owned(),
                abs_path: Arc::clone(&target_workspace_entry.abs_path),
                last_opened_at: Some(last_opened_at),
            };

            let known_workspaces = self.known_workspaces().await?;
            known_workspaces
                .write()
                .await
                .insert(target_workspace_entry.id, Arc::new(updated_workspace_entry));
        }

        // Update the workspace entry in the global storage
        {
            let workspace_storage = self.global_storage.workspaces_store();
            let mut txn = self.global_storage.begin_write().await?;
            workspace_storage.upsert_workspace(
                &mut txn,
                target_workspace_entry.name.to_owned(),
                WorkspaceInfoEntity { last_opened_at },
            )?;
            txn.commit()?;
        }

        self.set_active_workspace(target_workspace_entry.id, workspace);

        Ok(OpenWorkspaceOutput {
            id: target_workspace_entry.id,
            abs_path: target_workspace_entry.abs_path.to_owned(),
        })
    }
}
