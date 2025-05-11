pub mod api;

use anyhow::{Context, Result};
use dashmap::DashMap;
use moss_activity_indicator::ActivityIndicator;
use moss_collection::{
    collection::Collection,
    indexer::{self, IndexerHandle},
};
use moss_common::{
    leased_slotmap::{LeasedSlotMap, ResourceKey},
    models::primitives::Identifier,
};
use moss_environment::environment::{Environment, EnvironmentCache, VariableCache};
use moss_fs::{FileSystem, utils::decode_name};
use moss_storage::{WorkspaceStorage, workspace_storage::WorkspaceStorageImpl};
use std::{
    collections::HashMap,
    ops::Deref,
    path::{Path, PathBuf},
    sync::{Arc, atomic::AtomicUsize},
};
use tauri::{AppHandle, Runtime as TauriRuntime};
use tokio::sync::{OnceCell, RwLock, mpsc};

use crate::models::types::CollectionInfo;

pub const COLLECTIONS_DIR: &'static str = "collections";
pub const ENVIRONMENTS_DIR: &str = "environments";

// pub struct CollectionInfoNew {
//     pub id: Identifier,
//     pub name: String,
//     pub order: Option<usize>,
// }

type CollectionSlot = (Collection, CollectionInfo);
// type CollectionMap = LeasedSlotMap<ResourceKey, CollectionSlot>;

type EnvironmentSlot = (Environment, EnvironmentCache);
type EnvironmentMap = LeasedSlotMap<ResourceKey, EnvironmentSlot>;

pub struct CollectionEntry {
    pub id: Identifier,
    pub name: String,
    pub display_name: String,
    pub order: Option<usize>,
    pub inner: Collection,
}

impl Deref for CollectionEntry {
    type Target = Collection;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

type CollectionMap = HashMap<Identifier, Arc<CollectionEntry>>;

pub struct Workspace<R: TauriRuntime> {
    #[allow(dead_code)]
    app_handle: AppHandle<R>,
    abs_path: Arc<Path>,
    fs: Arc<dyn FileSystem>,
    workspace_storage: Arc<dyn WorkspaceStorage>,
    collections: OnceCell<RwLock<CollectionMap>>,
    environments: OnceCell<RwLock<EnvironmentMap>>,
    #[allow(dead_code)]
    activity_indicator: ActivityIndicator<R>,
    indexer_handle: IndexerHandle,
    next_collection_entry_id: Arc<AtomicUsize>,
    next_collection_id: Arc<AtomicUsize>,
}

impl<R: TauriRuntime> Workspace<R> {
    pub fn new(
        app_handle: AppHandle<R>,
        path: Arc<Path>,
        fs: Arc<dyn FileSystem>,
        activity_indicator: ActivityIndicator<R>,
    ) -> Result<Self> {
        let state_db_manager = WorkspaceStorageImpl::new(&path)
            .context("Failed to open the workspace state database")?;

        let (tx, rx) = mpsc::unbounded_channel();
        let indexer_handle = IndexerHandle::new(tx);
        tauri::async_runtime::spawn({
            let fs_clone = Arc::clone(&fs);
            let activity_indicator_clone = activity_indicator.clone();

            async move {
                indexer::run(activity_indicator_clone, fs_clone, rx).await;
            }
        });

        Ok(Self {
            app_handle,
            abs_path: path,
            fs,
            workspace_storage: Arc::new(state_db_manager),
            collections: OnceCell::new(),
            environments: OnceCell::new(),
            indexer_handle,
            activity_indicator,
            next_collection_entry_id: Arc::new(AtomicUsize::new(0)),
            next_collection_id: Arc::new(AtomicUsize::new(0)),
        })
    }

    pub fn abs_path(&self) -> &Arc<Path> {
        &self.abs_path
    }

    pub(super) fn absolutize<P: AsRef<Path>>(&self, path: P) -> PathBuf {
        self.abs_path.join(path)
    }

    async fn environments(&self) -> Result<&RwLock<EnvironmentMap>> {
        let result = self
            .environments
            .get_or_try_init(|| async move {
                let mut environments = LeasedSlotMap::new();

                if !self.abs_path.join(ENVIRONMENTS_DIR).exists() {
                    return Ok(RwLock::new(environments));
                }

                let mut envs_from_fs = HashMap::new();
                let mut environment_dir = self
                    .fs
                    .read_dir(&self.abs_path.join(ENVIRONMENTS_DIR))
                    .await?;
                while let Some(entry) = environment_dir.next_entry().await? {
                    if entry.file_type().await?.is_dir() {
                        continue;
                    }

                    let path = entry.path();

                    if path.extension().map(|ext| ext == "json").unwrap_or(false) {
                        let environment_name =
                            path.file_name().unwrap().to_string_lossy().to_string(); // TODO: Is unwrap here is safe?

                        let environment = Environment::new(path, self.fs.clone()).await?;
                        envs_from_fs.insert(environment_name, environment);
                    }
                }

                let mut scan_result = self.workspace_storage.environment_store().scan()?;
                for (name, env) in envs_from_fs {
                    let environment_entity = scan_result.remove(&name);

                    let environment_cache = if let Some(environment_entity) = environment_entity {
                        EnvironmentCache {
                            decoded_name: name, // TODO: decode name
                            order: environment_entity.order,
                            variables_cache: environment_entity
                                .local_values
                                .into_iter()
                                .map(|(name, state_entity)| {
                                    VariableCache::try_from(state_entity).map(|cache| (name, cache))
                                })
                                .collect::<Result<HashMap<_, _>, _>>()?,
                        }
                    } else {
                        EnvironmentCache {
                            decoded_name: name, // TODO: decode name,
                            order: None,
                            variables_cache: HashMap::new(),
                        }
                    };

                    environments.insert((env, environment_cache));
                }

                Ok::<_, anyhow::Error>(RwLock::new(environments))
            })
            .await?;

        Ok(result)
    }

    pub async fn collections(&self) -> Result<&RwLock<CollectionMap>> {
        let result = self
            .collections
            .get_or_try_init(|| async move {
                let mut collections = HashMap::new();

                if !self.abs_path.join(COLLECTIONS_DIR).exists() {
                    return Ok(RwLock::new(collections));
                }

                // TODO: Support external collections with absolute path
                for (path, collection_data) in self
                    .workspace_storage
                    .collection_store()
                    .list_collection()?
                {
                    debug_assert!(path.is_relative());

                    let (display_name, encoded_name) = match path.file_name() {
                        Some(name) => {
                            let name = name.to_string_lossy().to_string();

                            (decode_name(&name)?, name)
                        }
                        None => {
                            // TODO: logging
                            println!("failed to get the collection {:?} name", path);
                            continue;
                        }
                    };

                    // TODO: A self-healing mechanism needs to be implemented here.
                    // Collections that are found in the database but do not actually exist
                    // in the file system should be collected and deleted from the database in
                    // a parallel thread.

                    let id = Identifier::new(&self.next_collection_id);
                    let abs_path: Arc<Path> = self.abs_path.join(path).into();
                    let collection = Collection::new(
                        abs_path.to_path_buf(), // FIXME: change to Arc<Path> in Collection::new
                        self.fs.clone(),
                        self.indexer_handle.clone(),
                        self.next_collection_entry_id.clone(),
                    )?;
                    collections.insert(
                        id,
                        CollectionEntry {
                            id,
                            name: encoded_name,
                            display_name,
                            order: collection_data.order,
                            inner: collection,
                        }
                        .into(),
                    );
                }

                Ok::<_, anyhow::Error>(RwLock::new(collections))
            })
            .await?;

        Ok(result)
    }
}

impl<R: TauriRuntime> Workspace<R> {
    #[cfg(test)]
    pub fn truncate(&self) -> Result<()> {
        // let collection_store = self.workspace_storage.collection_store();

        // let (mut txn, table) = collection_store.begin_write()?;
        // table.truncate(&mut txn)?;
        // Ok(txn.commit()?)
        todo!()
    }
}
