use anyhow::{anyhow, Context, Result};
use moss_collection::collection::{Collection, CollectionMetadata};
use moss_fs::{
    FileSystem, RemoveOptions, RenameOptions,
    utils::{decode_directory_name, encode_directory_name},
};
use slotmap::KeyData;
use std::path::PathBuf;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::{OnceCell, RwLock};
use validator::{Validate, ValidationErrors};
use crate::leased_slotmap::LeasedSlotMap;
use crate::models::operations::{
    CreateCollectionInput, CreateCollectionOutput, DeleteCollectionInput, ListCollectionsOutput,
    RenameCollectionInput,
};
use crate::models::types::CollectionInfo;
use crate::storage::state_db_manager::StateDbManagerImpl;
use crate::storage::{CollectionEntity, StateDbManager};

slotmap::new_key_type! {
    pub struct CollectionKey;
}

impl From<u64> for CollectionKey {
    fn from(value: u64) -> Self {
        Self(KeyData::from_ffi(value))
    }
}

impl CollectionKey {
    pub fn as_u64(self) -> u64 {
        self.0.as_ffi()
    }
}

impl std::fmt::Display for CollectionKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_u64())
    }
}

#[derive(Error, Debug)]
pub enum OperationError {
    #[error("validation error: {0}")]
    Validation(#[from] ValidationErrors),

    #[error("collection {name} not found at {path}")]
    NotFound { name: String, path: PathBuf },

    #[error("collection {name} already exists at {path}")]
    AlreadyExists { name: String, path: PathBuf },

    #[error("unknown error: {0}")]
    Unknown(#[from] anyhow::Error),
}

type CollectionSlot = (Collection, CollectionMetadata);
type CollectionMap = LeasedSlotMap<CollectionKey, CollectionSlot>;

// TODO: create collections at workspace/collections

// workspace1/collections/collection1/requests
pub struct Workspace {
    fs: Arc<dyn FileSystem>,
    path: PathBuf,
    // We have to use Option so that we can temporarily drop it
    // TODO: implement is_external flag for absolute/relative path
    // Right now, we are storing relative paths in the db_manager
    state_db_manager: Option<Arc<dyn StateDbManager>>,
    collections: OnceCell<RwLock<CollectionMap>>,
}

impl Workspace {
    pub fn new(path: PathBuf, fs: Arc<dyn FileSystem>) -> Result<Self> {
        let state_db_manager = StateDbManagerImpl::new(&path)
            .context("Failed to open the workspace state database")?;

        Ok(Self {
            fs,
            path,
            state_db_manager: Some(Arc::new(state_db_manager)),
            collections: OnceCell::new(),
        })
    }
    pub fn state_db_manager(&self) -> Result<Arc<dyn StateDbManager>> {
        self.state_db_manager.clone().ok_or(anyhow!("The state_db_manager has been dropped"))
    }
    async fn collections(&self) -> Result<&RwLock<CollectionMap>> {
        let result = self
            .collections
            .get_or_try_init(|| async move {
                let mut collections = LeasedSlotMap::new();


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
                    let metadata = CollectionMetadata {
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

impl Workspace {
    pub async fn list_collections(&self) -> Result<ListCollectionsOutput, OperationError> {
        let collections = self.collections().await?;
        let collections_lock = collections.read().await;

        Ok(ListCollectionsOutput(
            collections_lock
                .iter()
                .filter(|(_, iter_slot)| !iter_slot.is_leased())
                .map(|(key, iter_slot)| {
                    let (_, metadata) = iter_slot.value();
                    CollectionInfo {
                        key: key.as_u64(),
                        name: metadata.name.clone(),
                        order: metadata.order,
                    }
                })
                .collect(),
        ))
    }

    pub async fn create_collection(
        &self,
        input: CreateCollectionInput,
    ) -> Result<CreateCollectionOutput, OperationError> {
        input.validate()?;

        let relative_path = encode_directory_name(&input.name);
        // workspace_path/encoded_collection_folder
        let full_path = self.path().join(&relative_path);

        if full_path.exists() {
            return Err(OperationError::AlreadyExists {
                name: input.name,
                path: full_path.clone()
            });
        }

        let collections = self
            .collections()
            .await
            .context("Failed to get collections")?;

        let collection_store = self.state_db_manager()?.collection_store();
        let (mut txn, table) = collection_store.begin_write()?;

        table.insert(
            &mut txn,
            relative_path,
            &CollectionEntity { order: None },
        )?;

        self.fs
            .create_dir(&full_path)
            .await
            .context("Failed to create the collection directory")?;

        let collection = Collection::new(full_path.clone(), self.fs.clone())?;
        let metadata = CollectionMetadata {
            name: input.name.clone(),
            order: None,
        };

        let collection_key = {
            let mut collections_lock = collections.write().await;
            collections_lock.insert((collection, metadata))
        };

        txn.commit()?;

        Ok(CreateCollectionOutput {
            key: collection_key.as_u64(),
        })
    }

    pub async fn rename_collection(&self, input: RenameCollectionInput) -> Result<(), OperationError> {
        input.validate()?;

        let collections = self
            .collections()
            .await
            .context("Failed to get collections")?;

        let collection_key = CollectionKey::from(input.key);
        let mut collections_lock = collections.write().await;
        let mut lease_guard = collections_lock
            .lease(collection_key)
            .context("Failed to lease the collection")?;

        let (collection, metadata) = &mut *lease_guard;

        if metadata.name == input.new_name {
            return Ok(())
        }

        let old_full_path = collection.path().to_owned();
        if !old_full_path.exists() {
            return Err(OperationError::NotFound {
                name: metadata.name.clone(),
                path: old_full_path,
            })
        }
        let old_relative_path = old_full_path.strip_prefix(&self.path).unwrap();
        let new_relative_path = old_relative_path.parent().context("Parent directory not found")?
            .join(encode_directory_name(&input.new_name));
        let new_full_path = self.path.join(&new_relative_path);

        if new_full_path.exists() {
            return Err(OperationError::AlreadyExists {
                name: input.new_name,
                path: new_full_path,
            })
        }

        let collection_store = self.state_db_manager()?.collection_store();
        let (mut txn, table) = collection_store.begin_write()?;

        let old_table_key = old_relative_path.to_string_lossy().to_string();
        let new_table_key = new_relative_path.to_string_lossy().to_string();

        table.remove(&mut txn, old_table_key)?;
        table.insert(
            &mut txn,
            new_table_key,
            &CollectionEntity {
                order: metadata.order,
            },
        )?;

        // The state_db_manager will hold the `state.db` file open, preventing renaming on Windows
        // We need to temporarily drop it, and reload the database after that
        collection
            .reset(new_full_path)
            .await
            .context("Failed to reset the collection")?;

        metadata.name = input.new_name.clone();

        Ok(txn.commit()?)
    }

    pub async fn delete_collection(&self, input: DeleteCollectionInput) -> Result<(), OperationError> {
        let collections = self.collections().await?;
        let collection_key = CollectionKey::from(input.key);

        let mut collections_lock = collections.write().await;
        let (collection, _) = collections_lock
            .remove(collection_key)
            .context("Failed to remove the collection")?;

        let collection_path = collection.path().clone();
        let collection_relative_path = collection_path.strip_prefix(&self.path).unwrap();
        let collection_store = self.state_db_manager()?.collection_store();

        // TODO: If any of the following operations fail, we should place the task
        // in the dead queue and attempt the deletion later.

        let (mut txn, table) = collection_store.begin_write()?;
        let table_key = collection_relative_path.to_string_lossy().to_string();
        table.remove(&mut txn, table_key)?;

        // TODO: logging if the folder has already been removed from the filesystem
        self.fs
            .remove_dir(
                &collection_path,
                RemoveOptions {
                    recursive: true,
                    ignore_if_not_exists: true,
                },
            )
            .await?;

        Ok(txn.commit()?)
    }
    pub(crate) async fn reset(&mut self, new_path: PathBuf) -> Result<()> {
        let _ = self.state_db_manager.take();

        let old_path = std::mem::replace(&mut self.path, new_path.clone());
        self.fs.rename(&old_path, &new_path, RenameOptions::default()).await?;

        let state_db_manager_impl = StateDbManagerImpl::new(new_path).context(format!(
            "Failed to open the workspace {} state database",
            self.path.display()
        ))?;
        self.state_db_manager = Some(Arc::new(state_db_manager_impl));

        Ok(())
    }

}

impl Workspace {
    #[cfg(test)]
    pub fn truncate(&self) -> Result<()> {
        let collection_store = self.state_db_manager()?.collection_store();
        let (mut txn, table) = collection_store.begin_write()?;
        table.truncate(&mut txn)?;
        Ok(txn.commit()?)
    }
}

