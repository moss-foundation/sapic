use async_trait::async_trait;
use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;
use rustc_hash::FxHashMap;
use std::{path::Path, sync::Arc};

use crate::{
    adapters::StorageAdapter, application_storage::ApplicationStorageBackend,
    workspace_storage::WorkspaceStorageBackend,
};

#[async_trait]
pub trait StorageBackendProvider: Send + Sync {
    async fn application(&self) -> joinerror::Result<Arc<dyn StorageAdapter>>;
    // async fn workspace(&self, workspace_id: Arc<String>) -> joinerror::Result<WorkspaceStorageBackend>;
}

pub struct AppStorageBackendProvider {
    application: ApplicationStorageBackend,
    // workspaces: FxHashMap<Arc<String>, WorkspaceStorageBackend>,
}

impl AppStorageBackendProvider {
    pub async fn new(globals_dir: &Path) -> joinerror::Result<Self> {
        let application = ApplicationStorageBackend::new(globals_dir).await?;

        Ok(Self { application })
    }
}

#[async_trait]
impl StorageBackendProvider for AppStorageBackendProvider {
    async fn application(&self) -> joinerror::Result<Arc<dyn StorageAdapter>> {
        Ok(self.application.storage().await?)
    }
}
