use chrono::Utc;
use moss_applib::context::Context;
use moss_common::api::{OperationError, OperationOptionExt, OperationResult};
use moss_db::primitives::AnyValue;
use moss_storage::{global_storage::entities::WorkspaceInfoEntity, storage::operations::PutItem};
use moss_workspace::Workspace;
use std::sync::Arc;
use tauri::Runtime as TauriRuntime;

use crate::{
    app::App,
    models::operations::{OpenWorkspaceInput, OpenWorkspaceOutput},
    services::workspace_service::{WorkspaceDescriptor, WorkspaceService},
    storage::segments::WORKSPACE_SEGKEY,
};

impl<R: TauriRuntime> App<R> {
    pub async fn open_workspace<C: Context<R>>(
        &self,
        _ctx: &C,
        input: &OpenWorkspaceInput,
    ) -> OperationResult<OpenWorkspaceOutput> {
        let workspace_service = self.service::<WorkspaceService<R>>();
        let workspaces = workspace_service.workspaces().await?;
        let descriptor = workspaces
            .read()
            .await
            .get(&input.id)
            .map_err_as_not_found(format!("workspace with name {}", input.id))?
            .clone();

        if !descriptor.abs_path.exists() {
            return Err(OperationError::NotFound(
                descriptor.abs_path.to_string_lossy().to_string(),
            ));
        }

        if let Some(active_workspace_id) = workspace_service.active_workspace_id().await {
            if active_workspace_id == descriptor.id {
                return Ok(OpenWorkspaceOutput {
                    id: descriptor.id,
                    abs_path: Arc::clone(&descriptor.abs_path),
                });
            }
        }

        let workspace = Workspace::load(
            self.fs.clone(),
            &descriptor.abs_path,
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

            workspaces
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

        workspace_service
            .activate_workspace(descriptor.id, workspace)
            .await;

        Ok(OpenWorkspaceOutput {
            id: descriptor.id,
            abs_path: descriptor.abs_path.to_owned(),
        })
    }
}
