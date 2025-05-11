pub mod collection_storage;
pub mod common;
pub mod global_storage;
pub mod workspace_storage;

use anyhow::Result;
use async_trait::async_trait;
use common::Transactional;
use std::{
    future::Future,
    path::{Path, PathBuf},
    pin::Pin,
    sync::Arc,
};

pub trait GlobalStorage: Transactional + Send + Sync {
    fn workspaces_store(&self) -> Arc<dyn global_storage::WorkspacesStore>;
}

pub trait WorkspaceStorage: Transactional + Send + Sync {
    fn collection_store(&self) -> Arc<dyn workspace_storage::CollectionStore>;
    fn environment_store(&self) -> Arc<dyn workspace_storage::EnvironmentStore>;
    fn state_store(&self) -> Arc<dyn workspace_storage::StateStore>;
}

#[async_trait]
pub trait ResettableStorage {
    async fn reset(
        &self,
        path: &Path,
        after_drop: Pin<Box<dyn Future<Output = Result<()>> + Send>>,
    ) -> Result<()>;
}

#[async_trait]
pub trait CollectionStorage: ResettableStorage + Transactional + Send + Sync {
    async fn request_store(&self) -> Arc<dyn collection_storage::RequestStore>;
    async fn state_store(&self) -> Arc<dyn collection_storage::StateStore>;
}
