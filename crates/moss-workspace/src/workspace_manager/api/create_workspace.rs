use std::sync::Arc;

use anyhow::Context as _;
use moss_fs::utils::encode_directory_name;
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

        let full_path = self.workspaces_dir.join(encode_directory_name(&input.name));

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

        let current_workspace =
            Workspace::new(full_path.clone(), self.fs.clone(), self.app_handle.clone())?;
        let workspace_key = {
            let mut workspaces_lock = workspaces.write().await;
            workspaces_lock.insert(WorkspaceInfo {
                path: full_path.clone(),
                name: input.name,
            })
        };

        // // Automatically switch the workspace to the new one.
        self.current_workspace
            .store(Some(Arc::new((workspace_key, current_workspace))));

        Ok(CreateWorkspaceOutput { key: workspace_key })
    }
}
