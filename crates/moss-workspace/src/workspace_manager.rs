use anyhow::Result;
use arc_swap::ArcSwapOption;
use dashmap::DashMap;
use moss_app::service::prelude::AppService;
use moss_fs::ports::{FileSystem, RemoveOptions};
use std::{path::PathBuf, sync::Arc};
use tokio::sync::OnceCell;

use crate::{
    models::{
        operations::{
            CreateWorkspaceInput, DeleteWorkspaceInput, ListWorkspacesOutput, OpenWorkspaceInput,
        },
        types::WorkspaceInfo,
    },
    workspace::Workspace,
};

type WorkspaceInfoMap = DashMap<PathBuf, WorkspaceInfo>;

pub struct WorkspaceManager {
    fs: Arc<dyn FileSystem>,
    workspaces_dir: PathBuf,
    current_workspace: ArcSwapOption<Workspace>,
    known_workspaces: OnceCell<WorkspaceInfoMap>,
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

    async fn known_workspaces(&self) -> Result<&WorkspaceInfoMap> {
        Ok(self
            .known_workspaces
            .get_or_try_init(|| async move {
                let workspaces = DashMap::new();
                let mut dir = self.fs.read_dir(&self.workspaces_dir).await?;

                while let Some(entry) = dir.next_entry().await? {
                    let file_type = entry.file_type().await?;
                    if file_type.is_file() {
                        continue;
                    }

                    let path = entry.path();
                    let file_name_str = entry.file_name().to_string_lossy().to_string();
                    workspaces.insert(
                        path.clone(),
                        WorkspaceInfo {
                            path,
                            name: file_name_str,
                        },
                    );
                }

                Ok::<WorkspaceInfoMap, anyhow::Error>(workspaces)
            })
            .await?)
    }
}

impl WorkspaceManager {
    pub async fn create_workspace(&self, input: CreateWorkspaceInput) -> Result<()> {
        let path = self.workspaces_dir.join(input.name);
        self.fs.create_dir(&path).await?;

        let workspace = Workspace::new(path, self.fs.clone())?;

        // Automatically switch the workspace to the new one.
        self.current_workspace.store(Some(Arc::new(workspace)));

        Ok(())
    }

    pub async fn delete_workspace(&self, input: DeleteWorkspaceInput) -> Result<()> {
        let workspaces = self.known_workspaces().await?;

        self.fs
            .remove_dir(
                &input.path,
                RemoveOptions {
                    recursive: true,
                    ignore_if_not_exists: true,
                },
            )
            .await?;

        workspaces.remove(&input.path);

        Ok(())
    }

    pub async fn list_workspaces(&self) -> Result<ListWorkspacesOutput> {
        let workspaces = self.known_workspaces().await?;
        let content = workspaces.iter().map(|item| (*item).clone()).collect();

        Ok(ListWorkspacesOutput(content))
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
        let workspace_manager = WorkspaceManager::new(fs, dir.clone());

        let workspace_name = random_workspace_name();

        // Pre-create a workspace to test that it is listed.
        {
            workspace_manager
                .create_workspace(CreateWorkspaceInput {
                    name: workspace_name.clone(),
                })
                .await
                .unwrap();
        }

        let workspaces = workspace_manager.known_workspaces().await.unwrap();

        let target_workspace_path = dir.join(workspace_name);
        let found = workspaces
            .iter()
            .any(|entry| entry.key() == &target_workspace_path);

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
        workspace_manager
            .create_workspace(CreateWorkspaceInput {
                name: workspace_name,
            })
            .await
            .unwrap();

        // Delete workspace
        let result = workspace_manager
            .delete_workspace(DeleteWorkspaceInput {
                path: workspace_path.clone(),
            })
            .await;

        assert!(result.is_ok());

        // Verify workspace was deleted
        let workspaces = workspace_manager.known_workspaces().await.unwrap();
        assert!(!workspaces
            .iter()
            .any(|entry| entry.key() == &workspace_path));
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
