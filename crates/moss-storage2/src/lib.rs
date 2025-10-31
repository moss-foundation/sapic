pub mod adapters;
pub mod application_storage;
pub mod models;
pub mod workspace_storage;

use async_trait::async_trait;
use derive_more::Deref;
use joinerror::OptionExt;
use moss_app_delegate::AppDelegate;
use moss_applib::{AppRuntime, subscription::EventEmitter};
use rustc_hash::FxHashMap;
use serde_json::Value as JsonValue;
use std::{path::Path, sync::Arc};
use tokio::sync::RwLock;

use crate::{
    adapters::KeyedStorage,
    application_storage::ApplicationStorageBackend,
    models::{events::OnDidChangeValueEvent, primitives::StorageScope},
    workspace_storage::WorkspaceStorageBackend,
};

#[async_trait]
pub trait Storage: Send + Sync {
    async fn put(&self, scope: StorageScope, key: &str, value: JsonValue) -> joinerror::Result<()>;
    async fn get(&self, scope: StorageScope, key: &str) -> joinerror::Result<Option<JsonValue>>;
    async fn remove(&self, scope: StorageScope, key: &str) -> joinerror::Result<()>;
}

pub struct AppStorage {
    application: ApplicationStorageBackend,
    workspaces: RwLock<FxHashMap<Arc<String>, WorkspaceStorageBackend>>,

    on_did_change_value_emitter: EventEmitter<OnDidChangeValueEvent>,
}

#[async_trait]
impl Storage for AppStorage {
    async fn put(&self, scope: StorageScope, key: &str, value: JsonValue) -> joinerror::Result<()> {
        match scope.clone() {
            StorageScope::Application => self.application().await?.put(key, value).await,
            StorageScope::Workspace(workspace_id) => {
                self.workspace(workspace_id).await?.put(key, value).await
            }
            _ => unimplemented!(),
        }?;

        self.on_did_change_value_emitter
            .fire(OnDidChangeValueEvent {
                key: key.to_string(),
                scope: scope.clone(),
                removed: false,
            })
            .await;

        Ok(())
    }

    async fn get(&self, scope: StorageScope, key: &str) -> joinerror::Result<Option<JsonValue>> {
        match scope {
            StorageScope::Application => self.application().await?.get(key).await,
            StorageScope::Workspace(workspace_id) => {
                self.workspace(workspace_id).await?.get(key).await
            }
            _ => unimplemented!(),
        }
    }

    async fn remove(&self, scope: StorageScope, key: &str) -> joinerror::Result<()> {
        match scope.clone() {
            StorageScope::Application => self.application().await?.remove(key).await,
            StorageScope::Workspace(workspace_id) => {
                self.workspace(workspace_id).await?.remove(key).await
            }
            _ => unimplemented!(),
        }?;

        self.on_did_change_value_emitter
            .fire(OnDidChangeValueEvent {
                key: key.to_string(),
                scope: scope.clone(),
                removed: true,
            })
            .await;

        Ok(())
    }
}

impl AppStorage {
    pub async fn new(globals_dir: &Path) -> joinerror::Result<Arc<Self>> {
        let application = ApplicationStorageBackend::new(globals_dir).await?;

        Ok(Self {
            application,
            workspaces: RwLock::new(FxHashMap::default()),
            on_did_change_value_emitter: EventEmitter::<OnDidChangeValueEvent>::new(),
        }
        .into())
    }

    async fn workspace(
        &self,
        workspace_id: Arc<String>,
    ) -> joinerror::Result<Arc<dyn KeyedStorage>> {
        let workspaces = self.workspaces.read().await;

        Ok(workspaces
            .get(&workspace_id)
            .cloned()
            .ok_or_join_err::<()>("workspace not found")?
            .storage()
            .await?)
    }

    async fn application(&self) -> joinerror::Result<Arc<dyn KeyedStorage>> {
        Ok(self.application.storage().await?)
    }
}

#[derive(Deref, Clone)]
pub struct GlobalStorage(Arc<dyn Storage>);

impl dyn Storage {
    pub fn global<R: AppRuntime>(delegate: &AppDelegate<R>) -> Arc<dyn Storage> {
        delegate.global::<GlobalStorage>().0.clone()
    }

    pub fn set_global<R: AppRuntime>(delegate: &AppDelegate<R>, v: Arc<dyn Storage>) {
        delegate.set_global(GlobalStorage(v));
    }
}
