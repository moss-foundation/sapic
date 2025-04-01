pub mod api;

mod error;
pub use error::*;

use crate::storage::state_db_manager::StateDbManagerImpl;
use crate::storage::StateDbManager;
use anyhow::{anyhow, Context, Result};
use moss_collection::collection::{Collection, CollectionCache};
use moss_common::leased_slotmap::{LeasedSlotMap, ResourceKey};
use moss_environment::environment::{Environment, EnvironmentCache, VariableCache};
use moss_fs::{utils::decode_directory_name, FileSystem};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::{OnceCell, RwLock};

pub const COLLECTIONS_DIR: &'static str = "collections";
pub const ENVIRONMENTS_DIR: &str = "environments";

type CollectionSlot = (Collection, CollectionCache);
type CollectionMap = LeasedSlotMap<ResourceKey, CollectionSlot>;

type EnvironmentSlot = (Environment, EnvironmentCache);
type EnvironmentMap = LeasedSlotMap<ResourceKey, EnvironmentSlot>;

pub struct Workspace {
    path: PathBuf,
    fs: Arc<dyn FileSystem>,
    // We have to use Option so that we can temporarily drop it
    // TODO: implement is_external flag for absolute/relative path
    // Right now, we are storing relative paths in the db_manager
    state_db_manager: Option<Arc<dyn StateDbManager>>,
    collections: OnceCell<RwLock<CollectionMap>>,
    environments: OnceCell<RwLock<EnvironmentMap>>,
}

impl Workspace {
    pub fn new(path: PathBuf, fs: Arc<dyn FileSystem>) -> Result<Self> {
        let state_db_manager = StateDbManagerImpl::new(&path)
            .context("Failed to open the workspace state database")?;

        Ok(Self {
            path,
            fs,
            state_db_manager: Some(Arc::new(state_db_manager)),
            collections: OnceCell::new(),
            environments: OnceCell::new(),
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

                let mut scan_result = self
                    .state_db_manager
                    .as_ref()
                    .unwrap() // FIXME:
                    .environment_store()
                    .scan()?;
                for (name, env) in envs_from_fs {
                    let environment_entity = scan_result.remove(&name);

                    let environment_cache = if let Some(environment_entity) = environment_entity {
                        EnvironmentCache {
                            decoded_name: name, // TODO: decode name
                            order: environment_entity.order,
                            variables_cache: environment_entity
                                .local_values
                                .into_iter()
                                .map(|(name, state)| (name, VariableCache::from(state)))
                                .collect(),
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

    pub fn state_db_manager(&self) -> Result<Arc<dyn StateDbManager>> {
        self.state_db_manager
            .clone()
            .ok_or(anyhow!("The state_db_manager has been dropped"))
    }

    async fn collections(&self) -> Result<&RwLock<CollectionMap>> {
        let result = self
            .collections
            .get_or_try_init(|| async move {
                let mut collections = LeasedSlotMap::new();

                if !self.path.join(COLLECTIONS_DIR).exists() {
                    return Ok(RwLock::new(collections));
                }

                // TODO: Support external collections with absolute path
                for (relative_path, collection_data) in
                    self.state_db_manager()?.collection_store().scan()?
                {
                    let name = match relative_path.file_name() {
                        Some(name) => decode_directory_name(&name.to_string_lossy().to_string())?,
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
                    let collection = Collection::new(full_path, self.fs.clone())?;
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
}

// impl Workspace {
//     pub(crate) async fn reset(&mut self, new_path: PathBuf) -> Result<()> {
//         let _ = self.state_db_manager.take();

//         let old_path = std::mem::replace(&mut self.path, new_path.clone());
//         self.fs
//             .rename(&old_path, &new_path, RenameOptions::default())
//             .await?;

//         let state_db_manager_impl = StateDbManagerImpl::new(&new_path).context(format!(
//             "Failed to open the workspace {} state database",
//             self.path.display()
//         ))?;
//         self.state_db_manager = Some(Arc::new(state_db_manager_impl));

//         Ok(())
//     }
// }

impl Workspace {
    #[cfg(test)]
    pub fn truncate(&self) -> Result<()> {
        let collection_store = self.state_db_manager()?.collection_store();
        let (mut txn, table) = collection_store.begin_write()?;
        table.truncate(&mut txn)?;
        Ok(txn.commit()?)
    }
}
