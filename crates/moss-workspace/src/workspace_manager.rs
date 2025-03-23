use anyhow::{anyhow, Context, Result};
use arc_swap::ArcSwapOption;
use dashmap::DashMap;
use moss_app::service::prelude::AppService;
use moss_fs::ports::{FileSystem, RemoveOptions};
use std::{path::PathBuf, sync::Arc};
use arc_swap::access::Access;
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
use crate::models::operations::{CreateWorkspaceOutput, RenameWorkspaceInput};
use crate::storage::global_db_manager::GlobalDbManagerImpl;
use crate::storage::{GlobalDbManager, WorkspaceEntity};
use crate::storage::state_db_manager::StateDbManagerImpl;
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
    global_db_manager: Option<Arc<dyn GlobalDbManager>>,
    current_workspace: ArcSwapOption<(WorkspaceKey, Workspace)>,
    known_workspaces: OnceCell<RwLock<WorkspaceInfoMap>>,
}

impl WorkspaceManager {
    pub fn new(fs: Arc<dyn FileSystem>, workspaces_dir: PathBuf) -> Result<Self> {
        let global_db_manager = GlobalDbManagerImpl::new(&workspaces_dir).context("Failed to open the global state database")?;


        Ok(Self {
            fs,
            workspaces_dir,
            global_db_manager: Some(Arc::new(global_db_manager)),
            current_workspace: ArcSwapOption::new(None),
            known_workspaces: Default::default(),
        })
    }

    pub fn global_db_manager(&self) -> Result<Arc<dyn GlobalDbManager>> {
        self.global_db_manager.clone().ok_or(anyhow!("The global_db_manager has been dropped"))
    }

    async fn known_workspaces(&self) -> Result<&RwLock<WorkspaceInfoMap>> {
        let result = self
            .known_workspaces
            .get_or_try_init(|| async move {
                let mut workspaces = LeasedSlotMap::new();

                for (workspace_path, _workspace_data) in
                    self.global_db_manager()?.workspace_store().scan()? {
                    let name = match workspace_path.file_name() {
                        Some(name) => name.to_string_lossy().to_string(),
                        None => {
                            // TODO: logging
                            println!("failed to get the workspace {:?} name", workspace_path);
                            continue;
                        }
                    };

                    // TODO:A self-healing mechanism needs to be implemented here similar to workspace.rs
                    let workspace_info = WorkspaceInfo {
                        name,
                        path: workspace_path
                    };
                    workspaces.insert(workspace_info);

                }

                Ok::<_, anyhow::Error>(RwLock::new(workspaces))
            })
            .await?;
        Ok(result)
    }
}

impl WorkspaceManager {
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
    pub async fn create_workspace(
        &self,
        input: CreateWorkspaceInput,
    ) -> Result<CreateWorkspaceOutput, OperationError> {
        input.validate()?;

        let full_path = self.workspaces_dir.join(&input.name);

        // Check if workspace already exists
        if full_path.exists() {
            return Err(OperationError::AlreadyExists {
                key: PathBuf::from(&input.name),
                path: full_path.clone(),
            });
        }

        let workspaces = self.known_workspaces().await.context("Failed to get known workspaces")?;

        let workspace_store = self.global_db_manager()?.workspace_store();
        let (mut txn, table) = workspace_store.begin_write()?;

        table.insert(
            &mut txn,
            full_path.to_string_lossy().to_string(),
            &WorkspaceEntity {}
        )?;

        self.fs.create_dir(&full_path).await.map_err(OperationError::Unknown)?;

        let workspace_key = {
            let mut workspaces_lock = workspaces.write().await;
            workspaces_lock.insert(WorkspaceInfo {
                path: full_path.clone(),
                name: input.name,
            })
        };

        let current_workspace = Workspace::new(full_path.clone(), self.fs.clone())?;

        // // Automatically switch the workspace to the new one.
        self.current_workspace.store(Some(Arc::new((workspace_key, current_workspace))));

        txn.commit()?;

        Ok(CreateWorkspaceOutput {
            key: workspace_key.as_u64()
        })

    }

    pub async fn rename_workspace(&self, input: RenameWorkspaceInput) -> Result<(), OperationError> {
        input.validate()?;

        let workspaces = self
            .known_workspaces()
            .await
            .context("Failed to get known workspaces")?;

        let workspace_key = WorkspaceKey::from(input.key);
        let mut workspaces_lock = workspaces.write().await;
        let mut workspace_info = workspaces_lock.read_mut(workspace_key).context("Failed to lease the workspace")?;

        workspace_info.name = input.new_name.clone();

        let old_path = workspace_info.path.clone();
        let new_path = old_path.parent().context("Parent directory not found")?.join(&input.new_name);

        workspace_info.path = new_path.clone();

        let workspace_store = self.global_db_manager()?.workspace_store();
        let (mut txn, table) = workspace_store.begin_write()?;

        let entity_key = old_path.to_string_lossy().to_string();

        table.remove(&mut txn, entity_key.clone())?;
        table.insert(
            &mut txn,
            entity_key,
            &WorkspaceEntity {}
        )?;

        // An opened workspace db will prevent its parent folder from being renamed
        // If we are renaming the current workspace, we need to call the reset method

        let current_entry = self.current_workspace.swap(None);

        // FIXME: This is probably not the best approach
        if let Some(mut entry) = current_entry {
            if entry.0 == workspace_key {
                Arc::get_mut(&mut entry)
                    .unwrap()
                    .1
                    .reset(new_path.clone()).await?;
            } else {
                self.fs.rename(&old_path, &new_path, RenameOptions::default()).await?;
            }
            self.current_workspace.store(Some(entry))
        } else {
            self.fs.rename(&old_path, &new_path, RenameOptions::default()).await?;
        }

        Ok(txn.commit()?)
    }

    pub async fn delete_workspace(&self, input: DeleteWorkspaceInput) -> Result<(), OperationError> {
        let known_workspaces = self.known_workspaces().await?;
        let workspace_key = WorkspaceKey::from(input.key);

        let mut workspaces_lock = known_workspaces.write().await;
        let workspace_info = workspaces_lock.remove(workspace_key).context("Failed to remove the workspace")?;

        let workspace_path = workspace_info.path;
        let workspace_store = self.global_db_manager()?.workspace_store();

        // TODO: If any of the following operations fail, we should place the task
        // in the dead queue and attempt the deletion later.

        let (mut txn, table) = workspace_store.begin_write()?;
        table.remove(&mut txn, workspace_path.to_string_lossy().to_string())?;

        self.fs
            .remove_dir(
                &workspace_path,
                RemoveOptions {
                    recursive: true,
                    ignore_if_not_exists: true,
                },
            )
            .await?;

        Ok(txn.commit()?)
    }



    pub async fn open_workspace(&self, input: OpenWorkspaceInput) -> Result<(), OperationError> {
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
            .next() {
            key
        } else {
            workspaces_lock.insert(WorkspaceInfo {
                name: input.path.file_name().unwrap().to_string_lossy().to_string(),
                path: input.path,
            })
        };

        // get the key for the
        self.current_workspace.store(Some(Arc::new((workspace_key, workspace))));
        Ok(())
    }

    pub fn current_workspace(&self) -> Result<Arc<(WorkspaceKey, Workspace)>> {
        self.current_workspace
            .load()
            .clone()
            .ok_or(anyhow::anyhow!("Current workspace not set"))
    }
}

impl AppService for WorkspaceManager {}

#[cfg(test)]
mod tests {
    use crate::utils::random_workspace_name;
    use moss_fs::adapters::disk::DiskFileSystem;

    use super::*;

    #[tokio::test]
    async fn create_workspace() {
        let fs = Arc::new(DiskFileSystem::new());
        let dir: PathBuf =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../samples/workspaces");
        let workspace_manager = WorkspaceManager::new(fs, dir.clone());

        let workspace_name = random_workspace_name();
        let result = workspace_manager
            .create_workspace(CreateWorkspaceInput {
                name: workspace_name.clone(),
            })
            .await;

        assert!(result.is_ok());

        // Clean up
        {
            std::fs::remove_dir_all(dir.join(workspace_name)).unwrap();
        }
    }

    #[tokio::test]
    async fn list_workspaces() {
        let fs = Arc::new(DiskFileSystem::new());
        let dir: PathBuf =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../samples/workspaces");
        let workspace_manager = WorkspaceManager::new(fs, dir.clone()).unwrap();

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
        let workspace_manager = WorkspaceManager::new(fs, dir.clone()).unwrap();

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
        let workspace_manager = WorkspaceManager::new(fs, dir.clone()).unwrap();

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

    #[tokio::test]
    async fn rename_workspace() {
        let fs = Arc::new(DiskFileSystem::new());
        let dir: PathBuf =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../samples/workspaces");
        let workspace_manager = WorkspaceManager::new(fs, dir.clone()).unwrap();

        let old_workspace_name = random_workspace_name();
        let old_workspace_path = dir.join(&old_workspace_name);
        let new_workspace_name = "New Workspace".to_string();
        let new_workspace_path = dir.join(&new_workspace_name);
        // Pre-create a workspace to test that it is listed.
        let workspace_key = workspace_manager
            .create_workspace(CreateWorkspaceInput {
                name: old_workspace_name.clone(),
            })
            .await
            .unwrap().key;


        let result = workspace_manager.rename_workspace(
            RenameWorkspaceInput {
                key: workspace_key,
                new_name: new_workspace_name.clone(),
            }
        ).await;

        assert!(result.is_ok());

        // Verify old workspace name does not exist in WorkspaceInfoMap
        let workspaces = workspace_manager.known_workspaces().await.unwrap();
        let workspaces_lock = workspaces.read().await;
        assert!(
            !workspaces_lock.iter().any(|(key, info)| info.value().name == old_workspace_name)
        );
        assert!(
            workspaces_lock.iter().any(|(key, info)| info.value().name == new_workspace_name)
        );

        // Check the currently active workspace's path is updated
        assert_eq!(workspace_manager.current_workspace().unwrap().1.path(), new_workspace_path);

        {
            std::fs::remove_dir_all(new_workspace_path).unwrap();
        }
    }
}
