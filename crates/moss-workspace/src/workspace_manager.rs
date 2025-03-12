use anyhow::Result;
use arc_swap::ArcSwapOption;
use dashmap::DashSet;
use moss_db::ReDbClient;
use std::{path::PathBuf, sync::Arc};

use crate::models::{operations::ListWorkspacesOutput, types::WorkspaceInfo};

const WORKSPACE_STATE_DB_NAME: &str = "state.db";

pub struct Workspace {
    db_client: ReDbClient,
}

impl Workspace {
    pub fn new(path: PathBuf) -> Result<Self> {
        let db_client = ReDbClient::new(path.join(WORKSPACE_STATE_DB_NAME))?;

        Ok(Self { db_client })
    }
}

pub struct WorkspaceManager {
    current_workspace: arc_swap::ArcSwapOption<Workspace>,
    known_workspaces: DashSet<WorkspaceInfo>,
}

impl WorkspaceManager {
    pub fn new() -> Self {
        Self {
            current_workspace: ArcSwapOption::new(None),
            known_workspaces: Default::default(),
        }
    }

    pub fn list_workspaces(&self) -> Result<ListWorkspacesOutput> {
        let w: Vec<WorkspaceInfo> = self.known_workspaces.iter().cloned().collect();
        Ok(ListWorkspacesOutput(w))
    }

    pub fn set_workspace(&self, path: PathBuf) -> Result<()> {
        let workspace = Workspace::new(path)?;
        self.current_workspace.store(Some(Arc::new(workspace)));
        Ok(())
    }
}
