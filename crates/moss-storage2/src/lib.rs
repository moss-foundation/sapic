pub mod adapters;
pub mod application_storage;
pub mod models;
pub mod workspace_storage;

use async_trait::async_trait;
use derive_more::Deref;
use joinerror::{OptionExt, ResultExt};
use moss_app_delegate::AppDelegate;
use moss_applib::{AppRuntime, subscription::EventEmitter};
use moss_logging::session;
use rustc_hash::FxHashMap;
use serde_json::Value as JsonValue;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::RwLock;

use crate::{
    adapters::{KeyedStorage, Options},
    application_storage::ApplicationStorageBackend,
    models::{events::OnDidChangeValueEvent, primitives::StorageScope},
    workspace_storage::WorkspaceStorageBackend,
};

#[async_trait]
pub trait Storage: Send + Sync {
    async fn add_workspace(&self, workspace_id: Arc<String>) -> joinerror::Result<()>;
    async fn remove_workspace(&self, workspace_id: Arc<String>) -> joinerror::Result<()>;

    async fn put(&self, scope: StorageScope, key: &str, value: JsonValue) -> joinerror::Result<()>;
    async fn get(&self, scope: StorageScope, key: &str) -> joinerror::Result<Option<JsonValue>>;
    async fn remove(&self, scope: StorageScope, key: &str) -> joinerror::Result<Option<JsonValue>>;

    async fn put_batch(
        &self,
        scope: StorageScope,
        items: &[(&str, JsonValue)],
    ) -> joinerror::Result<()>;
    async fn get_batch(
        &self,
        scope: StorageScope,
        keys: &[&str],
    ) -> joinerror::Result<Vec<(String, Option<JsonValue>)>>;
    async fn remove_batch(
        &self,
        scope: StorageScope,
        keys: &[&str],
    ) -> joinerror::Result<Vec<(String, Option<JsonValue>)>>;

    async fn get_batch_by_prefix(
        &self,
        scope: StorageScope,
        prefix: &str,
    ) -> joinerror::Result<Vec<(String, Option<JsonValue>)>>;

    async fn remove_batch_by_prefix(
        &self,
        scope: StorageScope,
        prefix: &str,
    ) -> joinerror::Result<Vec<(String, Option<JsonValue>)>>;

    async fn capabilities(self: Arc<Self>) -> Arc<dyn StorageCapabilities>;
}

pub enum FlushMode {
    Checkpoint,
    Force,
}

#[async_trait]
pub trait StorageCapabilities: Send + Sync {
    async fn last_checkpoint(&self) -> Option<Instant>;
    async fn flush(&self, mode: FlushMode) -> joinerror::Result<()>;
    async fn optimize(&self) -> joinerror::Result<()>;
}

#[derive(Debug, Clone)]
pub struct AppStorageOptions {
    pub in_memory: Option<bool>,
    pub busy_timeout: Option<Duration>,
}

impl Into<Options> for AppStorageOptions {
    fn into(self) -> Options {
        Options {
            in_memory: self.in_memory,
            busy_timeout: self.busy_timeout,
        }
    }
}
pub struct AppStorage {
    workspaces_dir: PathBuf,
    application: ApplicationStorageBackend,
    workspaces: RwLock<FxHashMap<Arc<String>, WorkspaceStorageBackend>>,
    options: Option<AppStorageOptions>,

    on_did_change_value_emitter: EventEmitter<OnDidChangeValueEvent>,
    last_checkpoint: RwLock<Option<Instant>>,
}

#[async_trait]
impl Storage for AppStorage {
    async fn add_workspace(&self, workspace_id: Arc<String>) -> joinerror::Result<()> {
        let workspace = WorkspaceStorageBackend::new(
            &self.workspaces_dir.join(workspace_id.as_str()),
            self.options.clone().map(Into::into),
        )
        .await
        .join_err_with::<()>(|| {
            format!(
                "failed to create workspace storage backend for workspace `{}`",
                workspace_id
            )
        })?;

        self.workspaces
            .write()
            .await
            .insert(workspace_id, workspace);

        Ok(())
    }

    async fn remove_workspace(&self, workspace_id: Arc<String>) -> joinerror::Result<()> {
        self.workspaces.write().await.remove(&workspace_id);

        Ok(())
    }

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

    async fn remove(&self, scope: StorageScope, key: &str) -> joinerror::Result<Option<JsonValue>> {
        let value = match scope.clone() {
            StorageScope::Application => self.application().await?.remove(key).await?,
            StorageScope::Workspace(workspace_id) => {
                self.workspace(workspace_id).await?.remove(key).await?
            }
            _ => unimplemented!(),
        };

        self.on_did_change_value_emitter
            .fire(OnDidChangeValueEvent {
                key: key.to_string(),
                scope: scope.clone(),
                removed: true,
            })
            .await;

        Ok(value)
    }

    async fn put_batch(
        &self,
        scope: StorageScope,
        items: &[(&str, JsonValue)],
    ) -> joinerror::Result<()> {
        match scope.clone() {
            StorageScope::Application => self.application().await?.put_batch(items).await,
            StorageScope::Workspace(workspace_id) => {
                self.workspace(workspace_id).await?.put_batch(items).await
            }
            _ => unimplemented!(),
        }
    }

    async fn get_batch(
        &self,
        scope: StorageScope,
        keys: &[&str],
    ) -> joinerror::Result<Vec<(String, Option<JsonValue>)>> {
        match scope.clone() {
            StorageScope::Application => self.application().await?.get_batch(keys).await,
            StorageScope::Workspace(workspace_id) => {
                self.workspace(workspace_id).await?.get_batch(keys).await
            }
            _ => unimplemented!(),
        }
    }

    async fn remove_batch(
        &self,
        scope: StorageScope,
        keys: &[&str],
    ) -> joinerror::Result<Vec<(String, Option<JsonValue>)>> {
        match scope.clone() {
            StorageScope::Application => self.application().await?.remove_batch(keys).await,
            StorageScope::Workspace(workspace_id) => {
                self.workspace(workspace_id).await?.remove_batch(keys).await
            }
            _ => unimplemented!(),
        }
    }

    async fn get_batch_by_prefix(
        &self,
        scope: StorageScope,
        prefix: &str,
    ) -> joinerror::Result<Vec<(String, Option<JsonValue>)>> {
        match scope.clone() {
            StorageScope::Application => {
                self.application().await?.get_batch_by_prefix(prefix).await
            }
            StorageScope::Workspace(workspace_id) => {
                self.workspace(workspace_id)
                    .await?
                    .get_batch_by_prefix(prefix)
                    .await
            }
            _ => unimplemented!(),
        }
    }

    async fn remove_batch_by_prefix(
        &self,
        scope: StorageScope,
        prefix: &str,
    ) -> joinerror::Result<Vec<(String, Option<JsonValue>)>> {
        match scope.clone() {
            StorageScope::Application => {
                self.application()
                    .await?
                    .remove_batch_by_prefix(prefix)
                    .await
            }
            StorageScope::Workspace(workspace_id) => {
                self.workspace(workspace_id)
                    .await?
                    .remove_batch_by_prefix(prefix)
                    .await
            }
            _ => unimplemented!(),
        }
    }

    async fn capabilities(self: Arc<Self>) -> Arc<dyn StorageCapabilities> {
        self.clone()
    }
}

#[async_trait]
impl StorageCapabilities for AppStorage {
    async fn last_checkpoint(&self) -> Option<Instant> {
        self.last_checkpoint.read().await.clone()
    }

    async fn flush(&self, mode: FlushMode) -> joinerror::Result<()> {
        let mut storages_to_flush = vec![self.application.capabilities().await?.flushable.clone()];

        for workspace in self.workspaces.read().await.values() {
            storages_to_flush.push(workspace.capabilities().await?.flushable.clone());
        }

        for storage in storages_to_flush {
            let storage = if let Some(storage) = storage {
                storage
            } else {
                continue;
            };

            match mode {
                FlushMode::Checkpoint => {
                    if let Err(e) = storage.checkpoint().await {
                        session::error!(format!("failed to checkpoint storage: {}", e));
                    }
                }
                FlushMode::Force => {
                    if let Err(e) = storage.flush().await {
                        session::error!(format!("failed to flush storage: {}", e));
                    }
                }
            }
        }

        let mut last_checkpoint_lock = self.last_checkpoint.write().await;
        *last_checkpoint_lock = Some(Instant::now());

        Ok(())
    }

    async fn optimize(&self) -> joinerror::Result<()> {
        unimplemented!()
    }
}

impl AppStorage {
    pub async fn new(
        globals_dir: &Path,
        workspaces_dir: PathBuf,
        options: Option<AppStorageOptions>,
    ) -> joinerror::Result<Arc<Self>> {
        let application =
            ApplicationStorageBackend::new(globals_dir, options.clone().map(Into::into)).await?;

        Ok(Self {
            workspaces_dir,
            application,
            workspaces: RwLock::new(FxHashMap::default()),
            options,
            on_did_change_value_emitter: EventEmitter::<OnDidChangeValueEvent>::new(),
            last_checkpoint: RwLock::new(None),
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
