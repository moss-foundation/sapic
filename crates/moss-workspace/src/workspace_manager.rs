use anyhow::Result;
use arc_swap::ArcSwapOption;
use dashmap::DashSet;
use moss_fs::ports::FileSystem;
use std::{path::PathBuf, sync::Arc};
use tokio::sync::OnceCell;

use crate::{
    models::{
        operations::{ListWorkspacesOutput, SetWorkspaceInput},
        types::WorkspaceInfo,
    },
    workspace::Workspace,
};

pub struct WorkspaceManager {
    fs: Arc<dyn FileSystem>,
    workspaces_dir: PathBuf,
    current_workspace: ArcSwapOption<Workspace>,
    known_workspaces: OnceCell<DashSet<WorkspaceInfo>>,
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

    async fn known_workspaces(&self) -> Result<&DashSet<WorkspaceInfo>> {
        Ok(self
            .known_workspaces
            .get_or_try_init(|| async move {
                let workspaces = DashSet::new();
                let mut dir = self.fs.read_dir(&self.workspaces_dir).await?;

                while let Some(entry) = dir.next_entry().await? {
                    let file_type = entry.file_type().await?;
                    if file_type.is_file() {
                        continue;
                    }

                    let file_name_str = entry.file_name().to_string_lossy().to_string();
                    workspaces.insert(WorkspaceInfo {
                        path: entry.path(),
                        name: file_name_str,
                    });
                }

                Ok::<DashSet<WorkspaceInfo>, anyhow::Error>(workspaces)
            })
            .await?)
    }
}

impl WorkspaceManager {
    pub async fn list_workspaces(&self) -> Result<ListWorkspacesOutput> {
        let workspaces = self.known_workspaces().await?;
        let content = workspaces.iter().map(|item| (*item).clone()).collect();

        Ok(ListWorkspacesOutput(content))
    }

    pub fn set_workspace(&self, input: SetWorkspaceInput) -> Result<()> {
        let workspace = Workspace::new(input.path)?;
        self.current_workspace.store(Some(Arc::new(workspace)));
        Ok(())
    }
}
