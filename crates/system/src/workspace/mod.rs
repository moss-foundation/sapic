pub mod types;
pub mod workspace_edit_service;
pub mod workspace_service;

use async_trait::async_trait;
use moss_storage2::KvStorage;
use sapic_base::workspace::types::primitives::WorkspaceId;
use sapic_core::context::AnyAsyncContext;
use std::path::PathBuf;

pub struct LookedUpWorkspace {
    pub id: WorkspaceId,
    pub name: String,
    pub abs_path: PathBuf,
}

#[async_trait]
pub trait WorkspaceServiceFs: Send + Sync {
    async fn lookup_workspaces(
        &self,
        ctx: &dyn AnyAsyncContext,
    ) -> joinerror::Result<Vec<LookedUpWorkspace>>;
    async fn create_workspace(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &WorkspaceId,
        name: &str,
    ) -> joinerror::Result<PathBuf>;
    async fn delete_workspace(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &WorkspaceId,
    ) -> joinerror::Result<Option<PathBuf>>;
}

pub struct WorkspaceEditParams {
    pub name: Option<String>,
}

#[async_trait]
pub trait WorkspaceEditBackend: Send + Sync {
    async fn edit(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &WorkspaceId,
        params: WorkspaceEditParams,
    ) -> joinerror::Result<()>;
}

#[async_trait]
pub trait WorkspaceEditOp: Send + Sync {
    async fn edit(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &WorkspaceId,
        params: WorkspaceEditParams,
    ) -> joinerror::Result<()>;
}

pub struct CreatedWorkspace {
    pub id: WorkspaceId,
    pub name: String,
    pub abs_path: PathBuf,
}

#[async_trait]
pub trait WorkspaceCreateOp: Send + Sync {
    async fn create(
        &self,
        ctx: &dyn AnyAsyncContext,
        name: String,
    ) -> joinerror::Result<CreatedWorkspace>;
}
