use async_trait::async_trait;
use rustc_hash::FxHashMap;
use std::{path::Path, sync::Arc};
use tokio::sync::RwLock;

use crate::{
    adapters::StorageAdapter, application_storage::ApplicationStorageBackend,
    workspace_storage::WorkspaceStorageBackend,
};

#[async_trait]
pub trait StorageBackendProvider: Send + Sync {
    async fn application(&self) -> joinerror::Result<Arc<dyn StorageAdapter>>;
    async fn workspace(
        &self,
        workspace_id: Arc<String>,
    ) -> joinerror::Result<Option<Arc<dyn StorageAdapter>>>;
}

pub struct AppStorageBackendProvider {
    application: ApplicationStorageBackend,
    workspaces: RwLock<FxHashMap<Arc<String>, WorkspaceStorageBackend>>,
}

impl AppStorageBackendProvider {
    pub async fn new(globals_dir: &Path) -> joinerror::Result<Self> {
        let application = ApplicationStorageBackend::new(globals_dir).await?;

        Ok(Self {
            application,
            workspaces: RwLock::new(FxHashMap::default()),
        })
    }
}

#[async_trait]
impl StorageBackendProvider for AppStorageBackendProvider {
    async fn application(&self) -> joinerror::Result<Arc<dyn StorageAdapter>> {
        Ok(self.application.storage().await?)
    }

    async fn workspace(
        &self,
        workspace_id: Arc<String>,
    ) -> joinerror::Result<Option<Arc<dyn StorageAdapter>>> {
        let workspaces = self.workspaces.read().await;
        let workspace = workspaces.get(&workspace_id).cloned();

        if let Some(workspace) = workspace {
            return Ok(Some(workspace.storage().await?));
        } else {
            return Ok(None);
        }
    }
}
