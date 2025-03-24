use anyhow::{anyhow, Context, Result};
use arc_swap::ArcSwapOption;
use moss_app::service::prelude::AppService;
use moss_fs::{FileSystem, RemoveOptions, RenameOptions};
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
use crate::sanitizer::{decode_directory_name, encode_directory_name};
use crate::storage::global_db_manager::GlobalDbManagerImpl;
use crate::storage::{GlobalDbManager, WorkspaceEntity};

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

    #[error("workspace {name} not found at {path}")]
    NotFound { name: String, path: PathBuf },

    #[error("workspace {name} already exists at {path}")]
    AlreadyExists { name: String, path: PathBuf },

    #[error("unknown error: {0}")]
    Unknown(#[from] anyhow::Error),
}

type WorkspaceInfoMap = LeasedSlotMap<WorkspaceKey, WorkspaceInfo>;

pub struct WorkspaceManager {
    fs: Arc<dyn FileSystem>,
    // TODO: Should we allow creating workspaces at arbitrary locations?
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
                        Some(name) => decode_directory_name(&name.to_string_lossy().to_string())?,
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
    // TODO: (How) Should we write tests for this function?
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

        let full_path = self.workspaces_dir.join(encode_directory_name(&input.name));

        // Check if workspace already exists
        if full_path.exists() {
            return Err(OperationError::AlreadyExists {
                name: input.name,
                path: full_path,
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

        self.fs.create_dir(&full_path).await.context("Failed to create the workspace directory")?;

        let current_workspace = Workspace::new(full_path.clone(), self.fs.clone())?;
        let workspace_key = {
            let mut workspaces_lock = workspaces.write().await;
            workspaces_lock.insert(WorkspaceInfo {
                path: full_path.clone(),
                name: input.name,
            })
        };

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

        if workspace_info.name == input.new_name {
            return Ok(())
        }

        let old_path = workspace_info.path.clone();
        if !old_path.exists() {
            return Err(OperationError::NotFound {
                name: workspace_info.name.clone(),
                path: old_path,
            })
        }

        let new_path = old_path
            .parent()
            .context("Parent directory not found")?
            .join(encode_directory_name(&input.new_name));
        if new_path.exists() {
            return Err(OperationError::AlreadyExists {
                name: input.new_name,
                path: new_path,
            })
        }

        let workspace_store = self.global_db_manager()?.workspace_store();
        let (mut txn, table) = workspace_store.begin_write()?;

        let entity_key = old_path.to_string_lossy().to_string();
        let new_entity_key = new_path.to_string_lossy().to_string();

        let entity = table.remove(&mut txn, entity_key.clone())?;
        table.insert(
            &mut txn,
            new_entity_key,
            &entity
        )?;

        // An opened workspace db will prevent its parent folder from being renamed
        // If we are renaming the current workspace, we need to call the reset method

        let current_entry = self.current_workspace.swap(None);

        // FIXME: This is probably not the best approach
        // If the current workspace needs to be renamed
        // We will first drop the workspace, do fs renaming, and reload it
        if let Some(mut entry) = current_entry {
            if entry.0 == workspace_key {
                std::mem::drop(entry);
                self.fs.rename(&old_path, &new_path, RenameOptions::default()).await?;
                entry = Arc::new((workspace_key, Workspace::new(new_path.clone(), self.fs.clone())?))
            } else {
                self.fs.rename(&old_path, &new_path, RenameOptions::default()).await?;
            }
            self.current_workspace.store(Some(entry))
        } else {
            self.fs.rename(&old_path, &new_path, RenameOptions::default()).await?;
        }

        workspace_info.name = input.new_name;
        workspace_info.path = new_path;

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

        // TODO: logging if the folder has already been removed from the filesystem
        self.fs
            .remove_dir(
                &workspace_path,
                RemoveOptions {
                    recursive: true,
                    ignore_if_not_exists: true,
                },
            )
            .await?;

        // Deleting a workspace will remove it from current workspace if it is
        let current_entry = self.current_workspace.swap(None);

        if let Some(entry) = current_entry {
            if entry.0 != workspace_key {
                self.current_workspace.store(Some(entry))
            }
        }

        Ok(txn.commit()?)
    }

    pub async fn open_workspace(&self, input: OpenWorkspaceInput) -> Result<(), OperationError> {
        if !input.path.exists() {
            return Err(OperationError::NotFound {
                name: input.path.file_name().unwrap_or_default().to_string_lossy().to_string(),
                path: input.path.clone()
            });
        }

        // Check if the workspace is already active
        let current_workspace = self.current_workspace();
        if current_workspace.is_ok() && current_workspace.unwrap().1.path() == input.path {
            return Ok(())
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
            .next() {
            key
        } else {
            workspaces_lock.insert(WorkspaceInfo {
                name: decode_directory_name(&input.path.file_name().unwrap().to_string_lossy().to_string())
                    .map_err(|_| OperationError::Unknown(anyhow!("Invalid directory encoding")))?,
                path: input.path,
            })
        };

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

