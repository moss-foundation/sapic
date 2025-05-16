use std::{path::Path, sync::Arc};

use anyhow::Context as _;
use chrono::Utc;
use moss_common::{
    api::{OperationError, OperationResult, OperationResultExt},
    models::primitives::Identifier,
};
use moss_db::primitives::AnyValue;
use moss_fs::utils::encode_name;
use moss_storage::{global_storage::entities::WorkspaceInfoEntity, storage::operations::PutItem};
use moss_workspace::Workspace;
use tauri::Runtime as TauriRuntime;
use validator::Validate;

use crate::{
    models::operations::{CreateWorkspaceInput, CreateWorkspaceOutput},
    storage::segments::WORKSPACE_SEGKEY,
    workbench::{Workbench, WorkspaceInfoEntry},
};

impl<R: TauriRuntime> Workbench<R> {
    pub async fn create_workspace(
        &self,
        input: &CreateWorkspaceInput,
    ) -> OperationResult<CreateWorkspaceOutput> {
        input.validate()?;

        let encoded_name = encode_name(&input.name);
        let abs_path: Arc<Path> = self.absolutize(&encoded_name).into();
        if abs_path.exists() {
            return Err(OperationError::AlreadyExists {
                name: input.name.clone(),
                path: abs_path.to_path_buf(),
            });
        }

        let workspaces = self
            .workspaces()
            .await
            .context("Failed to get known workspaces")
            .map_err_as_internal()?;

        self.fs
            .create_dir(&abs_path)
            .await
            .context("Failed to create workspace")
            .map_err_as_internal()?;

        let new_workspace = Workspace::new(
            self.app_handle.clone(),
            Arc::clone(&abs_path),
            Arc::clone(&self.fs),
            self.activity_indicator.clone(),
        )?;

        let last_opened_at = if input.open_on_creation {
            Some(Utc::now().timestamp())
        } else {
            None
        };

        let id = Identifier::new(&self.options.next_workspace_id);
        workspaces.write().await.insert(
            id,
            WorkspaceInfoEntry {
                id,
                name: encoded_name.to_owned(),
                display_name: input.name.to_owned(),
                last_opened_at,
                abs_path: Arc::clone(&abs_path),
            }
            .into(),
        );

        match (last_opened_at, input.open_on_creation) {
            (Some(last_opened_at), true) => {
                self.set_active_workspace(id, new_workspace);

                let item_store = self.global_storage.item_store();
                let segkey = WORKSPACE_SEGKEY.join(encoded_name);
                let value = AnyValue::serialize(&WorkspaceInfoEntity { last_opened_at })?;
                PutItem::put(item_store.as_ref(), segkey, value)?;
            }
            _ => {}
        }

        Ok(CreateWorkspaceOutput { id, abs_path })
    }
}
