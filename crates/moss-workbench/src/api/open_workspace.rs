use chrono::Utc;
use moss_applib::context::Context;
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
    pub async fn open_workspace<C: Context<R>>(
        &self,
        ctx: &C,
        input: &OpenWorkspaceInput,
    ) -> OperationResult<OpenWorkspaceOutput> {
        let workspaces = self.workspaces(ctx).await?;
        let descriptor = if let Some(d) = workspaces.read().await.get(&input.id) {
            Arc::clone(d)
        } else {
            return Err(OperationError::NotFound(format!(
                "workspace with name {}",
                input.id
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

        let workspace =
            Workspace::load(ctx, &descriptor.abs_path, self.activity_indicator.clone()).await?;

        let last_opened_at = Utc::now().timestamp();

        {
            let updated_workspace_entry = WorkspaceDescriptor {
                id: descriptor.id,
                name: descriptor.name.to_owned(),
                abs_path: Arc::clone(&descriptor.abs_path),
                last_opened_at: Some(last_opened_at),
            };

            let known_workspaces = self.workspaces(ctx).await?;
            known_workspaces
                .write()
                .await
                .insert(descriptor.id, Arc::new(updated_workspace_entry));
        }

        {
            let id_str = descriptor.id.to_string();
            let item_store = self.global_storage.item_store();
            let segkey = WORKSPACE_SEGKEY.join(id_str);
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
