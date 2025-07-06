use anyhow::{Context as _, Result};
use chrono::Utc;
use derive_more::{Deref, DerefMut};
use moss_activity_indicator::ActivityIndicator;
use moss_applib::{PublicServiceMarker, ServiceMarker};
use moss_common::api::OperationError;
use moss_db::DatabaseError;
use moss_fs::{FileSystem, RemoveOptions};
use moss_workspace::{
    Workspace,
    builder::{WorkspaceBuilder, WorkspaceCreateParams, WorkspaceLoadParams},
    context::{WorkspaceContext, WorkspaceContextState},
    services::{
        collection_service::CollectionService, layout_service::LayoutService,
        storage_service::StorageService as WorkspaceStorageService,
    },
    workspace::WorkspaceModifyParams,
};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};
use tauri::{AppHandle, Runtime as TauriRuntime};
use thiserror::Error;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::{
    context::{AnyAppContext, ctxkeys},
    dirs,
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
pub(crate) struct ActiveWorkspace<R: TauriRuntime> {
    id: Uuid,

    #[deref]
    #[deref_mut]
    handle: Arc<Workspace<R>>,
    context: Arc<RwLock<WorkspaceContextState>>,
}

pub(crate) struct WorkspaceItemCreateParams {
    pub name: String,
}

pub(crate) struct WorkspaceItemUpdateParams {
    pub name: Option<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct WorkspaceItem {
    pub id: Uuid,
    pub name: String,
    pub abs_path: Arc<Path>,
    pub last_opened_at: Option<i64>,
}

pub(crate) struct WorkspaceItemDescription {
    pub id: Uuid,
    pub name: String,
    pub abs_path: Arc<Path>,
    pub last_opened_at: Option<i64>,
    pub active: bool,
}

type WorkspaceMap = HashMap<Uuid, WorkspaceItem>;

#[derive(Default)]
struct ServiceState<R: TauriRuntime> {
    known_workspaces: WorkspaceMap,
    active_workspace: Option<ActiveWorkspace<R>>,
}

pub struct WorkspaceService<R: TauriRuntime> {
    /// The absolute path to the workspaces directory
    abs_path: Arc<Path>,
    fs: Arc<dyn FileSystem>,
    // global_storage: Arc<dyn GlobalStorage>,
    storage: Arc<StorageService>, // TODO: should be a trait
    state: Arc<RwLock<ServiceState<R>>>,
    // known_workspaces: OnceCell<RwLock<WorkspaceMap>>,
    // active_workspace: RwLock<Option<ActiveWorkspace<R>>>,
}

impl<R: TauriRuntime> ServiceMarker for WorkspaceService<R> {}
impl<R: TauriRuntime> PublicServiceMarker for WorkspaceService<R> {}

impl<R: TauriRuntime> WorkspaceService<R> {
    pub async fn new(
        storage_service: Arc<StorageService>,
        fs: Arc<dyn FileSystem>,
        abs_path: &Path,
    ) -> WorkspaceServiceResult<Self> {
        debug_assert!(abs_path.is_absolute());
        let abs_path: Arc<Path> = abs_path.join(dirs::WORKSPACES_DIR).into();
        debug_assert!(abs_path.exists());

        let known_workspaces =
            restore_known_workspaces::<R>(&abs_path, &fs, &storage_service).await?;

        Ok(Self {
            fs,
            storage: storage_service,
            abs_path,
            state: Arc::new(RwLock::new(ServiceState {
                known_workspaces,
                active_workspace: None,
            })),
            // known_workspaces: OnceCell::new(),
            // active_workspace: RwLock::new(None),
        })
    }

    pub fn absolutize(&self, path: impl AsRef<Path>) -> PathBuf {
        self.abs_path.join(path)
    }

    pub(crate) async fn list_workspaces(
        &self,
    ) -> WorkspaceServiceResult<Vec<WorkspaceItemDescription>> {
        let state_lock = self.state.read().await;
        let active_workspace_id = state_lock.active_workspace.as_ref().map(|a| a.id);

        let workspaces = state_lock
            .known_workspaces
            .values()
            .map(|item| WorkspaceItemDescription {
                id: item.id,
                name: item.name.clone(),
                abs_path: item.abs_path.clone(),
                last_opened_at: item.last_opened_at,
                active: Some(item.id) == active_workspace_id,
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
            .insert(descriptor.id, descriptor);

        Ok(())
    }

    pub(crate) async fn delete_workspace<C: AnyAppContext<R>>(
        &self,
        ctx: &C,
        id: Uuid,
    ) -> WorkspaceServiceResult<()> {
        let (active_workspace_id, item) = {
            let state_lock = self.state.read().await;

            let active_workspace_id = state_lock.active_workspace.as_ref().map(|a| a.id);
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
            let id_str = id.to_string();

            // Only try to remove from database if it exists (ignore error if not found)
            let _ = self
                .storage
                .remove_all_by_prefix(&segkey_workspace(&id_str).to_string())
                .map_err(|e| WorkspaceServiceError::Storage(e.to_string()));
        }

        if active_workspace_id != Some(item.id) {
            return Ok(());
        }

        Ok(self.deactivate_workspace(ctx).await?)
    }

    // pub(crate) async fn load_workspace(
    //     &self,
    //     id: Uuid,
    //     activity_indicator: ActivityIndicator<R>,
    // ) -> WorkspaceServiceResult<(Workspace<R>, WorkspaceItem)> {
    //     let state_lock = self.state.write().await;
    //     let descriptor = if let Some(d) = state_lock.known_workspaces.get(&id) {
    //         d.clone()
    //     } else {
    //         return Err(WorkspaceServiceError::NotFound(id.to_string()));
    //     };

    //     if !descriptor.abs_path.exists() {
    //         return Err(WorkspaceServiceError::NotFound(
    //             descriptor.abs_path.to_string_lossy().to_string(),
    //         ));
    //     }

    //     let active_workspace_id = state_lock.active_workspace.as_ref().map(|a| a.id);
    //     if active_workspace_id == Some(id) {
    //         return Err(WorkspaceServiceError::AlreadyLoaded(id.to_string()));
    //     }

    //     let storage_service: Arc<StorageService> = StorageService::new(&descriptor.abs_path)
    //         .context("Failed to load the storage service")
    //         .map_err(|e| WorkspaceServiceError::Workspace(e.to_string()))?
    //         .into();

    //     let collection_service = CollectionService::new(
    //         descriptor.abs_path.clone(),
    //         self.fs.clone(),
    //         storage_service.clone(),
    //     )
    //     .await
    //     // .context("Failed to load the collection service")
    //     .map_err(|e| WorkspaceServiceError::Workspace(e.to_string()))?;

    //     let layout_service = LayoutService::new(storage_service.clone());

    //     let workspace = WorkspaceBuilder::new(self.fs.clone())
    //         .with_service::<StorageService>(storage_service.clone())
    //         .with_service(collection_service)
    //         .with_service(layout_service)
    //         .load(
    //             WorkspaceLoadParams {
    //                 abs_path: descriptor.abs_path.clone(),
    //             },
    //             activity_indicator,
    //         )
    //         .await
    //         .context("Failed to load the workspace")
    //         .map_err(|e| WorkspaceServiceError::Workspace(e.to_string()))?;

    //     Ok((workspace, descriptor))
    // }

    pub(crate) async fn create_workspace(
        &self,
        id: Uuid,
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

        // let storage_service: Arc<StorageService> = StorageService::new(&abs_path)
        //     .context("Failed to load the storage service")
        //     .map_err(|e| WorkspaceServiceError::Workspace(e.to_string()))?
        //     .into();

        // let collection_service =
        //     CollectionService::new(abs_path.clone(), self.fs.clone(), storage_service.clone())
        //         .await
        //         // .context("Failed to load the collection service")
        //         .map_err(|e| WorkspaceServiceError::Workspace(e.to_string()))?;

        // let layout_service = LayoutService::new(storage_service.clone());

        // let workspace = WorkspaceBuilder::new(self.fs.clone())
        //     .with_service::<StorageService>(storage_service.clone())
        //     .with_service(collection_service)
        //     .with_service(layout_service)
        //     .create(
        //         WorkspaceCreateParams {
        //             name: name.to_string(),
        //             abs_path: abs_path.clone(),
        //         },
        //         activity_indicator,
        //     )
        //     .await
        //     .context("Failed to create the workspace")
        //     .map_err(|e| WorkspaceServiceError::Workspace(e.to_string()))?;

        WorkspaceBuilder::initialize(
            self.fs.clone(),
            WorkspaceCreateParams {
                name: params.name.clone(),
                abs_path: abs_path.clone(),
            },
        )
        .await
        .context("Failed to initialize the workspace")
        .map_err(|e| WorkspaceServiceError::Workspace(e.to_string()))?;

        state_lock.known_workspaces.insert(
            id,
            WorkspaceItem {
                id,
                name: params.name.clone(),
                last_opened_at: None,
                abs_path: Arc::clone(&abs_path),
            },
        );

        Ok(WorkspaceItemDescription {
            id,
            name: params.name,
            abs_path: Arc::clone(&abs_path),
            last_opened_at: None,
            active: false,
        })
    }

    // TODO: remove this after we remove the describe_workbench_state api
    pub(crate) async fn workspace(&self) -> Option<Arc<Workspace<R>>> {
        let state_lock = self.state.read().await;
        if state_lock.active_workspace.is_none() {
            return None;
        }
        // let id = guard.as_ref()?.id;
        // let workspace_guard = RwLockReadGuard::map(guard, |opt| {
        //     opt.as_ref().map(|a| &a.handle).unwrap() // This is safe because we checked for None above
        // });

        Some(state_lock.active_workspace.as_ref()?.handle.clone())
    }

    pub async fn workspace_with_context(
        &self,
        app_handle: AppHandle<R>,
    ) -> Option<(Arc<Workspace<R>>, WorkspaceContext<R>)> {
        let state_lock = self.state.read().await;
        if state_lock.active_workspace.is_none() {
            return None;
        }

        // let id = state_lock.active_workspace.as_ref()?.id;
        let context_state = state_lock.active_workspace.as_ref()?.context.clone();
        // let workspace_guard = RwLockReadGuard::map(state_lock.active_workspace.as_ref(), |opt| {
        //     opt.as_ref().map(|a| &a.handle).unwrap() // This is safe because we checked for None above
        // });

        let context = WorkspaceContext::new(app_handle, context_state);
        Some((
            state_lock.active_workspace.as_ref()?.handle.clone(),
            context,
        ))
    }

    pub(crate) async fn activate_workspace<C: AnyAppContext<R>>(
        &self,
        ctx: &C,
        id: Uuid,
        activity_indicator: ActivityIndicator<R>,
        // workspace: Workspace<R>,
    ) -> WorkspaceServiceResult<WorkspaceItemDescription> {
        let mut state_lock = self.state.write().await;
        let item = state_lock
            .known_workspaces
            .get_mut(&id)
            .ok_or(WorkspaceServiceError::NotFound(id.to_string()))?;

        let last_opened_at = Utc::now().timestamp();
        let name = item.name.clone();
        let abs_path: Arc<Path> = self.absolutize(&id.to_string()).into();
        let storage_service: Arc<WorkspaceStorageService> = WorkspaceStorageService::new(&abs_path)
            .context("Failed to load the storage service")
            .map_err(|e| WorkspaceServiceError::Workspace(e.to_string()))?
            .into();

        let collection_service =
            CollectionService::new(abs_path.clone(), self.fs.clone(), storage_service.clone())
                .await
                // .context("Failed to load the collection service")
                .map_err(|e| WorkspaceServiceError::Workspace(e.to_string()))?;

        let layout_service = LayoutService::new(storage_service.clone());

        let workspace = WorkspaceBuilder::new(self.fs.clone())
            .with_service::<WorkspaceStorageService>(storage_service.clone())
            .with_service(collection_service)
            .with_service(layout_service)
            .load(
                WorkspaceLoadParams {
                    abs_path: abs_path.clone(),
                },
                activity_indicator,
            )
            .await
            .context("Failed to create the workspace")
            .map_err(|e| WorkspaceServiceError::Workspace(e.to_string()))?;

        item.last_opened_at = Some(last_opened_at);
        state_lock.active_workspace = Some(ActiveWorkspace {
            id,
            handle: Arc::new(workspace),
            context: Arc::new(RwLock::new(WorkspaceContextState::new())),
        });

        let id_str = id.to_string();

        {
            let mut txn = self.storage.begin_write()?;

            self.storage
                .put_last_active_workspace_txn(&mut txn, &id_str)?;
            self.storage
                .put_last_opened_at_txn(&mut txn, &id_str, last_opened_at)?;

            txn.commit()?;
        }

        // let item_store = self.global_storage.item_store();
        // let id_str = id.to_string();
        // let segkey = SEGKEY_WORKSPACE.join(id_str);
        // let value = AnyValue::serialize(&WorkspaceInfoEntity { last_opened_at })?;
        // PutItem::put(item_store.as_ref(), segkey, value)?;

        let workspace_id: ctxkeys::WorkspaceId = id.into();
        ctx.set_value(workspace_id);

        Ok(WorkspaceItemDescription {
            id,
            name,
            abs_path: Arc::clone(&abs_path),
            last_opened_at: Some(last_opened_at),
            active: true,
        })
    }

    pub(crate) async fn deactivate_workspace<C: AnyAppContext<R>>(
        &self,
        ctx: &C,
    ) -> WorkspaceServiceResult<()> {
        let mut state_lock = self.state.write().await;
        state_lock.active_workspace = None;

        self.storage.remove_last_active_workspace()?;

        ctx.remove_value::<ctxkeys::WorkspaceId>();

        Ok(())
    }

    // async fn workspaces(&self) -> WorkspaceServiceResult<&RwLock<WorkspaceMap>> {
    //     Ok(self
    //         .known_workspaces
    //         .get_or_try_init(|| async move {
    //             let mut workspaces: WorkspaceMap = HashMap::new();

    //             let restored_items = ListByPrefix::list_by_prefix(
    //                 self.global_storage.item_store().as_ref(),
    //                 WORKSPACE_SEGKEY.as_str().expect("invalid utf-8"),
    //             )
    //             .map_err(|e| WorkspaceServiceError::Storage(e.to_string()))?;
    //             let filtered_restored_items = restored_items.iter().filter_map(|(k, v)| {
    //                 let path = k.after(&WORKSPACE_SEGKEY);
    //                 if let Some(path) = path {
    //                     Some((path, v))
    //                 } else {
    //                     None
    //                 }
    //             });

    //             let mut restored_entities = HashMap::with_capacity(restored_items.len());
    //             for (segkey, value) in filtered_restored_items {
    //                 let encoded_name = match String::from_utf8(segkey.as_bytes().to_owned()) {
    //                     Ok(name) => name,
    //                     Err(_) => {
    //                         // TODO: logging
    //                         println!("failed to get the workspace {:?} name", segkey);
    //                         continue;
    //                     }
    //                 };

    //                 restored_entities.insert(encoded_name, value);
    //             }

    //             let mut read_dir = self
    //                 .fs
    //                 .read_dir(&self.abs_path)
    //                 .await
    //                 .map_err(|e| WorkspaceServiceError::Io(e.to_string()))?;

    //             while let Some(entry) = read_dir
    //                 .next_entry()
    //                 .await
    //                 .map_err(|e| WorkspaceServiceError::Io(e.to_string()))?
    //             {
    //                 if !entry
    //                     .file_type()
    //                     .await
    //                     .map_err(|e| WorkspaceServiceError::Io(e.to_string()))?
    //                     .is_dir()
    //                 {
    //                     continue;
    //                 }

    //                 let id_str = entry.file_name().to_string_lossy().to_string();
    //                 let id = match Uuid::parse_str(&id_str) {
    //                     Ok(id) => id,
    //                     Err(_) => {
    //                         // TODO: logging
    //                         println!("failed to get the collection {:?} name", id_str);
    //                         continue;
    //                     }
    //                 };

    //                 let summary = Workspace::<R>::summary(self.fs.clone(), &entry.path())
    //                     .await
    //                     .map_err(|e| WorkspaceServiceError::Workspace(e.to_string()))?;

    //                 let restored_entity =
    //                     match restored_entities.remove(&id_str).map_or(Ok(None), |v| {
    //                         v.deserialize::<WorkspaceInfoEntity>().map(Some)
    //                     }) {
    //                         Ok(value) => value,
    //                         Err(_err) => {
    //                             // TODO: logging
    //                             println!("failed to get the workspace {:?} info", id_str);
    //                             continue;
    //                         }
    //                     };

    //                 workspaces.insert(
    //                     id,
    //                     WorkspaceDescriptor {
    //                         id,
    //                         name: summary.manifest.name,
    //                         abs_path: entry.path().into(),
    //                         last_opened_at: restored_entity.map(|v| v.last_opened_at),
    //                     }
    //                     .into(),
    //                 );
    //             }

    //             Ok::<_, WorkspaceServiceError>(RwLock::new(workspaces))
    //         })
    //         .await?)
    // }
}

// TODO: These methods might later be moved into a wrapper around this service for integration tests
impl<R: TauriRuntime> WorkspaceService<R> {
    pub async fn is_workspace_open(&self) -> Option<Uuid> {
        let state_lock = self.state.read().await;
        state_lock.active_workspace.as_ref().map(|a| a.id)
    }
}

async fn restore_known_workspaces<R: TauriRuntime>(
    abs_path: &Path,
    fs: &Arc<dyn FileSystem>,
    storage_service: &Arc<StorageService>,
) -> WorkspaceServiceResult<WorkspaceMap> {
    let mut workspaces = HashMap::new();

    let restored_items =
        storage_service.list_all_by_prefix(SEGKEY_WORKSPACE.as_str().expect("invalid utf-8"))?;

    // let restored_items = ListByPrefix::list_by_prefix(
    //     global_storage.item_store().as_ref(),
    //     SEGKEY_WORKSPACE.as_str().expect("invalid utf-8"),
    // )
    // .map_err(|e| WorkspaceServiceError::Storage(e.to_string()))?;

    // let filtered_restored_items = restored_items.iter().filter_map(|(k, v)| {
    //     let path = k.after(&SEGKEY_WORKSPACE);
    //     if let Some(path) = path {
    //         Some((path, v))
    //     } else {
    //         None
    //     }
    // });

    // let mut restored_entities = HashMap::with_capacity(restored_items.len());
    // for (segkey, value) in filtered_restored_items {
    //     let encoded_name = match String::from_utf8(segkey.as_bytes().to_owned()) {
    //         Ok(name) => name,
    //         Err(_) => {
    //             // TODO: logging
    //             println!("failed to get the workspace {:?} name", segkey);
    //             continue;
    //         }
    //     };

    //     restored_entities.insert(encoded_name, value);
    // }

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
        let id = match Uuid::parse_str(&id_str) {
            Ok(id) => id,
            Err(_) => {
                // TODO: logging
                println!("failed to get the collection {:?} name", id_str);
                continue;
            }
        };

        let summary = Workspace::<R>::summary(fs.clone(), &entry.path())
            .await
            .map_err(|e| WorkspaceServiceError::Workspace(e.to_string()))?;

        // let collection_restored_item = restored_items
        //     .iter()
        //     .filter(|(k, _)| {
        //         k.after(&SEGKEY_WORKSPACE).map_or(false, |p| p == id_str)
        //     })
        //     .next();

        let filtered_items = restored_items
            .iter()
            .filter(|(key, _)| key.starts_with(&segkey_workspace(&id_str)))
            .collect::<HashMap<_, _>>();

        let last_opened_at = filtered_items
            .get(&segkey_last_opened_at(&id_str))
            .map(|v| {
                v.deserialize::<i64>()
                    .map_err(|e| WorkspaceServiceError::Storage(e.to_string()))
            })
            .transpose()?;

        // let restored_entity = match restored_entities.remove(&id_str).map_or(Ok(None), |v| {
        //     v.deserialize::<WorkspaceInfoEntity>().map(Some)
        // }) {
        //     Ok(value) => value,
        //     Err(_err) => {
        //         // TODO: logging
        //         println!("failed to get the workspace {:?} info", id_str);
        //         continue;
        //     }
        // };

        workspaces.insert(
            id,
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
