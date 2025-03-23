use anyhow::{anyhow, Context, Result};
use moss_collection::collection::{Collection, CollectionMetadata};
use moss_fs::ports::{FileSystem, RemoveOptions, RenameOptions};
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

    #[error("collection {key} not found at {path}")]
    NotFound { key: PathBuf, path: PathBuf },

    #[error("collection {key} already exists at {path}")]
    AlreadyExists { key: PathBuf, path: PathBuf },

    #[error("unknown error: {0}")]
    Unknown(#[from] anyhow::Error),
}

type CollectionSlot = (Collection, CollectionMetadata);
type CollectionMap = LeasedSlotMap<CollectionKey, CollectionSlot>;

pub struct Workspace {
    fs: Arc<dyn FileSystem>,
    path: PathBuf,
    // We have to use Option so that we can temporarily drop it
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

                for (collection_path, collection_data) in
                    self.state_db_manager()?.collection_store().scan()?
                {
                    let name = match collection_path.file_name() {
                        Some(name) => name.to_string_lossy().to_string(),
                        None => {
                            // TODO: logging
                            println!("failed to get the collection {:?} name", collection_path);
                            continue;
                        }
                    };

                    // TODO:A self-healing mechanism needs to be implemented here.
                    // Collections that are found in the database but do not actually exist
                    // in the file system should be collected and deleted from the database in
                    // a parallel thread.

                    let collection = Collection::new(collection_path.clone(), self.fs.clone())?;
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

        let full_path = input.path.join(&input.name);

        if full_path.exists() {
            return Err(OperationError::AlreadyExists {
                key: PathBuf::from(&input.name),
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
            full_path.to_string_lossy().to_string(),
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
            name: input.name,
            path: full_path,
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
        metadata.name = input.new_name.clone();

        let old_path = collection.path().to_owned();
        let new_path = old_path
            .parent()
            .context("Parent directory not found")?
            .join(&input.new_name);

        let collection_store = self.state_db_manager()?.collection_store();
        let (mut txn, table) = collection_store.begin_write()?;

        let entity_key = old_path.to_string_lossy().to_string();

        table.remove(&mut txn, entity_key.clone())?;
        table.insert(
            &mut txn,
            entity_key,
            &CollectionEntity {
                order: metadata.order,
            },
        )?;

        // The state_db_manager will hold the `state.db` file open, preventing renaming on Windows
        // We need to temporarily drop it, and reload the database after that
        collection
            .reset(new_path)
            .await
            .context("Failed to reset the collection")?;

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
        let collection_store = self.state_db_manager()?.collection_store();

        // TODO: If any of the following operations fail, we should place the task
        // in the dead queue and attempt the deletion later.

        let (mut txn, table) = collection_store.begin_write()?;
        table.remove(&mut txn, collection_path.to_string_lossy().to_string())?;

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

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;
    use crate::utils::{random_collection_name, random_string};
    use moss_fs::adapters::disk::DiskFileSystem;


    async fn setup_test_workspace() -> (PathBuf, Workspace) {
        let fs = Arc::new(DiskFileSystem::new());
        let workspace_path: PathBuf =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("../../samples/workspaces/{}", random_string(10)));
        fs::create_dir_all(&workspace_path).unwrap();
        let workspace = Workspace::new(workspace_path.clone(), fs).unwrap();
        (workspace_path, workspace)
    }

    #[tokio::test]
    async fn create_collection() {
        let (workspace_path, workspace) = setup_test_workspace().await;
        let collection_name = random_collection_name();
        let expected_path = workspace_path.join(&collection_name);

        let create_collection_result = workspace
            .create_collection(CreateCollectionInput {
                name: collection_name.clone(),
                path: workspace_path.clone(),
            })
            .await;


        let create_collection_output = create_collection_result.unwrap();

        assert_eq!(create_collection_output.name, collection_name);
        assert_eq!(create_collection_output.path, expected_path.clone());

        // Clean up
        {
            workspace.truncate().unwrap();
            std::fs::remove_dir_all(workspace_path).unwrap();
        }
    }

    #[tokio::test]
    async fn rename_collection() {
        let (workspace_path, workspace) = setup_test_workspace().await;
        let old_name = random_collection_name();
        let new_name = "New Test Collection".to_string(); // random_collection_name();

        // Create a test collection
        let create_collection_output = workspace
            .create_collection(CreateCollectionInput {
                name: old_name.clone(),
                path: workspace_path.clone(),
            })
            .await
            .unwrap();

        // Rename collection
        let rename_collection_result = workspace
            .rename_collection(RenameCollectionInput {
                key: create_collection_output.key,
                new_name: new_name.clone(),
            })
            .await.unwrap();


        // Verify rename
        let collections = workspace.list_collections().await.unwrap();

        assert_eq!(collections.0.len(), 1);
        assert_eq!(collections.0[0].name, new_name);

        // Clean up
        {
            workspace.truncate().unwrap();
            std::fs::remove_dir_all(workspace_path).unwrap();
        }
    }

    #[tokio::test]
    async fn delete_collection() {
        let (workspace_path, workspace) = setup_test_workspace().await;
        let collection_name = random_collection_name();

        // Create a test collection
        let create_collection_output = workspace
            .create_collection(CreateCollectionInput {
                name: collection_name.clone(),
                path: workspace_path.clone(),
            })
            .await
            .unwrap();

        // Delete collection
        let delete_collection_result = workspace
            .delete_collection(DeleteCollectionInput {
                key: create_collection_output.key,
            })
            .await;

        assert!(delete_collection_result.is_ok());

        // Verify deletion
        let collections = workspace.list_collections().await.unwrap();
        assert_eq!(collections.0.len(), 0);

        // Clean up
        {
            workspace.truncate().unwrap();
            std::fs::remove_dir_all(workspace_path).unwrap();
        }
    }
}
