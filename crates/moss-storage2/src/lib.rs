pub mod adapters;
pub mod application_storage;
pub mod models;
mod project_storage;
pub mod workspace_storage;

use async_trait::async_trait;
use derive_more::Deref;
use joinerror::{OptionExt, ResultExt};
use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;
use moss_logging::session;
use rustc_hash::{FxHashMap, FxHashSet};
use sapic_core::subscription::EventEmitter;
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
    project_storage::ProjectStorageBackend,
    workspace_storage::WorkspaceStorageBackend,
};

#[async_trait]
pub trait SubstoreManager: Send + Sync {
    async fn add_workspace(&self, workspace_id: Arc<String>) -> joinerror::Result<()>;
    async fn remove_workspace(&self, workspace_id: Arc<String>) -> joinerror::Result<()>;
    async fn add_project(
        &self,
        workspace_id: Arc<String>,
        project_id: Arc<String>,
    ) -> joinerror::Result<()>;
    async fn remove_project(
        &self,
        workspace_id: Arc<String>,
        project_id: Arc<String>,
    ) -> joinerror::Result<()>;
}

#[async_trait]
pub trait Storage: SubstoreManager + Send + Sync {
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
    ) -> joinerror::Result<Vec<(String, JsonValue)>>;

    async fn remove_batch_by_prefix(
        &self,
        scope: StorageScope,
        prefix: &str,
    ) -> joinerror::Result<Vec<(String, JsonValue)>>;

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

type WorkspaceId = Arc<String>;
type ProjectId = Arc<String>;

pub struct AppStorage {
    workspaces_dir: PathBuf,
    application: ApplicationStorageBackend,
    workspaces: RwLock<FxHashMap<WorkspaceId, WorkspaceStorageBackend>>,
    projects: RwLock<FxHashMap<ProjectId, ProjectStorageBackend>>,

    // Storing which workspace contains which projects
    // So when we drop a workspace storage, we drop all associated projects' as well
    workspace_to_projects: RwLock<FxHashMap<WorkspaceId, FxHashSet<ProjectId>>>,

    options: Option<AppStorageOptions>,

    on_did_change_value_emitter: EventEmitter<OnDidChangeValueEvent>,
    last_checkpoint: RwLock<Option<Instant>>,
}

#[cfg(feature = "integration-tests")]
impl AppStorage {
    pub async fn close(&self) -> joinerror::Result<()> {
        self.application
            .capabilities()
            .await?
            .closable
            .expect("Must be closable")
            .close()
            .await;
        self.workspaces.write().await.clear();
        self.projects.write().await.clear();
        Ok(())
    }
}

#[async_trait]
impl SubstoreManager for AppStorage {
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
            .insert(workspace_id.clone(), workspace);

        self.workspace_to_projects
            .write()
            .await
            .insert(workspace_id, FxHashSet::default());

        Ok(())
    }

    // Remove a workspace and all associated project storages
    async fn remove_workspace(&self, workspace_id: Arc<String>) -> joinerror::Result<()> {
        if let Some(workspace_storage) = self.workspaces.write().await.remove(&workspace_id) {
            // Properly close the database handle to prevent any lock
            workspace_storage
                .capabilities()
                .await?
                .closable
                .expect("Must be closable")
                .close()
                .await;
        }

        let projects = self
            .workspace_to_projects
            .write()
            .await
            .remove(&workspace_id);
        if let Some(projects) = projects {
            let mut projects_lock = self.projects.write().await;
            for project_id in projects {
                if let Some(project_storage) = projects_lock.remove(&project_id) {
                    // Properly close the database handle to prevent any lock
                    project_storage
                        .capabilities()
                        .await?
                        .closable
                        .expect("Must be closable")
                        .close()
                        .await;
                }
            }
        }

        Ok(())
    }

    async fn add_project(
        &self,
        workspace_id: Arc<String>,
        project_id: Arc<String>,
    ) -> joinerror::Result<()> {
        let mut workspace_projects_lock = self.workspace_to_projects.write().await;

        let projects = if let Some(projects) = workspace_projects_lock.get_mut(&workspace_id) {
            projects
        } else {
            joinerror::bail!("workspace `{}` not found", workspace_id);
        };

        let project = ProjectStorageBackend::new(
            &self
                .workspaces_dir
                .join(workspace_id.as_str())
                .join("projects")
                .join(project_id.as_str()),
            self.options.clone().map(Into::into),
        )
        .await
        .join_err_with::<()>(|| {
            format!(
                "failed to create project storage backend for project `{}`",
                project_id
            )
        })?;

        self.projects
            .write()
            .await
            .insert(project_id.clone(), project);

        projects.insert(project_id);

        Ok(())
    }

    async fn remove_project(
        &self,
        workspace_id: Arc<String>,
        project_id: Arc<String>,
    ) -> joinerror::Result<()> {
        if let Some(project_storage) = self.projects.write().await.remove(&project_id) {
            // Properly close the database handle to prevent any lock
            project_storage
                .capabilities()
                .await?
                .closable
                .expect("Must be closable")
                .close()
                .await;
        };
        if let Some(workspace_projects) = self
            .workspace_to_projects
            .write()
            .await
            .get_mut(&workspace_id)
        {
            workspace_projects.remove(&project_id);
        }
        Ok(())
    }
}

#[async_trait]
impl Storage for AppStorage {
    async fn put(&self, scope: StorageScope, key: &str, value: JsonValue) -> joinerror::Result<()> {
        match scope.clone() {
            StorageScope::Application => self.application().await?.put(key, value).await,
            StorageScope::Workspace(workspace_id) => {
                self.workspace(workspace_id).await?.put(key, value).await
            }
            StorageScope::Project(project_id) => {
                self.project(project_id).await?.put(key, value).await
            }
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
            StorageScope::Project(project_id) => self.project(project_id).await?.get(key).await,
        }
    }

    async fn remove(&self, scope: StorageScope, key: &str) -> joinerror::Result<Option<JsonValue>> {
        let value = match scope.clone() {
            StorageScope::Application => self.application().await?.remove(key).await?,
            StorageScope::Workspace(workspace_id) => {
                self.workspace(workspace_id).await?.remove(key).await?
            }
            StorageScope::Project(project_id) => {
                self.project(project_id).await?.remove(key).await?
            }
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
            StorageScope::Project(project_id) => {
                self.project(project_id).await?.put_batch(items).await
            }
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
            StorageScope::Project(project_id) => {
                self.project(project_id).await?.get_batch(keys).await
            }
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
            StorageScope::Project(project_id) => {
                self.project(project_id).await?.remove_batch(keys).await
            }
        }
    }

    async fn get_batch_by_prefix(
        &self,
        scope: StorageScope,
        prefix: &str,
    ) -> joinerror::Result<Vec<(String, JsonValue)>> {
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
            StorageScope::Project(project_id) => {
                self.project(project_id)
                    .await?
                    .get_batch_by_prefix(prefix)
                    .await
            }
        }
    }

    async fn remove_batch_by_prefix(
        &self,
        scope: StorageScope,
        prefix: &str,
    ) -> joinerror::Result<Vec<(String, JsonValue)>> {
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
            StorageScope::Project(project_id) => {
                self.project(project_id)
                    .await?
                    .remove_batch_by_prefix(prefix)
                    .await
            }
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

        for project in self.projects.read().await.values() {
            storages_to_flush.push(project.capabilities().await?.flushable.clone());
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
            projects: RwLock::new(FxHashMap::default()),
            workspace_to_projects: RwLock::new(FxHashMap::default()),
            options,
            on_did_change_value_emitter: EventEmitter::<OnDidChangeValueEvent>::new(),
            last_checkpoint: RwLock::new(None),
        }
        .into())
    }

    async fn project(&self, project_id: Arc<String>) -> joinerror::Result<Arc<dyn KeyedStorage>> {
        let projects = self.projects.read().await;

        Ok(projects
            .get(&project_id)
            .ok_or_join_err::<()>("project storage not found")?
            .storage()
            .await?)
    }

    async fn workspace(
        &self,
        workspace_id: Arc<String>,
    ) -> joinerror::Result<Arc<dyn KeyedStorage>> {
        let workspaces = self.workspaces.read().await;

        Ok(workspaces
            .get(&workspace_id)
            .ok_or_join_err::<()>("workspace storage not found")?
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
