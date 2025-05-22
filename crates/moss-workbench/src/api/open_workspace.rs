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
    workbench::{Workbench, WorkspaceDescriptor},
};

impl<R: TauriRuntime> Workbench<R> {
    pub async fn open_workspace(
        &self,
        input: &OpenWorkspaceInput,
    ) -> OperationResult<OpenWorkspaceOutput> {
        let descriptor = if let Some(d) = self.workspace_by_name(&input.name).await? {
            d
        } else {
            return Err(OperationError::NotFound(format!(
                "workspace with name {}",
                input.name
            )));
        };

        if !descriptor.abs_path.exists() {
            return Err(OperationError::NotFound(
                descriptor.abs_path.to_string_lossy().to_string(),
            ));
        }

        // Check if the workspace is already active
        if self
            .active_workspace()
            .map(|active_workspace| active_workspace.id == descriptor.id)
            .unwrap_or(false)
        {
            return Ok(OpenWorkspaceOutput {
                id: descriptor.id,
                abs_path: Arc::clone(&descriptor.abs_path),
            });
        }

        let workspace = Workspace::load(
            self.app_handle.clone(),
            &descriptor.abs_path,
            Arc::clone(&self.fs),
            self.activity_indicator.clone(),
        )
        .await?;

        let last_opened_at = Utc::now().timestamp();

        {
            let updated_workspace_entry = WorkspaceDescriptor {
                id: descriptor.id,
                name: descriptor.name.to_owned(),
                abs_path: Arc::clone(&descriptor.abs_path),
                last_opened_at: Some(last_opened_at),
            };

            let known_workspaces = self.workspaces().await?;
            known_workspaces
                .write()
                .await
                .insert(descriptor.id, Arc::new(updated_workspace_entry));
        }

        {
            let item_store = self.global_storage.item_store();
            let segkey = WORKSPACE_SEGKEY.join(descriptor.name.to_owned());
            let value = AnyValue::serialize(&WorkspaceInfoEntity { last_opened_at })?;
            PutItem::put(item_store.as_ref(), segkey, value)?;
        }

        self.set_active_workspace(descriptor.id, workspace);

        Ok(OpenWorkspaceOutput {
            id: descriptor.id,
            abs_path: descriptor.abs_path.to_owned(),
        })
    }
}
