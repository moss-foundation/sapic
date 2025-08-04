use anyhow::{Context as _, Result};
use chrono::Utc;
use derive_more::{Deref, DerefMut};
use moss_activity_indicator::ActivityIndicator;
use moss_applib::{AppRuntime, PublicServiceMarker, ServiceMarker};
use moss_common::api::OperationError;
use moss_db::DatabaseError;
use moss_fs::{FileSystem, RemoveOptions, model_registry::GlobalModelRegistry};
use moss_workspace::{
    Workspace,
    builder::{CreateWorkspaceParams, LoadWorkspaceParams, WorkspaceBuilder},
    services::{
        DynCollectionService as WorkspaceDynCollectionService,
        DynLayoutService as WorkspaceDynLayoutService,
        DynStorageService as WorkspaceDynStorageService, collection_service::CollectionService,
        environment_service::EnvironmentService, layout_service::LayoutService,
        storage_service::StorageService as WorkspaceStorageService,
    },
    workspace::WorkspaceModifyParams,
};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};
use thiserror::Error;
use tokio::sync::RwLock;

use crate::{
    dirs,
    models::primitives::WorkspaceId,
    services::storage_service::StorageService,
    storage::segments::{SEGKEY_WORKSPACE, segkey_last_opened_at, segkey_workspace},
};

#[derive(Debug, Error)]
pub enum WorkspaceServiceError {
    #[error("IO error: {0}")]
    Io(String),

    #[error("Workspace already exists: {0}")]
    AlreadyExists(String),

    #[error("Workspace already loaded: {0}")]
    AlreadyLoaded(String),

    #[error("Storage error: {0}")]
    Storage(String),

    #[error("Workspace not found: {0}")]
    NotFound(String),

    #[error("Workspace is not active")]
    NotActive,

    #[error("Workspace error: {0}")]
    Workspace(String),
}

impl From<DatabaseError> for WorkspaceServiceError {
    fn from(err: DatabaseError) -> Self {
        WorkspaceServiceError::Storage(err.to_string())
    }
}

impl From<WorkspaceServiceError> for OperationError {
    fn from(err: WorkspaceServiceError) -> Self {
        match err {
            WorkspaceServiceError::Io(e) => OperationError::Internal(e),
            WorkspaceServiceError::AlreadyExists(e) => OperationError::AlreadyExists(e),
            WorkspaceServiceError::AlreadyLoaded(e) => OperationError::InvalidInput(e),
            WorkspaceServiceError::Storage(e) => OperationError::Internal(e),
            WorkspaceServiceError::NotFound(e) => OperationError::NotFound(e),
            WorkspaceServiceError::NotActive => {
                OperationError::FailedPrecondition("No active workspace".to_string())
            }
            WorkspaceServiceError::Workspace(e) => OperationError::Internal(e),
        }
    }
}

type WorkspaceServiceResult<T> = Result<T, WorkspaceServiceError>;

#[derive(Deref, DerefMut)]
pub struct ActiveWorkspace<R: AppRuntime> {
    id: WorkspaceId,

    #[deref]
    #[deref_mut]
    handle: Workspace<R>,
}

impl<R: AppRuntime> ActiveWorkspace<R> {
    pub fn id(&self) -> WorkspaceId {
        self.id.clone()
    }
}

pub(crate) struct WorkspaceItemCreateParams {
    pub name: String,
}

pub(crate) struct WorkspaceItemUpdateParams {
    pub name: Option<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct WorkspaceItem {
    pub id: WorkspaceId,
    pub name: String,
    pub abs_path: Arc<Path>,
    pub last_opened_at: Option<i64>,
}

pub(crate) struct WorkspaceItemDescription {
    pub id: WorkspaceId,
    pub name: String,
    pub abs_path: Arc<Path>,
    pub last_opened_at: Option<i64>,
    pub active: bool,
}

type WorkspaceMap = HashMap<WorkspaceId, WorkspaceItem>;

#[derive(Default)]
struct ServiceState<R: AppRuntime> {
    known_workspaces: WorkspaceMap,
    active_workspace: Option<Arc<ActiveWorkspace<R>>>,
}

pub struct WorkspaceService<R: AppRuntime> {
    /// The absolute path to the workspaces directory
    abs_path: Arc<Path>,
    fs: Arc<dyn FileSystem>,
    storage: Arc<StorageService<R>>,
    model_registry: Arc<GlobalModelRegistry>,
    state: Arc<RwLock<ServiceState<R>>>,
}

impl<R: AppRuntime> ServiceMarker for WorkspaceService<R> {}
impl<R: AppRuntime> PublicServiceMarker for WorkspaceService<R> {}

impl<R: AppRuntime> WorkspaceService<R> {
    pub async fn new(
        ctx: &R::AsyncContext,
        storage_service: Arc<StorageService<R>>,
        fs: Arc<dyn FileSystem>,
        abs_path: &Path,
        model_registry: Arc<GlobalModelRegistry>,
    ) -> WorkspaceServiceResult<Self> {
        debug_assert!(abs_path.is_absolute());
        let abs_path: Arc<Path> = abs_path.join(dirs::WORKSPACES_DIR).into();
        debug_assert!(abs_path.exists());

        let known_workspaces =
            restore_known_workspaces::<R>(ctx, &abs_path, &fs, &storage_service).await?;

        Ok(Self {
            fs,
            storage: storage_service,
            abs_path,
            model_registry,
            state: Arc::new(RwLock::new(ServiceState {
                known_workspaces,
                active_workspace: None,
            })),
        })
    }

    pub fn absolutize(&self, path: impl AsRef<Path>) -> PathBuf {
        self.abs_path.join(path)
    }

    pub(crate) async fn list_workspaces(
        &self,
    ) -> WorkspaceServiceResult<Vec<WorkspaceItemDescription>> {
        let state_lock = self.state.read().await;
        let active_workspace_id = state_lock.active_workspace.as_ref().map(|a| a.id.clone());

        let workspaces = state_lock
            .known_workspaces
            .values()
            .map(|item| WorkspaceItemDescription {
                id: item.id.clone(),
                name: item.name.clone(),
                abs_path: item.abs_path.clone(),
                last_opened_at: item.last_opened_at,
                active: Some(item.id.clone()) == active_workspace_id,
            })
            .collect();
        Ok(workspaces)
    }

    pub(crate) async fn update_workspace(
        &self,
        params: WorkspaceItemUpdateParams,
    ) -> WorkspaceServiceResult<()> {
        let mut state_lock = self.state.write().await;
        let workspace = state_lock
            .active_workspace
            .as_ref()
            .ok_or(WorkspaceServiceError::NotActive)?;

        let mut descriptor = state_lock
            .known_workspaces
            .get(&workspace.id)
            .ok_or(WorkspaceServiceError::NotFound(workspace.id.to_string()))?
            .clone();

        workspace
            .modify(WorkspaceModifyParams {
                name: params.name.clone(),
            })
            .await
            .map_err(|e| WorkspaceServiceError::Workspace(e.to_string()))?;

        if let Some(new_name) = params.name {
            descriptor.name = new_name;
        }

        state_lock
            .known_workspaces
            .insert(descriptor.id.clone(), descriptor);

        Ok(())
    }

    pub(crate) async fn delete_workspace(
        &self,
        ctx: &R::AsyncContext,
        id: &WorkspaceId,
    ) -> WorkspaceServiceResult<()> {
        let (active_workspace_id, item) = {
            let state_lock = self.state.read().await;

            let active_workspace_id = state_lock.active_workspace.as_ref().map(|a| a.id.clone());
            let item = state_lock.known_workspaces.get(&id).cloned();

            (active_workspace_id, item)
        };

        let item = item.ok_or(WorkspaceServiceError::NotFound(id.to_string()))?;
        if item.abs_path.exists() {
            self.fs
                .remove_dir(
                    &item.abs_path,
                    RemoveOptions {
                        recursive: true,
                        ignore_if_not_exists: true,
                    },
                )
                .await
                .map_err(|e| WorkspaceServiceError::Io(e.to_string()))?;
        }

        {
            let mut state_lock = self.state.write().await;
            state_lock.known_workspaces.remove(&id);
        }

        {
            // Only try to remove from database if it exists (ignore error if not found)
            let _ = self
                .storage
                .remove_all_by_prefix(ctx, &segkey_workspace(&id).to_string())
                .await
                .map_err(|e| WorkspaceServiceError::Storage(e.to_string())); // TODO: log error
        }

        if active_workspace_id != Some(item.id) {
            return Ok(());
        }

        Ok(self.deactivate_workspace(ctx).await?)
    }

    pub(crate) async fn create_workspace(
        &self,
        id: &WorkspaceId,
        params: WorkspaceItemCreateParams,
    ) -> WorkspaceServiceResult<WorkspaceItemDescription> {
        let mut state_lock = self.state.write().await;

        let id_str = id.to_string();

        let abs_path: Arc<Path> = self.absolutize(&id_str).into();
        self.fs
            .create_dir(&abs_path)
            .await
            .context("Failed to create workspace directory")
            .map_err(|e| WorkspaceServiceError::Io(e.to_string()))?;

        WorkspaceBuilder::initialize(
            self.fs.clone(),
            CreateWorkspaceParams {
                name: params.name.clone(),
                abs_path: abs_path.clone(),
            },
        )
        .await
        .context("Failed to initialize the workspace")
        .map_err(|e| WorkspaceServiceError::Workspace(e.to_string()))?;

        state_lock.known_workspaces.insert(
            id.clone(),
            WorkspaceItem {
                id: id.clone(),
                name: params.name.clone(),
                last_opened_at: None,
                abs_path: Arc::clone(&abs_path),
            },
        );

        Ok(WorkspaceItemDescription {
            id: id.to_owned(),
            name: params.name,
            abs_path: Arc::clone(&abs_path),
            last_opened_at: None,
            active: false,
        })
    }

    pub async fn workspace(&self) -> Option<Arc<ActiveWorkspace<R>>> {
        let state_lock = self.state.read().await;
        if state_lock.active_workspace.is_none() {
            return None;
        }

        Some(state_lock.active_workspace.as_ref()?.clone())
    }

    pub(crate) async fn activate_workspace(
        &self,
        ctx: &R::AsyncContext,
        id: &WorkspaceId,
        activity_indicator: ActivityIndicator<R::EventLoop>,
    ) -> WorkspaceServiceResult<WorkspaceItemDescription> {
        let mut state_lock = self.state.write().await;
        let item = state_lock
            .known_workspaces
            .get_mut(&id)
            .ok_or(WorkspaceServiceError::NotFound(id.to_string()))?;

        let last_opened_at = Utc::now().timestamp();
        let name = item.name.clone();
        let abs_path: Arc<Path> = self.absolutize(&id.to_string()).into();

        let storage_service: Arc<WorkspaceDynStorageService<R>> = {
            let service: Arc<WorkspaceStorageService<R>> = WorkspaceStorageService::new(&abs_path)
                .context("Failed to load the storage service")
                .map_err(|e| WorkspaceServiceError::Workspace(e.to_string()))?
                .into();

            WorkspaceDynStorageService::new(service)
        };

        let collection_service: Arc<WorkspaceDynCollectionService<R>> = {
            let service: Arc<CollectionService<R>> = CollectionService::new(
                ctx,
                abs_path.clone(),
                self.fs.clone(),
                storage_service.clone(),
            )
            .await
            .map_err(|e| WorkspaceServiceError::Workspace(e.to_string()))?
            .into();

            WorkspaceDynCollectionService::new(service)
        };

        let layout_service: Arc<WorkspaceDynLayoutService<R>> = {
            let service: Arc<LayoutService<R>> = LayoutService::new(storage_service.clone()).into();

            WorkspaceDynLayoutService::new(service)
        };

        let environment_service: Arc<EnvironmentService<R>> =
            EnvironmentService::new(&abs_path, self.fs.clone(), self.model_registry.clone())
                .await
                .map_err(|e| WorkspaceServiceError::Workspace(e.to_string()))?
                .into();

        let workspace = WorkspaceBuilder::new(self.fs.clone())
            .with_service::<WorkspaceDynStorageService<R>>(storage_service.clone())
            .with_service::<WorkspaceDynCollectionService<R>>(collection_service)
            .with_service::<WorkspaceDynLayoutService<R>>(layout_service)
            .with_service::<EnvironmentService<R>>(environment_service)
            .load(
                activity_indicator,
                LoadWorkspaceParams {
                    abs_path: abs_path.clone(),
                },
            )
            .await
            .context("Failed to create the workspace")
            .map_err(|e| WorkspaceServiceError::Workspace(e.to_string()))?;

        item.last_opened_at = Some(last_opened_at);
        state_lock.active_workspace = Some(
            ActiveWorkspace {
                id: id.clone(),
                handle: workspace,
            }
            .into(),
        );

        {
            let mut txn = self.storage.begin_write_with_context(ctx).await?;

            self.storage
                .put_last_active_workspace_txn(ctx, &mut txn, &id)
                .await?;
            self.storage
                .put_last_opened_at_txn(ctx, &mut txn, &id, last_opened_at)
                .await?;

            txn.commit()?;
        }

        // let active_workspace_id: ctxkeys::ActiveWorkspaceId = id.to_owned().into();
        // ctx.set_value(active_workspace_id);

        Ok(WorkspaceItemDescription {
            id: id.to_owned(),
            name,
            abs_path: Arc::clone(&abs_path),
            last_opened_at: Some(last_opened_at),
            active: true,
        })
    }

    pub(crate) async fn deactivate_workspace(
        &self,
        ctx: &R::AsyncContext,
    ) -> WorkspaceServiceResult<()> {
        let mut state_lock = self.state.write().await;
        state_lock.active_workspace = None;

        self.storage.remove_last_active_workspace(ctx).await?;

        // ctx.remove_value::<ctxkeys::ActiveWorkspaceId>();

        Ok(())
    }
}

async fn restore_known_workspaces<R: AppRuntime>(
    ctx: &R::AsyncContext,
    abs_path: &Path,
    fs: &Arc<dyn FileSystem>,
    storage_service: &Arc<StorageService<R>>,
) -> WorkspaceServiceResult<WorkspaceMap> {
    let mut workspaces = HashMap::new();

    let restored_items = storage_service
        .list_all_by_prefix(ctx, SEGKEY_WORKSPACE.as_str().expect("invalid utf-8"))
        .await?;

    let mut read_dir = fs
        .read_dir(&abs_path)
        .await
        .map_err(|e| WorkspaceServiceError::Io(e.to_string()))?;

    while let Some(entry) = read_dir
        .next_entry()
        .await
        .map_err(|e| WorkspaceServiceError::Io(e.to_string()))?
    {
        if !entry
            .file_type()
            .await
            .map_err(|e| WorkspaceServiceError::Io(e.to_string()))?
            .is_dir()
        {
            continue;
        }

        let id_str = entry.file_name().to_string_lossy().to_string();
        let id: WorkspaceId = id_str.into();

        let summary = Workspace::<R>::summary(fs.clone(), &entry.path())
            .await
            .map_err(|e| WorkspaceServiceError::Workspace(e.to_string()))?;

        let filtered_items = restored_items
            .iter()
            .filter(|(key, _)| key.starts_with(&segkey_workspace(&id)))
            .collect::<HashMap<_, _>>();

        let last_opened_at = filtered_items
            .get(&segkey_last_opened_at(&id))
            .map(|v| {
                v.deserialize::<i64>()
                    .map_err(|e| WorkspaceServiceError::Storage(e.to_string()))
            })
            .transpose()?;

        workspaces.insert(
            id.clone(),
            WorkspaceItem {
                id,
                name: summary.manifest.name,
                abs_path: entry.path().into(),
                last_opened_at,
            }
            .into(),
        );
    }

    Ok(workspaces)
}
