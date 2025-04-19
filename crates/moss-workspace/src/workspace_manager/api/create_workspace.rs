use std::sync::Arc;

use anyhow::Context as _;
use chrono::Utc;
use moss_fs::utils::encode_name;
use moss_storage::{global_storage::entities::WorkspaceInfoEntity, workspace_storage};
use tauri::Runtime as TauriRuntime;
use validator::Validate;

use crate::{
    models::{
        operations::{CreateWorkspaceInput, CreateWorkspaceOutput},
        types::WorkspaceInfo,
    },
    workspace::Workspace,
    workspace_manager::{OperationError, WorkspaceManager},
};

impl<R: TauriRuntime> WorkspaceManager<R> {
    pub async fn create_workspace(
        &self,
        input: CreateWorkspaceInput,
    ) -> Result<CreateWorkspaceOutput, OperationError> {
        input.validate()?;

        let encoded_name = encode_name(&input.name);
        let full_path = self.workspaces_dir.join(&encoded_name);

        // Check if workspace already exists
        if full_path.exists() {
            return Err(OperationError::AlreadyExists {
                name: input.name,
                path: full_path,
            });
        }

        let workspaces = self
            .known_workspaces()
            .await
            .context("Failed to get known workspaces")?;

        self.fs
            .create_dir(&full_path)
            .await
            .context("Failed to create the workspace directory")?;

        let current_workspace = Workspace::new(
            self.app_handle.clone(),
            full_path.clone(),
            self.fs.clone(),
            self.activity_indicator.clone(),
        )?;
        let last_opened_at = Utc::now().timestamp();

        let workspace_storage = self.global_storage.workspaces_store();
        workspace_storage.set_workspace(encoded_name, WorkspaceInfoEntity { last_opened_at })?;

        let workspace_key = {
            let mut workspaces_lock = workspaces.write().await;
            workspaces_lock.insert(WorkspaceInfo {
                path: full_path.clone(),
                name: input.name,
                last_opened_at: Some(last_opened_at),
            })
        };

        // Automatically switch the workspace to the new one.
        self.current_workspace
            .store(Some(Arc::new((workspace_key, current_workspace))));

        Ok(CreateWorkspaceOutput {
            key: workspace_key,
            path: full_path,
        })
    }
}
