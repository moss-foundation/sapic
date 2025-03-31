use anyhow::{anyhow, Context, Result};
use arc_swap::ArcSwapOption;
use moss_app::service::prelude::AppService;
use moss_collection::leased_slotmap::LeasedSlotMap;
use moss_common::leased_slotmap::ResourceKey;
use moss_fs::utils::{decode_directory_name, encode_directory_name};
use moss_fs::{FileSystem, RemoveOptions, RenameOptions};
use std::{path::PathBuf, sync::Arc};
use thiserror::Error;
use tokio::sync::{OnceCell, RwLock};
use validator::{Validate, ValidationErrors};

use crate::models::operations::{CreateWorkspaceOutput, RenameWorkspaceInput};
use crate::{
    models::{
        operations::{
            CreateWorkspaceInput, DeleteWorkspaceInput, ListWorkspacesOutput, OpenWorkspaceInput,
        },
        types::WorkspaceInfo,
    },
    workspace::Workspace,
};

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

type WorkspaceInfoMap = LeasedSlotMap<ResourceKey, WorkspaceInfo>;

pub struct WorkspaceManager {
    fs: Arc<dyn FileSystem>,
    workspaces_dir: PathBuf,
    current_workspace: ArcSwapOption<(ResourceKey, Workspace)>,
    known_workspaces: OnceCell<RwLock<WorkspaceInfoMap>>,
}

impl WorkspaceManager {
    pub fn new(fs: Arc<dyn FileSystem>, workspaces_dir: PathBuf) -> Result<Self> {
        Ok(Self {
            fs,
            workspaces_dir,
            current_workspace: ArcSwapOption::new(None),
            known_workspaces: Default::default(),
        })
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
    // TODO: (How) Should we write tests for this function?
    pub async fn list_workspaces(&self) -> Result<ListWorkspacesOutput, OperationError> {
        let workspaces = self.known_workspaces().await?;
        let workspaces_lock = workspaces.read().await;

        Ok(ListWorkspacesOutput(
            workspaces_lock
                .iter()
                .filter(|(_, iter_slot)| !iter_slot.is_leased())
                .map(|(_, iter_slot)| iter_slot.value().clone())
                .collect(),
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

        let workspaces = self
            .known_workspaces()
            .await
            .context("Failed to get known workspaces")?;

        self.fs
            .create_dir(&full_path)
            .await
            .context("Failed to create the workspace directory")?;

        let current_workspace = Workspace::new(full_path.clone(), self.fs.clone())?;
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

    pub async fn rename_workspace(
        &self,
        input: RenameWorkspaceInput,
    ) -> Result<(), OperationError> {
        input.validate()?;

        let workspaces = self
            .known_workspaces()
            .await
            .context("Failed to get known workspaces")?;

        let mut workspaces_lock = workspaces.write().await;
        let mut workspace_info = workspaces_lock
            .read_mut(input.key)
            .context("Failed to lease the workspace")?;

        if workspace_info.name == input.new_name {
            return Ok(());
        }

        let old_path = workspace_info.path.clone();
        if !old_path.exists() {
            return Err(OperationError::NotFound {
                name: workspace_info.name.clone(),
                path: old_path,
            });
        }

        let new_path = old_path
            .parent()
            .context("Parent directory not found")?
            .join(encode_directory_name(&input.new_name));
        if new_path.exists() {
            return Err(OperationError::AlreadyExists {
                name: input.new_name,
                path: new_path,
            });
        }

        let entity_key = old_path.to_string_lossy().to_string();
        let new_entity_key = new_path.to_string_lossy().to_string();

        // An opened workspace db will prevent its parent folder from being renamed
        // If we are renaming the current workspace, we need to call the reset method

        let current_entry = self.current_workspace.swap(None);

        // FIXME: This is probably not the best approach
        // If the current workspace needs to be renamed
        // We will first drop the workspace, do fs renaming, and reload it
        if let Some(mut entry) = current_entry {
            if entry.0 == input.key {
                std::mem::drop(entry);
                self.fs
                    .rename(&old_path, &new_path, RenameOptions::default())
                    .await?;
                entry = Arc::new((
                    input.key,
                    Workspace::new(new_path.clone(), self.fs.clone())?,
                ))
            } else {
                self.fs
                    .rename(&old_path, &new_path, RenameOptions::default())
                    .await?;
            }
            self.current_workspace.store(Some(entry))
        } else {
            self.fs
                .rename(&old_path, &new_path, RenameOptions::default())
                .await?;
        }

        workspace_info.name = input.new_name;
        workspace_info.path = new_path;

        Ok(())
    }

    pub async fn delete_workspace(
        &self,
        input: DeleteWorkspaceInput,
    ) -> Result<(), OperationError> {
        let known_workspaces = self.known_workspaces().await?;

        let mut workspaces_lock = known_workspaces.write().await;
        let workspace_info = workspaces_lock
            .remove(input.key)
            .context("Failed to remove the workspace")?;

        let workspace_path = workspace_info.path;

        // TODO: If any of the following operations fail, we should place the task
        // in the dead queue and attempt the deletion later.

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
            if entry.0 != input.key {
                self.current_workspace.store(Some(entry))
            }
        }

        Ok(())
    }

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

    pub fn current_workspace(&self) -> Result<Arc<(ResourceKey, Workspace)>> {
        self.current_workspace
            .load()
            .clone()
            .ok_or(anyhow::anyhow!("Current workspace not set"))
    }
}

impl AppService for WorkspaceManager {}
