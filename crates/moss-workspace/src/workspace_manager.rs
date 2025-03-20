use anyhow::{Context, Result};
use arc_swap::ArcSwapOption;
use dashmap::DashMap;
use moss_app::service::AppService;
use moss_fs::ports::{FileSystem, RemoveOptions};
use std::{path::PathBuf, sync::Arc};
use slotmap::KeyData;
use thiserror::Error;
use tokio::sync::{OnceCell, RwLock};
use validator::{Validate, ValidationErrors};

use crate::{
    models::{
        operations::{
            CreateWorkspaceInput, DeleteWorkspaceInput, ListWorkspacesOutput, OpenWorkspaceInput,
        },
        types::WorkspaceInfo,
    },
    workspace::Workspace,
};
use crate::leased_slotmap::LeasedSlotMap;
use crate::models::operations::CreateWorkspaceOutput;
use crate::workspace::CollectionKey;

slotmap::new_key_type! {
    pub struct WorkspaceKey;
}

impl From<u64> for WorkspaceKey {
    fn from(value: u64) -> Self {
        Self(KeyData::from_ffi(value))
    }
}

impl WorkspaceKey {
    pub fn as_u64(self) -> u64 {
        self.0.as_ffi()
    }
}

impl std::fmt::Display for WorkspaceKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_u64())
    }
}

#[derive(Error, Debug)]
pub enum OperationError {
    #[error("validation error: {0}")]
    Validation(#[from] ValidationErrors),

    #[error("workspace {key} not found at {path}")]
    NotFound { key: PathBuf, path: PathBuf },

    #[error("workspace {key} already exists at {path}")]
    AlreadyExists { key: PathBuf, path: PathBuf },

    #[error("unknown error: {0}")]
    Unknown(#[from] anyhow::Error),
}

type WorkspaceInfoMap = LeasedSlotMap<WorkspaceKey, WorkspaceInfo>;

pub struct WorkspaceManager {
    fs: Arc<dyn FileSystem>,
    workspaces_dir: PathBuf,
    current_workspace: ArcSwapOption<Workspace>,
    known_workspaces: OnceCell<RwLock<WorkspaceInfoMap>>,
}

impl WorkspaceManager {
    pub fn new(fs: Arc<dyn FileSystem>, workspaces_dir: PathBuf) -> Self {
        Self {
            fs,
            workspaces_dir,
            current_workspace: ArcSwapOption::new(None),
            known_workspaces: Default::default(),
        }
    }

    async fn known_workspaces(&self) -> Result<&RwLock<WorkspaceInfoMap>> {
        Ok(self
            .known_workspaces
            .get_or_try_init(|| async move {
                let mut workspaces = LeasedSlotMap::new();
                let mut dir = self.fs.read_dir(&self.workspaces_dir).await?;

                while let Some(entry) = dir.next_entry().await? {
                    let file_type = entry.file_type().await?;
                    if file_type.is_file() {
                        continue;
                    }

                    let path = entry.path();
                    let file_name_str = entry.file_name().to_string_lossy().to_string();
                    workspaces.insert(WorkspaceInfo {
                        path,
                        name: file_name_str,
                    });
                }

                Ok::<_, anyhow::Error>(RwLock::new(workspaces))
            })
            .await?)
    }
}

impl WorkspaceManager {
    pub async fn create_workspace(
        &self,
        input: CreateWorkspaceInput,
    ) -> Result<CreateWorkspaceOutput, OperationError> {
        input.validate()?;

        let path = self.workspaces_dir.join(&input.name);

        // Check if workspace already exists
        if path.exists() {
            return Err(OperationError::AlreadyExists {
                key: PathBuf::from(&input.name),
                path: path.clone(),
            });
        }

        self.fs
            .create_dir(&path)
            .await
            .map_err(|e| OperationError::Unknown(e))?;

        let workspace = Workspace::new(path.clone(), self.fs.clone())
            .map_err(|e| OperationError::Unknown(e))?;

        // Automatically switch the workspace to the new one.
        self.current_workspace.store(Some(Arc::new(workspace)));

        let workspaces = self.known_workspaces().await.context("Failed to get known workspaces")?;
        let key = {
            let mut workspaces_lock = workspaces.write().await;
            workspaces_lock.insert(WorkspaceInfo {
                path,
                name: input.name,
            })
        };
        Ok(CreateWorkspaceOutput {key: key.as_u64()})
    }

    pub async fn delete_workspace(&self, input: DeleteWorkspaceInput) -> Result<()> {
        let workspaces = self.known_workspaces().await?;
        let workspace_key = WorkspaceKey::from(input.key);

        let mut workspaces_lock = workspaces.write().await;
        let workspace_info = workspaces_lock.remove(workspace_key).context("Failed to remove the workspace")?;

        let workspace_path = workspace_info.path;

        self.fs
            .remove_dir(
                &workspace_path,
                RemoveOptions {
                    recursive: true,
                    ignore_if_not_exists: true,
                },
            )
            .await?;

        Ok(())
    }

    pub async fn list_workspaces(&self) -> Result<ListWorkspacesOutput, OperationError> {
        let workspaces = self.known_workspaces().await?;
        let workspaces_lock = workspaces.read().await;

        Ok(ListWorkspacesOutput(
            workspaces_lock
                .iter()
                .filter(|(_, iter_slot)| !iter_slot.is_leased())
                .map(|(_, iter_slot)| {
                    iter_slot.value().clone()
                })
                .collect()
        ))
    }

    pub fn open_workspace(&self, input: OpenWorkspaceInput) -> Result<()> {
        let workspace = Workspace::new(input.path, self.fs.clone())?;
        self.current_workspace.store(Some(Arc::new(workspace)));
        Ok(())
    }

    pub fn current_workspace(&self) -> Result<Arc<Workspace>> {
        self.current_workspace
            .load()
            .clone()
            .ok_or(anyhow::anyhow!("Current workspace not set"))
    }
}

impl AppService for WorkspaceManager {
    fn name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }

    fn dispose(&self) {}

    fn as_any(&self) -> &(dyn std::any::Any + Send) {
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::random_workspace_name;
    use moss_fs::adapters::disk::DiskFileSystem;

    use super::*;

    #[tokio::test]
    async fn list_workspaces() {
        let fs = Arc::new(DiskFileSystem::new());
        let dir: PathBuf =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../samples/workspaces");
        let workspace_manager = WorkspaceManager::new(fs, dir.clone());

        let workspace_name = random_workspace_name();

        // Pre-create a workspace to test that it is listed.
        let workspace_key = workspace_manager
            .create_workspace(CreateWorkspaceInput {
                name: workspace_name.clone(),
            })
            .await
            .unwrap().key;


        let workspaces = workspace_manager.known_workspaces().await.unwrap();
        let workspaces_lock = workspaces.read().await;
        let target_workspace_path = dir.join(workspace_name);

        let found = workspaces_lock
            .iter()
            .any(|(key, info)| key.as_u64() == workspace_key);

        assert!(
            found,
            "Created workspace {} not found in list of workspaces",
            target_workspace_path.display()
        );

        // Clean up
        {
            std::fs::remove_dir_all(target_workspace_path).unwrap();
        }
    }

    #[tokio::test]
    async fn delete_workspace() {
        let fs = Arc::new(DiskFileSystem::new());
        let dir: PathBuf =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../samples/workspaces");
        let workspace_manager = WorkspaceManager::new(fs, dir.clone());

        let workspace_name = random_workspace_name();
        let workspace_path = dir.join(&workspace_name);

        // Create workspace first
        let create_workspace_output = workspace_manager
            .create_workspace(CreateWorkspaceInput {
                name: workspace_name,
            })
            .await
            .unwrap();

        let workspace_key = create_workspace_output.key;
        // Delete workspace
        let result = workspace_manager
            .delete_workspace(DeleteWorkspaceInput {
                key: workspace_key,
            })
            .await;

        assert!(result.is_ok());

        // Verify workspace was deleted
        let workspaces = workspace_manager.known_workspaces().await.unwrap();
        let workspaces_lock = workspaces.read().await;
        assert!(!workspaces_lock
            .iter()
            .any(|(key, info)| key.as_u64() == workspace_key));
    }

    #[tokio::test]
    async fn open_workspace() {
        let fs = Arc::new(DiskFileSystem::new());
        let dir: PathBuf =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../samples/workspaces");
        let workspace_manager = WorkspaceManager::new(fs, dir.clone());

        let workspace_name = random_workspace_name();
        let workspace_path = dir.join(&workspace_name);

        // Create workspace first
        workspace_manager
            .create_workspace(CreateWorkspaceInput {
                name: workspace_name.clone(),
            })
            .await
            .unwrap();

        // Verify current workspace is set
        let current = workspace_manager.current_workspace();
        assert!(current.is_ok());

        // Clean up
        {
            std::fs::remove_dir_all(workspace_path).unwrap();
        }
    }
}
