use anyhow::{Context as _, Result};
use chrono::Utc;
use derive_more::{Deref, DerefMut};
use moss_activity_indicator::ActivityIndicator;
use moss_applib::ServiceMarker;
use moss_common::api::OperationError;
use moss_db::primitives::AnyValue;
use moss_fs::{FileSystem, RemoveOptions};
use moss_storage::{
    GlobalStorage,
    global_storage::entities::WorkspaceInfoEntity,
    primitives::segkey::SegmentExt,
    storage::operations::{ListByPrefix, PutItem, RemoveItem},
};
use moss_workspace::{
    Workspace,
    builder::{WorkspaceBuilder, WorkspaceCreateParams, WorkspaceLoadParams},
    context::{WorkspaceContext, WorkspaceContextState},
    services::{
        collection_service::CollectionService, layout_service::LayoutService,
        storage_service::StorageService,
    },
    workspace::ModifyParams,
};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};
use tauri::{AppHandle, Runtime as TauriRuntime};
use thiserror::Error;
use tokio::sync::{OnceCell, RwLock, RwLockMappedWriteGuard, RwLockReadGuard, RwLockWriteGuard};
use uuid::Uuid;

use crate::{
    context::{AnyAppContext, ctxkeys},
    dirs,
    storage::segments::WORKSPACE_SEGKEY,
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

pub type WorkspaceServiceResult<T> = Result<T, WorkspaceServiceError>;

#[derive(Debug, Clone)]
pub struct WorkspaceDescriptor {
    pub id: Uuid,
    pub name: String,
    pub abs_path: Arc<Path>,
    pub last_opened_at: Option<i64>,
}

type WorkspaceMap = HashMap<Uuid, Arc<WorkspaceDescriptor>>;

#[derive(Deref)]
pub struct WorkspaceReadGuard<'a, R: TauriRuntime> {
    pub id: Uuid,

    #[deref]
    pub guard: RwLockReadGuard<'a, Workspace<R>>,
}

#[derive(Deref, DerefMut)]
pub struct WorkspaceWriteGuard<'a, R: TauriRuntime> {
    pub id: Uuid,

    #[deref]
    #[deref_mut]
    pub guard: RwLockMappedWriteGuard<'a, Workspace<R>>,
}

#[derive(Deref, DerefMut)]
pub struct ActiveWorkspace<R: TauriRuntime> {
    id: Uuid,
    #[deref]
    #[deref_mut]
    this: Workspace<R>,
    context: Arc<RwLock<WorkspaceContextState>>,
}

pub struct WorkspaceService<R: TauriRuntime> {
    /// The absolute path to the workspaces directory
    abs_path: Arc<Path>,
    fs: Arc<dyn FileSystem>,
    global_storage: Arc<dyn GlobalStorage>,
    known_workspaces: OnceCell<RwLock<WorkspaceMap>>,
    active_workspace: RwLock<Option<ActiveWorkspace<R>>>,
}

impl<R: TauriRuntime> ServiceMarker for WorkspaceService<R> {}

impl<R: TauriRuntime> WorkspaceService<R> {
    pub fn new(
        global_storage: Arc<dyn GlobalStorage>,
        fs: Arc<dyn FileSystem>,
        abs_path: &Path,
    ) -> Self {
        debug_assert!(abs_path.is_absolute());
        let abs_path: Arc<Path> = abs_path.join(dirs::WORKSPACES_DIR).into();
        debug_assert!(abs_path.exists());

        Self {
            fs,
            global_storage,
            abs_path,
            known_workspaces: OnceCell::new(),
            active_workspace: RwLock::new(None),
        }
    }

    pub fn absolutize(&self, path: impl AsRef<Path>) -> PathBuf {
        self.abs_path.join(path)
    }

    pub(crate) async fn map_known_workspaces_to_vec<T>(
        &self,
        f: impl Fn(Uuid, Arc<WorkspaceDescriptor>) -> T,
    ) -> WorkspaceServiceResult<Vec<T>> {
        let workspaces = self.workspaces().await?;
        let workspaces_lock = workspaces.read().await;
        let mut result = Vec::with_capacity(workspaces_lock.len());

        for (&id, v) in workspaces_lock.iter() {
            result.push(f(id, v.clone()));
        }

        Ok(result)
    }

    pub(crate) async fn update_workspace(
        &self,
        params: ModifyParams,
    ) -> WorkspaceServiceResult<()> {
        let workspaces = self.workspaces().await?;
        let mut active_workspace = self.active_workspace.write().await;
        let workspace = active_workspace
            .as_mut()
            .ok_or(WorkspaceServiceError::NotActive)?;

        let mut workspaces_lock = workspaces.write().await;
        let mut descriptor = workspaces_lock
            .get(&workspace.id)
            .ok_or(WorkspaceServiceError::NotFound(workspace.id.to_string()))?
            .as_ref()
            .clone();

        workspace
            .modify(params.clone())
            .await
            .map_err(|e| WorkspaceServiceError::Workspace(e.to_string()))?;

        if let Some(new_name) = params.name {
            descriptor.name = new_name;
        }

        workspaces_lock.insert(workspace.id, Arc::new(descriptor));

        Ok(())
    }

    pub(crate) async fn delete_workspace<C: AnyAppContext<R>>(
        &self,
        ctx: &C,
        id: Uuid,
    ) -> WorkspaceServiceResult<()> {
        let workspaces = self.workspaces().await?;

        let (id, abs_path) = if let Some(descriptor) = workspaces.read().await.get(&id) {
            (descriptor.id, descriptor.abs_path.clone())
        } else {
            return Err(WorkspaceServiceError::NotFound(id.to_string()));
        };

        if abs_path.exists() {
            self.fs
                .remove_dir(
                    &abs_path,
                    RemoveOptions {
                        recursive: true,
                        ignore_if_not_exists: true,
                    },
                )
                .await
                .map_err(|e| WorkspaceServiceError::Io(e.to_string()))?;
        }

        {
            let item_store = self.global_storage.item_store();
            let segkey = WORKSPACE_SEGKEY.join(id.to_string());
            // Only try to remove from database if it exists (ignore error if not found)
            let _ = RemoveItem::remove(item_store.as_ref(), segkey);
        }

        {
            let mut workspaces_lock = workspaces.write().await;
            workspaces_lock.remove(&id);
        }

        let active_workspace_id = self.active_workspace.read().await.as_ref().map(|a| a.id);
        if active_workspace_id != Some(id) {
            return Ok(());
        }

        Ok(self.deactivate_workspace(ctx).await)
    }

    pub(crate) async fn load_workspace(
        &self,
        id: Uuid,
        activity_indicator: ActivityIndicator<R>,
    ) -> WorkspaceServiceResult<(Workspace<R>, Arc<WorkspaceDescriptor>)> {
        let workspaces = self.workspaces().await?;
        let descriptor = if let Some(d) = workspaces.read().await.get(&id) {
            d.clone()
        } else {
            return Err(WorkspaceServiceError::NotFound(id.to_string()));
        };

        if !descriptor.abs_path.exists() {
            return Err(WorkspaceServiceError::NotFound(
                descriptor.abs_path.to_string_lossy().to_string(),
            ));
        }

        let active_workspace_id = self.active_workspace.read().await.as_ref().map(|a| a.id);
        if active_workspace_id == Some(id) {
            return Err(WorkspaceServiceError::AlreadyLoaded(id.to_string()));
        }

        let storage_service: Arc<StorageService> = StorageService::new(&descriptor.abs_path)
            .context("Failed to load the storage service")
            .map_err(|e| WorkspaceServiceError::Workspace(e.to_string()))?
            .into();

        let collection_service = CollectionService::new(
            descriptor.abs_path.clone(),
            self.fs.clone(),
            storage_service.clone(),
        )
        .await
        .context("Failed to load the collection service")
        .map_err(|e| WorkspaceServiceError::Workspace(e.to_string()))?;

        let layout_service = LayoutService::new(storage_service.clone());

        let workspace = WorkspaceBuilder::new(self.fs.clone())
            .with_service::<StorageService>(storage_service.clone())
            .with_service(collection_service)
            .with_service(layout_service)
            .load(
                WorkspaceLoadParams {
                    abs_path: descriptor.abs_path.clone(),
                },
                activity_indicator,
            )
            .await
            .context("Failed to load the workspace")
            .map_err(|e| WorkspaceServiceError::Workspace(e.to_string()))?;

        Ok((workspace, descriptor))
    }

    pub(crate) async fn create_workspace(
        &self,
        name: &str,
        activity_indicator: ActivityIndicator<R>,
    ) -> WorkspaceServiceResult<(Workspace<R>, Arc<WorkspaceDescriptor>)> {
        let workspaces = self.workspaces().await?;

        let id = Uuid::new_v4();
        let id_str = id.to_string();

        let abs_path: Arc<Path> = self.absolutize(&id_str).into();
        self.fs
            .create_dir(&abs_path)
            .await
            .context("Failed to create workspace directory")
            .map_err(|e| WorkspaceServiceError::Io(e.to_string()))?;

        let storage_service: Arc<StorageService> = StorageService::new(&abs_path)
            .context("Failed to load the storage service")
            .map_err(|e| WorkspaceServiceError::Workspace(e.to_string()))?
            .into();

        let collection_service =
            CollectionService::new(abs_path.clone(), self.fs.clone(), storage_service.clone())
                .await
                .context("Failed to load the collection service")
                .map_err(|e| WorkspaceServiceError::Workspace(e.to_string()))?;

        let layout_service = LayoutService::new(storage_service.clone());

        let workspace = WorkspaceBuilder::new(self.fs.clone())
            .with_service::<StorageService>(storage_service.clone())
            .with_service(collection_service)
            .with_service(layout_service)
            .create(
                WorkspaceCreateParams {
                    name: name.to_string(),
                    abs_path: abs_path.clone(),
                },
                activity_indicator,
            )
            .await
            .context("Failed to create the workspace")
            .map_err(|e| WorkspaceServiceError::Workspace(e.to_string()))?;

        let descriptor: Arc<WorkspaceDescriptor> = WorkspaceDescriptor {
            id,
            name: name.to_owned(),
            last_opened_at: None,
            abs_path: Arc::clone(&abs_path),
        }
        .into();

        workspaces.write().await.insert(id, descriptor.clone());

        Ok((workspace, descriptor))
    }

    // TODO: remove this after we remove the describe_workbench_state api
    pub(crate) async fn workspace(&self) -> Option<WorkspaceReadGuard<'_, R>> {
        let guard = self.active_workspace.read().await;
        if guard.is_none() {
            return None;
        }
        let id = guard.as_ref()?.id;
        let workspace_guard = RwLockReadGuard::map(guard, |opt| {
            opt.as_ref().map(|a| &a.this).unwrap() // This is safe because we checked for None above
        });

        Some(WorkspaceReadGuard {
            id,
            guard: workspace_guard,
        })
    }

    pub(crate) async fn workspace_with_context(
        &self,
        app_handle: AppHandle<R>,
    ) -> Option<(WorkspaceReadGuard<'_, R>, WorkspaceContext<R>)> {
        let guard = self.active_workspace.read().await;
        if guard.is_none() {
            return None;
        }

        let id = guard.as_ref()?.id;
        let context_state = guard.as_ref()?.context.clone();
        let workspace_guard = RwLockReadGuard::map(guard, |opt| {
            opt.as_ref().map(|a| &a.this).unwrap() // This is safe because we checked for None above
        });

        let context = WorkspaceContext::new(app_handle, context_state);
        Some((
            WorkspaceReadGuard {
                id,
                guard: workspace_guard,
            },
            context,
        ))
    }

    pub(crate) async fn workspace_with_context_mut(
        &self,
        app_handle: AppHandle<R>,
    ) -> Option<(WorkspaceWriteGuard<'_, R>, WorkspaceContext<R>)> {
        let guard = self.active_workspace.write().await;
        if guard.is_none() {
            return None;
        }

        let id = guard.as_ref()?.id;
        let context_state = guard.as_ref()?.context.clone();
        let workspace_guard = RwLockWriteGuard::map(guard, |opt| {
            opt.as_mut().map(|a| &mut a.this).unwrap() // This is safe because we checked for None above
        });

        let context = WorkspaceContext::new(app_handle, context_state);
        Some((
            WorkspaceWriteGuard {
                id,
                guard: workspace_guard,
            },
            context,
        ))
    }

    pub(crate) async fn activate_workspace<C: AnyAppContext<R>>(
        &self,
        ctx: &C,
        id: Uuid,
        workspace: Workspace<R>,
    ) -> Result<()> {
        let last_opened_at = Utc::now().timestamp();
        let workspaces = self.workspaces().await?;
        let mut workspaces_lock = workspaces.write().await;
        let mut descriptor = workspaces_lock
            .get(&id)
            .ok_or(WorkspaceServiceError::NotFound(id.to_string()))?
            .as_ref()
            .clone();

        descriptor.last_opened_at = Some(last_opened_at);

        workspaces_lock.insert(id, Arc::new(descriptor));
        drop(workspaces_lock);

        let mut active_workspace = self.active_workspace.write().await;
        *active_workspace = Some(ActiveWorkspace {
            id,
            this: workspace,
            context: Arc::new(RwLock::new(WorkspaceContextState::new())),
        });

        let item_store = self.global_storage.item_store();
        let id_str = id.to_string();
        let segkey = WORKSPACE_SEGKEY.join(id_str);
        let value = AnyValue::serialize(&WorkspaceInfoEntity { last_opened_at })?;
        PutItem::put(item_store.as_ref(), segkey, value)?;

        let workspace_id: ctxkeys::WorkspaceId = id.into();
        ctx.set_value(workspace_id);

        Ok(())
    }

    pub(crate) async fn deactivate_workspace<C: AnyAppContext<R>>(&self, ctx: &C) {
        let mut active_workspace = self.active_workspace.write().await;
        *active_workspace = None;

        ctx.remove_value::<ctxkeys::WorkspaceId>();
    }

    async fn workspaces(&self) -> WorkspaceServiceResult<&RwLock<WorkspaceMap>> {
        Ok(self
            .known_workspaces
            .get_or_try_init(|| async move {
                let mut workspaces: WorkspaceMap = HashMap::new();

                let restored_items = ListByPrefix::list_by_prefix(
                    self.global_storage.item_store().as_ref(),
                    WORKSPACE_SEGKEY.as_str().expect("invalid utf-8"),
                )
                .map_err(|e| WorkspaceServiceError::Storage(e.to_string()))?;
                let filtered_restored_items = restored_items.iter().filter_map(|(k, v)| {
                    let path = k.after(&WORKSPACE_SEGKEY);
                    if let Some(path) = path {
                        Some((path, v))
                    } else {
                        None
                    }
                });

                let mut restored_entities = HashMap::with_capacity(restored_items.len());
                for (segkey, value) in filtered_restored_items {
                    let encoded_name = match String::from_utf8(segkey.as_bytes().to_owned()) {
                        Ok(name) => name,
                        Err(_) => {
                            // TODO: logging
                            println!("failed to get the workspace {:?} name", segkey);
                            continue;
                        }
                    };

                    restored_entities.insert(encoded_name, value);
                }

                let mut read_dir = self
                    .fs
                    .read_dir(&self.abs_path)
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
                    let id = match Uuid::parse_str(&id_str) {
                        Ok(id) => id,
                        Err(_) => {
                            // TODO: logging
                            println!("failed to get the collection {:?} name", id_str);
                            continue;
                        }
                    };

                    let summary = Workspace::<R>::summary(self.fs.clone(), &entry.path())
                        .await
                        .map_err(|e| WorkspaceServiceError::Workspace(e.to_string()))?;

                    let restored_entity =
                        match restored_entities.remove(&id_str).map_or(Ok(None), |v| {
                            v.deserialize::<WorkspaceInfoEntity>().map(Some)
                        }) {
                            Ok(value) => value,
                            Err(_err) => {
                                // TODO: logging
                                println!("failed to get the workspace {:?} info", id_str);
                                continue;
                            }
                        };

                    workspaces.insert(
                        id,
                        WorkspaceDescriptor {
                            id,
                            name: summary.manifest.name,
                            abs_path: entry.path().into(),
                            last_opened_at: restored_entity.map(|v| v.last_opened_at),
                        }
                        .into(),
                    );
                }

                Ok::<_, WorkspaceServiceError>(RwLock::new(workspaces))
            })
            .await?)
    }
}
