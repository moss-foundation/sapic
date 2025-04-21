pub mod collection_storage;
pub mod common;
pub mod global_storage;
pub mod workspace_storage;

use anyhow::Result;
use async_trait::async_trait;
use common::Transactional;
use std::{future::Future, path::PathBuf, pin::Pin, sync::Arc};

use collection_storage::*;
use global_storage::*;
use workspace_storage::*;

pub trait GlobalStorage: Transactional + Send + Sync {
    fn workspaces_store(&self) -> Arc<dyn WorkspacesStore>;
}

pub trait WorkspaceStorage: Transactional + Send + Sync {
    fn collection_store(&self) -> Arc<dyn CollectionStore>;
    fn environment_store(&self) -> Arc<dyn EnvironmentStore>;
    fn state_store(&self) -> Arc<dyn StateStore>;
}

#[async_trait]
pub trait ResettableStorage {
    async fn reset(
        &self,
        path: PathBuf,
        after_drop: Pin<Box<dyn Future<Output = Result<()>> + Send>>,
    ) -> Result<()>;
}

#[async_trait]
pub trait CollectionStorage: ResettableStorage + Transactional + Send + Sync {
    async fn request_store(&self) -> Arc<dyn RequestStore>;
}
