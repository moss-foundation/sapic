use anyhow::anyhow;
use moss_fs::utils::decode_directory_name;
use std::sync::Arc;

use crate::{
    models::{operations::OpenWorkspaceInput, types::WorkspaceInfo},
    workspace::Workspace,
    workspace_manager::{OperationError, WorkspaceManager},
};

impl WorkspaceManager {
    pub async fn open_workspace(&self, input: OpenWorkspaceInput) -> Result<(), OperationError> {
        if !input.path.exists() {
            return Err(OperationError::NotFound {
                name: input
                    .path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string(),
                path: input.path.clone(),
            });
        }

        // Check if the workspace is already active
        let current_workspace = self.current_workspace();
        if current_workspace.is_ok() && current_workspace.unwrap().1.path() == input.path {
            return Ok(());
        }

        let workspace = Workspace::new(input.path.clone(), self.fs.clone())?;

        let known_workspaces = self.known_workspaces().await?;
        let mut workspaces_lock = known_workspaces.write().await;

        // FIXME: Maybe the process can be improved
        // Find the key for the workspace to be opened
        // If not found, add the workspace to the known workspaces
        // This would allow for opening a workspace in a non-default folder
        let workspace_key = if let Some((key, _)) = workspaces_lock
            .iter()
            .filter(|(_, info)| &info.value().path == &input.path)
            .next()
        {
            key
        } else {
            workspaces_lock.insert(WorkspaceInfo {
                name: decode_directory_name(
                    &input
                        .path
                        .file_name()
                        .unwrap()
                        .to_string_lossy()
                        .to_string(),
                )
                .map_err(|_| OperationError::Unknown(anyhow!("Invalid directory encoding")))?,
                path: input.path,
            })
        };

        self.current_workspace
            .store(Some(Arc::new((workspace_key, workspace))));
        Ok(())
    }
}
