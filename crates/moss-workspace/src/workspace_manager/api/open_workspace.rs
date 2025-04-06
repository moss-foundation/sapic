use anyhow::anyhow;
use moss_fs::utils::{decode_directory_name, encode_directory_name};
use std::sync::Arc;
use tauri::Runtime as TauriRuntime;

use crate::{
    models::{operations::OpenWorkspaceInput, types::WorkspaceInfo},
    workspace::Workspace,
    workspace_manager::{OperationError, WorkspaceManager},
};

impl<R: TauriRuntime> WorkspaceManager<R> {
    pub async fn open_workspace(&self, input: &OpenWorkspaceInput) -> Result<(), OperationError> {
        let encoded_dir_name = encode_directory_name(&input.name);
        let full_path = self.workspaces_dir.join(&encoded_dir_name);

        if !full_path.exists() {
            return Err(OperationError::NotFound {
                name: encoded_dir_name,
                path: full_path,
            });
        }

        // Check if the workspace is already active
        if let Ok(current_workspace) = self.current_workspace() {
            if current_workspace.1.path() == full_path {
                return Ok(());
            }
        }

        let workspace = Workspace::new(
            full_path.clone(),
            self.fs.clone(),
            self.app_handle.clone(),
            self.activity_indicator.clone(),
        )?;

        let known_workspaces = self.known_workspaces().await?;
        let mut workspaces_lock = known_workspaces.write().await;

        // FIXME: Maybe the process can be improved
        // Find the key for the workspace to be opened
        // If not found, add the workspace to the known workspaces
        // This would allow for opening a workspace in a non-default folder
        let workspace_key = if let Some((key, _)) = workspaces_lock
            .iter()
            .filter(|(_, info)| &info.value().path == &full_path)
            .next()
        {
            key
        } else {
            workspaces_lock.insert(WorkspaceInfo {
                name: decode_directory_name(
                    &full_path.file_name().unwrap().to_string_lossy().to_string(),
                )
                .map_err(|_| OperationError::Unknown(anyhow!("Invalid directory encoding")))?,
                path: full_path,
            })
        };

        self.current_workspace
            .store(Some(Arc::new((workspace_key, workspace))));
        Ok(())
    }
}
