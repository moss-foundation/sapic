pub mod types;
pub mod workspace_edit_service;
pub mod workspace_service;

use async_trait::async_trait;
use sapic_base::workspace::types::primitives::WorkspaceId;
use std::path::PathBuf;

pub struct LookedUpWorkspace {
    pub id: WorkspaceId,
    pub name: String,
    pub abs_path: PathBuf,
}

#[async_trait]
pub trait WorkspaceServiceFs: Send + Sync {
    async fn lookup_workspaces(&self) -> joinerror::Result<Vec<LookedUpWorkspace>>;
    async fn create_workspace(&self, id: &WorkspaceId, name: &str) -> joinerror::Result<PathBuf>;
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

pub struct CreatedWorkspace {
    pub id: WorkspaceId,
    pub name: String,
    pub abs_path: PathBuf,
}

#[async_trait]
pub trait WorkspaceCreateOp: Send + Sync {
    async fn create(&self, name: String) -> joinerror::Result<CreatedWorkspace>;
}
