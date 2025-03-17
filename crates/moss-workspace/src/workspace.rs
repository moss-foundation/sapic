use anyhow::{anyhow, Context, Result};
use moss_collection::collection::{Collection, CollectionMetadata};
use moss_fs::ports::{FileSystem, RemoveOptions, RenameOptions};
use scopeguard::defer;
use slotmap::{KeyData, SlotMap};
use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::{OnceCell, RwLock};

use crate::models::operations::{
    CreateCollectionInput, CreateCollectionOutput, DeleteCollectionInput,
    ListCollectionRequestsInput, ListCollectionRequestsOutput, ListCollectionsOutput,
    RenameCollectionInput,
};
use crate::models::types::{CollectionInfo, RequestInfo};
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

struct Slot {
    leased: AtomicBool,
    data: Option<(Collection, CollectionMetadata)>,
}

type CollectionSlot = (Collection, CollectionMetadata);

enum CollectionSlotState {
    Available(CollectionSlot),
    Leased,
}

impl CollectionSlotState {
    pub fn as_available(self) -> Option<CollectionSlot> {
        match self {
            CollectionSlotState::Available(slot) => Some(slot),
            CollectionSlotState::Leased => None,
        }
    }

    pub fn is_available(&self) -> bool {
        matches!(self, CollectionSlotState::Available(_))
    }
}

type CollectionMap = SlotMap<CollectionKey, CollectionSlotState>;

#[derive(Clone, Debug, Error)]
pub enum CollectionOperationError {
    #[error("The name of a collection cannot be empty.")]
    EmptyName,
    #[error("`{name}` is an invalid name for a collection.")]
    InvalidName { name: String }, // TODO: validate name
    #[error("A collection named {name} already exists in {path}.")]
    DuplicateName { name: String, path: PathBuf },
    #[error("The collection named `{name}` does not exist in {path}")]
    NonexistentCollection { name: String, path: PathBuf },
}

pub struct Workspace {
    fs: Arc<dyn FileSystem>,
    state_db_manager: Arc<dyn StateDbManager>,
    collections: OnceCell<Arc<RwLock<CollectionMap>>>,
}

pub struct LeaseGuard {
    key: CollectionKey,
    slot: Option<CollectionSlot>,
    collections: Arc<RwLock<CollectionMap>>,
}

impl LeaseGuard {
    pub fn collection(&self) -> &Collection {
        &self
            .slot
            .as_ref()
            .expect("LeaseGuard is not holding a collection")
            .0
    }

    pub fn collection_mut(&mut self) -> &mut Collection {
        &mut self
            .slot
            .as_mut()
            .expect("LeaseGuard is not holding a collection")
            .0
    }

    pub fn collection_metadata(&self) -> &CollectionMetadata {
        &self
            .slot
            .as_ref()
            .expect("LeaseGuard is not holding a collection")
            .1
    }
}

impl Drop for LeaseGuard {
    fn drop(&mut self) {
        let collections = self.collections.clone();
        let key = self.key;
        let slot = self.slot.take().unwrap();

        tokio::spawn(async move {
            let mut collections_lock = collections.write().await;

            if let Some(state) = collections_lock.get_mut(key) {
                match state {
                    CollectionSlotState::Leased => {
                        *state = CollectionSlotState::Available(slot);
                    }
                    CollectionSlotState::Available(_) => {
                        // TODO: log here
                        println!("Collection was not leased");
                    }
                }
            } else {
                // TODO: log here
                println!("Collection not found");
            }
        });
    }
}

impl Workspace {
    pub fn new(path: PathBuf, fs: Arc<dyn FileSystem>) -> Result<Self> {
        let state_db_manager = StateDbManagerImpl::new(&path)
            .context("Failed to open the workspace state database")?;

        Ok(Self {
            fs,
            state_db_manager: Arc::new(state_db_manager),
            collections: OnceCell::new(),
        })
    }

    async fn lease_collection(&self, key: CollectionKey) -> Result<LeaseGuard> {
        let collections = self.collections().await?;

        let mut collections_lock = collections.write().await;
        match collections_lock.get_mut(key) {
            Some(slot_state) => {
                if let CollectionSlotState::Available(slot) =
                    std::mem::replace(slot_state, CollectionSlotState::Leased)
                {
                    Ok(LeaseGuard {
                        key,
                        slot: Some(slot),
                        collections: collections.clone(),
                    })
                } else {
                    Err(anyhow!("Collection is leased").into())
                }
            }
            None => Err(anyhow!("Collection not found").into()),
        }
    }

    async fn release_collection(&self, key: CollectionKey, slot: CollectionSlot) -> Result<()> {
        let collections = self.collections().await?;
        let mut collections_lock = collections.write().await;

        if let Some(state) = collections_lock.get_mut(key) {
            match state {
                CollectionSlotState::Leased => {
                    *state = CollectionSlotState::Available(slot);
                    Ok(())
                }
                CollectionSlotState::Available(_) => {
                    Err(anyhow!("Collection was not leased").into())
                }
            }
        } else {
            Err(anyhow!("Collection not found").into())
        }
    }

    async fn collections(&self) -> Result<Arc<RwLock<CollectionMap>>> {
        let result = self
            .collections
            .get_or_try_init(|| async move {
                let mut collections = SlotMap::with_key();

                for (collection_path, collection_data) in
                    self.state_db_manager.collection_store().scan()?
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

                    collections.insert(CollectionSlotState::Available((collection, metadata)));
                }

                Ok::<_, anyhow::Error>(Arc::new(RwLock::new(collections)))
            })
            .await?;

        Ok(Arc::clone(result))
    }
}

impl Workspace {
    pub async fn list_collections(&self) -> Result<ListCollectionsOutput> {
        let collections = self.collections().await?;
        let collections_lock = collections.read().await;

        Ok(ListCollectionsOutput(
            collections_lock
                .iter()
                .filter_map(|(_, state)| match state {
                    CollectionSlotState::Available((_, metadata)) => Some(CollectionInfo {
                        name: metadata.name.clone(),
                        order: metadata.order,
                    }),
                    CollectionSlotState::Leased => {
                        // TODO: logging
                        println!("collection is leased");
                        None
                    }
                })
                .collect(),
        ))
    }

    pub async fn list_collection_requests(
        &self,
        input: ListCollectionRequestsInput,
    ) -> Result<ListCollectionRequestsOutput> {
        let collections = self.collections().await?;
        let collection_key = CollectionKey::from(input.key);

        let collections_lock = collections.read().await;
        let collection_value = collections_lock
            .get(collection_key)
            .ok_or(anyhow::anyhow!("Collection not found"))?;

        let collection = match collection_value {
            CollectionSlotState::Available((collection, _)) => collection,
            CollectionSlotState::Leased => {
                return Err(anyhow::anyhow!("Collection is leased").into());
            }
        };

        let requests = collection.list_requests().await?;
        let requests_lock = requests.read().await;

        let requests_info: Vec<RequestInfo> = requests_lock
            .iter()
            .map(|(_, request)| RequestInfo {
                name: request.name.clone(),
                order: request.order,
                typ: request.typ.clone(),
            })
            .collect();

        Ok(ListCollectionRequestsOutput(requests_info))
    }

    pub async fn create_collection(
        &self,
        input: CreateCollectionInput,
    ) -> Result<CreateCollectionOutput> {
        if input.name.trim().is_empty() {
            return Err(CollectionOperationError::EmptyName.into());
        }

        let full_path = input.path.join(&input.name);
        let collections = self
            .collections()
            .await
            .context("Failed to get collections")?;

        // TODO: is dir with the same name already exists

        let collection_store = self.state_db_manager.collection_store();
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
            collections_lock.insert(CollectionSlotState::Available((collection, metadata)))
        };

        txn.commit()?;

        Ok(CreateCollectionOutput {
            key: collection_key.as_u64(),
            name: input.name,
            path: full_path,
        })
    }

    pub async fn rename_collection(&self, input: RenameCollectionInput) -> Result<()> {
        if input.new_name.trim().is_empty() {
            return Err(CollectionOperationError::EmptyName.into());
        }

        dbg!(1);

        // let collections = self
        //     .collections()
        //     .await
        //     .context("Failed to get collections")?;

        dbg!(2);

        let collection_key = CollectionKey::from(input.key);
        let mut lease_guard = self.lease_collection(collection_key).await?;
        // let lease_guard = scopeguard::guard(self.lease_collection(collection_key).await?, |slot| {
        //     let collections_clone = collections.clone();

        //     tokio::spawn(async move {
        //         if let Err(e) = collections_clone.release(collection_key, slot).await {
        //             eprintln!("Failed to release collection: {:?}", e);
        //         }
        //     });
        // });

        // let collection = &mut lease_guard.0;
        // let metadata = lease_guard.1;

        // let target_collection_metadata = {
        //     let collections_lock = collections.read().await;

        //     let value = collections_lock.get(collection_key).ok_or({
        //         // let old_name = input
        //         //     .old_path
        //         //     .file_name()
        //         //     .ok_or(anyhow!("Failed to get the old collection name"))?;

        //         // CollectionOperationError::NonexistentCollection {
        //         //     name: old_name.to_string_lossy().to_string(),
        //         //     path: input.old_path.clone(),
        //         // }

        //         anyhow::anyhow!("Collection not found")
        //     })?;

        //     match value {
        //         CollectionValue::Available((_, metadata)) => metadata,
        //         CollectionValue::Leased => {
        //             return Err(anyhow::anyhow!("Collection is leased").into());
        //         }
        //     }
        // };

        let old_path = lease_guard.collection().path();
        let new_path = old_path
            .parent()
            .context("Parent directory not found")?
            .join(&input.new_name);

        let collection_store = self.state_db_manager.collection_store();
        let (mut txn, table) = collection_store.begin_write()?;

        let entity_key = old_path.to_string_lossy().to_string();

        table.remove(&mut txn, entity_key.clone())?;
        table.insert(
            &mut txn,
            entity_key,
            &CollectionEntity {
                order: lease_guard.collection_metadata().order,
            },
        )?;

        self.fs
            .rename(&old_path, &new_path, RenameOptions::default())
            .await?;

        dbg!(4);

        lease_guard
            .collection_mut()
            .reset(new_path)
            .context("Failed to reset the collection")?;

        // collections
        //     .release(collection_key, (collection, metadata))
        //     .await
        //     .context("Failed to release the collection")?;

        // let mut collections_lock = collections.state.write().await;
        // let (collection, metadata) = collections_lock.remove(collection_key).unwrap(); // This is safe because we checked for the existence of the collection
        // collection
        //     .reset(new_path.clone())
        //     .context("Failed to reset the collection")?;

        // collections_lock.insert(
        //     collection_key,
        //     CollectionSlotState::Available((collection, metadata)),
        // );

        dbg!(5);

        Ok(txn.commit()?)
    }

    pub async fn delete_collection(&self, input: DeleteCollectionInput) -> Result<()> {
        let collections = self.collections().await?;
        let collection_key = CollectionKey::from(input.key);

        // Intentionally acquire the write mutex to ensure that no one else can access the collection being deleted.
        let mut collections_lock = collections.write().await;
        let slot_state = collections_lock
            .get(collection_key)
            .context("Collection not found")?;
        if !slot_state.is_available() {
            return Err(anyhow!("Collection is leased").into());
        }

        let (collection, _) = collections_lock
            .remove(collection_key)
            .unwrap()
            .as_available()
            .unwrap(); // This is safe because we checked for the existence of the collection

        let collection_path = collection.path().clone();
        let collection_store = self.state_db_manager.collection_store();
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

        collections_lock.remove(collection_key);
        Ok(txn.commit()?)
    }
}

impl Workspace {
    #[cfg(test)]
    pub fn truncate(&self) -> Result<()> {
        let collection_store = self.state_db_manager.collection_store();
        let (mut txn, table) = collection_store.begin_write()?;
        table.truncate(&mut txn)?;
        Ok(txn.commit()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::random_collection_name;
    use moss_fs::adapters::disk::DiskFileSystem;

    async fn setup_test_workspace() -> (PathBuf, Workspace) {
        let fs = Arc::new(DiskFileSystem::new());
        let workspace_path: PathBuf =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../samples/workspaces/My Workspace");
        let workspace = Workspace::new(workspace_path.clone(), fs).unwrap();
        (workspace_path, workspace)
    }

    #[tokio::test]
    async fn create_collection() {
        let (workspace_path, workspace) = setup_test_workspace().await;
        let collection_name = random_collection_name();
        let expected_path = workspace_path.join(&collection_name);

        let result = workspace
            .create_collection(CreateCollectionInput {
                name: collection_name.clone(),
                path: workspace_path.clone(),
            })
            .await;

        assert!(result.is_ok());

        let output = result.unwrap();

        assert_eq!(output.name, collection_name);
        assert_eq!(output.path, expected_path.clone());

        // Clean up
        {
            workspace.truncate().unwrap();
            std::fs::remove_dir_all(expected_path).unwrap();
        }
    }

    // #[tokio::test]
    // async fn test_rename_collection() {
    //     let (workspace_path, workspace) = setup_test_workspace().await;
    //     let old_name = random_collection_name();
    //     let new_name = "New Test Collection".to_string(); // random_collection_name();

    //     // Create a test collection
    //     workspace
    //         .create_collection(CreateCollectionInput {
    //             name: old_name.clone(),
    //             path: workspace_path.clone(),
    //         })
    //         .await
    //         .unwrap();

    //     // Rename collection
    //     let result = workspace
    //         .rename_collection(RenameCollectionInput {
    //             old_path: workspace_path.join(&old_name),
    //             new_name: new_name.clone(),
    //         })
    //         .await
    //         .unwrap();

    //     // assert!(result.is_ok());

    //     // Verify rename
    //     let collections = workspace.list_collections().await.unwrap();
    //     assert_eq!(collections.0.len(), 1);
    //     assert_eq!(collections.0[0].name, new_name);

    //     // Clean up
    //     {
    //         workspace.truncate().unwrap();
    //         std::fs::remove_dir_all(workspace_path.join(new_name)).unwrap();
    //     }
    // }

    // #[tokio::test]
    // async fn test_delete_collection() {
    //     let (workspace_path, workspace) = setup_test_workspace().await;
    //     let collection_name = random_collection_name();

    //     // Create a test collection
    //     workspace
    //         .create_collection(CreateCollectionInput {
    //             name: collection_name.clone(),
    //             path: workspace_path.clone(),
    //         })
    //         .await
    //         .unwrap();

    //     // Delete collection
    //     let result = workspace
    //         .delete_collection(DeleteCollectionInput {
    //             path: workspace_path.join(&collection_name),
    //         })
    //         .await;

    //     assert!(result.is_ok());

    //     // Verify deletion
    //     let collections = workspace.list_collections().await.unwrap();
    //     assert_eq!(collections.0.len(), 0);

    //     // Clean up
    //     {
    //         workspace.truncate().unwrap();
    //     }
    // }

    // #[tokio::test]
    // async fn test_list_collection_requests() {
    //     let (workspace_path, workspace) = setup_test_workspace().await;
    //     let collection_name = random_collection_name();
    //     let collection_path = workspace_path.join(&collection_name);

    //     // Create a test collection
    //     workspace
    //         .create_collection(CreateCollectionInput {
    //             name: collection_name.clone(),
    //             path: workspace_path.clone(),
    //         })
    //         .await
    //         .unwrap();

    //     // List requests (should be empty initially)
    //     let result = workspace
    //         .list_collection_requests(ListCollectionRequestsInput {
    //             path: collection_path.clone(),
    //         })
    //         .await;

    //     assert!(result.is_ok());
    //     assert_eq!(result.unwrap().0.len(), 0);

    //     // Clean up
    //     {
    //         workspace.truncate().unwrap();
    //         std::fs::remove_dir_all(collection_path).unwrap();
    //     }
    // }
}
