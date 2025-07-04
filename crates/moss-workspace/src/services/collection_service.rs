use anyhow::{Context as _, Result};
use derive_more::{Deref, DerefMut};
use futures::Stream;
use moss_applib::ServiceMarker;
use moss_bindingutils::primitives::{ChangePath, ChangeString};
use moss_collection::{self as collection, Collection as CollectionHandle, CollectionBuilder};
use moss_common::api::OperationError;
use moss_db::primitives::AnyValue;
use moss_fs::{FileSystem, RemoveOptions};
use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    sync::Arc,
};
use thiserror::Error;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::{
    dirs,
    services::{PublicServiceMarker, storage_service::StorageService},
    storage::segments::COLLECTION_SEGKEY,
};

#[derive(Error, Debug)]
pub enum CollectionError {
    #[error("invalid input: {0}")]
    InvalidInput(String),

    #[error("invalid kind: {0}")]
    InvalidKind(String),

    #[error("collection already exists: {0}")]
    AlreadyExists(String),

    #[error("collection is not found: {0}")]
    NotFound(String),

    #[error("io error: {0}")]
    Io(String),

    #[error("internal error: {0}")]
    Internal(String),

    #[error("unknown error: {0}")]
    Unknown(#[from] anyhow::Error),
}

impl From<moss_db::common::DatabaseError> for CollectionError {
    fn from(error: moss_db::common::DatabaseError) -> Self {
        CollectionError::Internal(error.to_string())
    }
}

impl From<serde_json::Error> for CollectionError {
    fn from(error: serde_json::Error) -> Self {
        CollectionError::Internal(error.to_string())
    }
}

impl From<CollectionError> for OperationError {
    fn from(error: CollectionError) -> Self {
        match error {
            CollectionError::InvalidInput(err) => OperationError::InvalidInput(err),
            CollectionError::InvalidKind(err) => OperationError::InvalidInput(err),
            CollectionError::AlreadyExists(err) => OperationError::AlreadyExists(err),
            CollectionError::NotFound(err) => OperationError::NotFound(err),
            CollectionError::Unknown(err) => OperationError::Unknown(err),
            CollectionError::Io(err) => OperationError::Internal(err.to_string()),
            CollectionError::Internal(err) => OperationError::Internal(err.to_string()),
        }
    }
}

type CollectionResult<T> = std::result::Result<T, CollectionError>;

pub(crate) struct CollectionItemUpdateParams {
    pub name: Option<String>,
    pub order: Option<usize>,
    pub expanded: Option<bool>,
    pub repository: Option<ChangeString>,
    pub icon: Option<ChangePath>,
}

pub(crate) struct CollectionItemCreateParams {
    pub name: String,
    pub order: usize,
    pub repository: Option<String>,
    pub external_path: Option<PathBuf>,
    pub icon_path: Option<PathBuf>,
}

#[derive(Deref, DerefMut)]
struct CollectionItem {
    pub id: Uuid,
    pub order: Option<usize>,

    #[deref]
    #[deref_mut]
    pub handle: Arc<CollectionHandle>,
}

pub(crate) struct CollectionItemDescription {
    pub id: Uuid,
    pub name: String,
    pub order: Option<usize>,
    pub expanded: bool,
    pub repository: Option<String>,
    pub icon_path: Option<PathBuf>,
    pub abs_path: Arc<Path>,
    pub external_path: Option<PathBuf>,
}

#[derive(Default)]
struct ServiceState {
    collections: HashMap<Uuid, CollectionItem>,
    expanded_items: HashSet<Uuid>,
}

pub struct CollectionService {
    abs_path: Arc<Path>,
    fs: Arc<dyn FileSystem>,
    storage: Arc<StorageService>, // TODO: should be a trait
    state: Arc<RwLock<ServiceState>>,
}

impl ServiceMarker for CollectionService {}
impl PublicServiceMarker for CollectionService {}

impl CollectionService {
    pub async fn new(
        abs_path: Arc<Path>,
        fs: Arc<dyn FileSystem>,
        storage: Arc<StorageService>,
    ) -> CollectionResult<Self> {
        let expanded_items = if let Ok(expanded_items) = storage.get_expanded_items::<Uuid>() {
            expanded_items.collect::<HashSet<_>>()
        } else {
            HashSet::new()
        };

        let collections = restore_collections(&abs_path, &fs, &storage).await?;

        Ok(Self {
            abs_path,
            fs,
            storage,
            state: Arc::new(RwLock::new(ServiceState {
                collections,
                expanded_items,
            })),
        })
    }

    fn absolutize<P: AsRef<Path>>(&self, path: P) -> PathBuf {
        self.abs_path.join(dirs::COLLECTIONS_DIR).join(path)
    }

    pub async fn collection(&self, id: Uuid) -> CollectionResult<Arc<CollectionHandle>> {
        let state_lock = self.state.read().await;
        let item = state_lock
            .collections
            .get(&id)
            .ok_or(CollectionError::NotFound(id.to_string()))?;

        Ok(item.handle.clone())
    }

    pub(crate) async fn create_collection(
        &self,
        id: Uuid,
        params: CollectionItemCreateParams,
    ) -> CollectionResult<CollectionItemDescription> {
        let id_str = id.to_string();
        let abs_path: Arc<Path> = self.absolutize(id_str).into();
        if abs_path.exists() {
            return Err(CollectionError::AlreadyExists(
                abs_path.to_path_buf().to_string_lossy().to_string(),
            ));
        }

        self.fs
            .create_dir(&abs_path)
            .await
            .context("Failed to create the collection directory")?;

        let collection = {
            let storage = Arc::new(collection::services::StorageService::new(&abs_path)?);
            let worktree = collection::services::WorktreeService::new(
                abs_path.clone(),
                self.fs.clone(),
                storage.clone(),
            );
            CollectionBuilder::new(self.fs.clone())
                .with_service::<collection::services::StorageService>(storage)
                .with_service(worktree)
                .create(collection::builder::CreateParams {
                    name: Some(params.name.to_owned()),
                    internal_abs_path: abs_path.clone(),
                    external_abs_path: params.external_path.as_deref().map(|p| p.to_owned().into()),
                    repository: params.repository.to_owned(),
                    icon_path: params.icon_path.to_owned(),
                })
                .await
                .map_err(|e| CollectionError::Internal(e.to_string()))?
        };
        let icon_path = collection.icon_path();

        // let on_did_change = collection.on_did_change().subscribe(|_event| async move {

        //     // TODO: Save in the database whether the collection was collapsed/expanded
        // });
        // ctx.subscribe(Subscribe::OnCollectionDidChange(id, on_did_change))
        //     .await;

        let mut state_lock = self.state.write().await;
        state_lock.expanded_items.insert(id);
        state_lock.collections.insert(
            id,
            CollectionItem {
                id,
                order: Some(params.order),
                handle: Arc::new(collection),
            },
        );

        {
            let mut txn = self.storage.begin_write()?;

            self.storage
                .put_item_order_txn(&mut txn, id, params.order)?;
            self.storage
                .put_expanded_items_txn(&mut txn, &state_lock.expanded_items)?;

            txn.commit()?;
        }

        Ok(CollectionItemDescription {
            id,
            name: params.name,
            order: Some(params.order),
            expanded: true,
            repository: params.repository,
            icon_path,
            abs_path,
            external_path: params.external_path,
        })
    }

    pub(crate) async fn delete_collection(
        &self,
        id: Uuid,
    ) -> CollectionResult<Option<CollectionItemDescription>> {
        let id_str = id.to_string();
        let abs_path = self.absolutize(id_str);

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
                .context("Failed to delete collection from file system")?;
        }

        let mut state_lock = self.state.write().await;
        let item = state_lock.collections.remove(&id);
        state_lock.expanded_items.remove(&id);

        {
            let mut txn = self.storage.begin_write()?;

            self.storage
                .remove_item_metadata_txn(&mut txn, COLLECTION_SEGKEY.join(&id.to_string()))?;
            self.storage
                .put_expanded_items_txn(&mut txn, &state_lock.expanded_items)?;

            txn.commit()?;
        }

        if let Some(item) = item {
            let manifest = item.handle.manifest().await;

            Ok(Some(CollectionItemDescription {
                id,
                name: manifest.name,
                order: item.order,
                expanded: false,
                repository: manifest.repository,
                icon_path: item.icon_path(),
                abs_path: item.abs_path().clone(),
                external_path: None, // TODO: implement
            }))
        } else {
            Ok(None)
        }
    }

    pub(crate) async fn update_collection(
        &self,
        id: Uuid,
        params: CollectionItemUpdateParams,
    ) -> CollectionResult<()> {
        let mut state_lock = self.state.write().await;
        let item = state_lock
            .collections
            .get_mut(&id)
            .ok_or(CollectionError::NotFound(id.to_string()))?;

        let mut txn = self.storage.begin_write()?;
        if let Some(order) = params.order {
            item.order = Some(order);
            self.storage.put_item_order_txn(&mut txn, id, order)?;
        }

        item.modify(collection::ModifyParams {
            name: params.name,
            repository: params.repository,
            icon: params.icon,
        })
        .await
        .map_err(|e| CollectionError::Internal(e.to_string()))?;

        if let Some(expanded) = params.expanded {
            if expanded {
                state_lock.expanded_items.insert(id);
            } else {
                state_lock.expanded_items.remove(&id);
            }

            self.storage
                .put_expanded_items_txn(&mut txn, &state_lock.expanded_items)?;
        }

        Ok(())
    }

    pub(crate) fn list_collections(&self) -> impl Stream<Item = CollectionItemDescription> + '_ {
        let state = self.state.clone();

        async_stream::stream! {
            let state_lock = state.read().await;
            for (id, item) in state_lock.collections.iter() {
                let manifest = item.handle.manifest().await;
                let expanded = state_lock.expanded_items.contains(id);

                yield CollectionItemDescription {
                    id: item.id,
                    name: manifest.name,
                    order: item.order,
                    expanded,
                    repository: manifest.repository,
                    icon_path: item.handle.icon_path(),
                    abs_path: item.handle.abs_path().clone(),
                    external_path: None, // TODO: implement
                };
            }
        }
    }
}

async fn restore_collections(
    abs_path: &Path,
    fs: &Arc<dyn FileSystem>,
    storage: &Arc<StorageService>,
) -> Result<HashMap<Uuid, CollectionItem>> {
    let dir_abs_path = abs_path.join(dirs::COLLECTIONS_DIR);
    if !dir_abs_path.exists() {
        return Ok(HashMap::new());
    }

    let mut collections = Vec::new();
    let mut read_dir = fs.read_dir(&dir_abs_path).await?;
    while let Some(entry) = read_dir.next_entry().await? {
        if !entry
            .file_type()
            .await
            .context("Failed to get the file type")?
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

        let collection = {
            let collection_abs_path: Arc<Path> = entry.path().to_owned().into();
            let storage = Arc::new(collection::services::StorageService::new(
                &collection_abs_path,
            )?);
            let worktree = collection::services::WorktreeService::new(
                collection_abs_path.clone(),
                fs.clone(),
                storage.clone(),
            );
            CollectionBuilder::new(fs.clone())
                .with_service::<collection::services::StorageService>(storage)
                .with_service(worktree)
                .load(collection::builder::LoadParams {
                    internal_abs_path: collection_abs_path,
                })
                .await?
        };

        collections.push((id, collection));
    }

    let metadata = storage
        .list_items_metadata(COLLECTION_SEGKEY.to_segkey_buf())?
        .collect::<HashMap<_, _>>();

    let mut result = HashMap::new();
    for (id, collection) in collections {
        let segkey_prefix = COLLECTION_SEGKEY.join(&id.to_string());

        let order = metadata
            .get(&segkey_prefix.join("order"))
            .map(|v| AnyValue::from(v.to_owned()).into());

        result.insert(
            id,
            CollectionItem {
                id,
                order,
                handle: Arc::new(collection),
            },
        );
    }

    Ok(result)
}
