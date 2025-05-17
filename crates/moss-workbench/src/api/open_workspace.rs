use chrono::Utc;
use moss_common::api::{OperationError, OperationResult};
use moss_db::primitives::AnyValue;
use moss_storage::{global_storage::entities::WorkspaceInfoEntity, storage::operations::PutItem};
use moss_workspace::Workspace;
use std::sync::Arc;
use tauri::Runtime as TauriRuntime;

use crate::{
    models::operations::{OpenWorkspaceInput, OpenWorkspaceOutput},
    storage::segments::WORKSPACE_SEGKEY,
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
                return Err(OperationError::NotFound(format!(
                    "workspace with name {}",
                    input.name
                )));
            };

        if !target_workspace_entry.abs_path.exists() {
            return Err(OperationError::NotFound(
                target_workspace_entry
                    .abs_path
                    .to_string_lossy()
                    .to_string(),
            ));
        }

        // Check if the workspace is already active
        if self
            .active_workspace()
            .map(|active_workspace| active_workspace.id == target_workspace_entry.id)
            .unwrap_or(false)
        {
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

        {
            let updated_workspace_entry = WorkspaceInfoEntry {
                id: target_workspace_entry.id,
                name: target_workspace_entry.name.to_owned(),
                display_name: target_workspace_entry.display_name.to_owned(),
                abs_path: Arc::clone(&target_workspace_entry.abs_path),
                last_opened_at: Some(last_opened_at),
            };

            let known_workspaces = self.workspaces().await?;
            known_workspaces
                .write()
                .await
                .insert(target_workspace_entry.id, Arc::new(updated_workspace_entry));
        }

        {
            let item_store = self.global_storage.item_store();
            let segkey = WORKSPACE_SEGKEY.join(target_workspace_entry.name.to_owned());
            let value = AnyValue::serialize(&WorkspaceInfoEntity { last_opened_at })?;
            PutItem::put(item_store.as_ref(), segkey, value)?;
        }

        self.set_active_workspace(target_workspace_entry.id, workspace);

        Ok(OpenWorkspaceOutput {
            id: target_workspace_entry.id,
            abs_path: target_workspace_entry.abs_path.to_owned(),
        })
    }
}
