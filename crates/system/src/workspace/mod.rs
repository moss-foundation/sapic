pub mod types;
pub mod workspace_edit_service;
pub mod workspace_service;

use async_trait::async_trait;
use sapic_base::workspace::types::primitives::WorkspaceId;
use std::path::PathBuf;

use crate::workspace::types::*;

#[async_trait]
pub trait WorkspaceServiceFs: Send + Sync {
    async fn lookup_workspaces(&self) -> joinerror::Result<Vec<DiscoveredWorkspace>>;
    async fn delete_workspace(&self, id: &WorkspaceId) -> joinerror::Result<Option<PathBuf>>;
}

pub struct WorkspaceEditParams {
    pub name: Option<String>,
}

#[async_trait]
pub trait WorkspaceEditBackend: Send + Sync {
    async fn edit(&self, id: &WorkspaceId, params: WorkspaceEditParams) -> joinerror::Result<()>;
}

#[async_trait]
pub trait WorkspaceEditOp: Send + Sync {
    async fn edit(&self, id: &WorkspaceId, params: WorkspaceEditParams) -> joinerror::Result<()>;
}
