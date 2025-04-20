pub mod collection_storage;
pub mod global_storage;
pub mod workspace_storage;

use anyhow::Result;
use async_trait::async_trait;
use std::{future::Future, path::PathBuf, pin::Pin, sync::Arc};

use collection_storage::*;
use global_storage::*;
use workspace_storage::*;

pub trait GlobalStorage: Send + Sync {
    fn workspaces_store(&self) -> Arc<dyn WorkspacesStore>;
}

pub trait WorkspaceStorage: Send + Sync {
    fn collection_store(&self) -> Arc<dyn CollectionStore>;
    fn environment_store(&self) -> Arc<dyn EnvironmentStore>;
    fn state_store(&self) -> Arc<dyn StateStore>;
}

#[async_trait]
pub trait CollectionStorage: Send + Sync {
    async fn reload(
        &self,
        path: PathBuf,
        after_drop: Pin<Box<dyn Future<Output = Result<()>> + Send>>,
    ) -> Result<()>;
    async fn request_store(&self) -> Arc<dyn RequestStore>;
}
