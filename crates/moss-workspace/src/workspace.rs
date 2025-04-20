pub mod api;

use anyhow::{Context, Result};
use moss_collection::{
    collection::{Collection, CollectionCache},
    indexer::{self, IndexerHandle},
};
use moss_common::leased_slotmap::{LeasedSlotMap, ResourceKey};
use moss_environment::environment::{Environment, EnvironmentCache, VariableCache};
use moss_fs::{utils::decode_name, FileSystem};
use moss_storage::{workspace_storage::WorkspaceStorageImpl, WorkspaceStorage};
use moss_workbench::activity_indicator::ActivityIndicator;
use std::{collections::HashMap, future::Future, path::PathBuf, sync::Arc};
use tauri::{AppHandle, Runtime as TauriRuntime};
use tokio::sync::{mpsc, OnceCell, RwLock};

pub const COLLECTIONS_DIR: &'static str = "collections";
pub const ENVIRONMENTS_DIR: &str = "environments";

type CollectionSlot = (Collection, CollectionCache);
type CollectionMap = LeasedSlotMap<ResourceKey, CollectionSlot>;

type EnvironmentSlot = (Environment, EnvironmentCache);
type EnvironmentMap = LeasedSlotMap<ResourceKey, EnvironmentSlot>;

pub struct Workspace<R: TauriRuntime> {
    app_handle: AppHandle<R>,
    path: PathBuf,
    fs: Arc<dyn FileSystem>,
    workspace_storage: Arc<dyn WorkspaceStorage>,
    collections: OnceCell<RwLock<CollectionMap>>,
    environments: OnceCell<RwLock<EnvironmentMap>>,
    activity_indicator: ActivityIndicator<R>,
    indexer_handle: IndexerHandle,
}

impl<R: TauriRuntime> Workspace<R> {
    pub fn new(
        app_handle: AppHandle<R>,
        path: PathBuf,
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
            path,
            fs,
            workspace_storage: Arc::new(state_db_manager),
            collections: OnceCell::new(),
            environments: OnceCell::new(),
            indexer_handle,
            activity_indicator,
        })
    }

    async fn environments(&self) -> Result<&RwLock<EnvironmentMap>> {
        let result = self
            .environments
            .get_or_try_init(|| async move {
                let mut environments = LeasedSlotMap::new();

                if !self.path.join(ENVIRONMENTS_DIR).exists() {
                    return Ok(RwLock::new(environments));
                }

                let mut envs_from_fs = HashMap::new();
                let mut environment_dir =
                    self.fs.read_dir(&self.path.join(ENVIRONMENTS_DIR)).await?;
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
                let mut collections = LeasedSlotMap::new();

                if !self.path.join(COLLECTIONS_DIR).exists() {
                    return Ok::<_, anyhow::Error>(RwLock::new(collections));
                }

                // TODO: Support external collections with absolute path
                for (relative_path, collection_data) in self
                    .workspace_storage
                    .collection_store()
                    .list_collections()?
                {
                    let name = match relative_path.file_name() {
                        Some(name) => decode_name(&name.to_string_lossy().to_string())?,
                        None => {
                            // TODO: logging
                            println!("failed to get the collection {:?} name", relative_path);
                            continue;
                        }
                    };

                    // TODO:A self-healing mechanism needs to be implemented here.
                    // Collections that are found in the database but do not actually exist
                    // in the file system should be collected and deleted from the database in
                    // a parallel thread.

                    // TODO: implement is_external flag for relative/absolute path

                    let full_path = self.path.join(relative_path);
                    let collection =
                        Collection::new(full_path, self.fs.clone(), self.indexer_handle.clone())?;
                    let metadata = CollectionCache {
                        name,
                        order: collection_data.order,
                    };

                    collections.insert((collection, metadata));
                }

                Ok::<_, anyhow::Error>(RwLock::new(collections))
            })
            .await?;

        Ok(result)
    }

    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }

    pub async fn with_collection<T, Fut>(
        &self,
        key: ResourceKey,
        f: impl FnOnce(&Collection) -> Fut,
    ) -> Result<T>
    where
        Fut: Future<Output = Result<T>>,
    {
        let collections = self.collections().await?;
        let collections_lock = collections.read().await;

        let (collection, _cache) = collections_lock.get(key).context("Collection not found")?; // TODO: use operation error

        f(collection).await
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
