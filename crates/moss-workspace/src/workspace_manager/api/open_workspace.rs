use anyhow::anyhow;
use chrono::Utc;
use moss_common::api::{OperationError, OperationResult};
use moss_fs::utils::{decode_name, encode_name};
use moss_storage::global_storage::entities::WorkspaceInfoEntity;
use std::sync::Arc;
use tauri::Runtime as TauriRuntime;

use crate::{
    models::{
        operations::{OpenWorkspaceInput, OpenWorkspaceOutput},
        types::WorkspaceInfo,
    },
    workspace::Workspace,
    workspace_manager::WorkspaceManager,
};

impl<R: TauriRuntime> WorkspaceManager<R> {
    pub async fn open_workspace(
        &self,
        input: &OpenWorkspaceInput,
    ) -> OperationResult<OpenWorkspaceOutput> {
        let encoded_name = encode_name(&input.name);
        let full_path = self.workspaces_dir.join(&encoded_name);

        if !full_path.exists() {
            return Err(OperationError::NotFound {
                name: encoded_name,
                path: full_path,
            });
        }

        // Check if the workspace is already active
        if let Ok(current_workspace) = self.current_workspace() {
            if current_workspace.1.path() == full_path {
                return Ok(OpenWorkspaceOutput { path: full_path });
            }
        }

        let workspace = Workspace::new(
            self.app_handle.clone(),
            full_path.clone(),
            self.fs.clone(),
            self.activity_indicator.clone(),
        )?;

        let known_workspaces = self.known_workspaces().await?;
        let mut workspaces_lock = known_workspaces.write().await;

        let last_opened_at = Utc::now().timestamp();
        let workspace_key = if let Some((key, _)) = workspaces_lock
            .iter()
            .filter(|(_, info)| &info.value().path == &full_path)
            .next()
        {
            key
        } else {
            // INFO: This is an anomaly, the workspace should be already known, since
            // we traverse the workspaces directory when opening the app.

            workspaces_lock.insert(WorkspaceInfo {
                name: decode_name(&full_path.file_name().unwrap().to_string_lossy().to_string())
                    .map_err(|_| OperationError::Unknown(anyhow!("Invalid directory encoding")))?,
                path: full_path.clone(),
                last_opened_at: Some(last_opened_at),
            })
        };

        let workspace_storage = self.global_storage.workspaces_store();
        workspace_storage.set_workspace(encoded_name, WorkspaceInfoEntity { last_opened_at })?;

        self.current_workspace
            .store(Some(Arc::new((workspace_key, workspace))));

        Ok(OpenWorkspaceOutput { path: full_path })
    }
}
