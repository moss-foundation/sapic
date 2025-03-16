use anyhow::{anyhow, Context, Result};
use moss_collection::collection::{Collection, CollectionMetadata};
use moss_fs::ports::{FileSystem, RemoveOptions, RenameOptions};
use std::sync::Arc;
use std::{collections::HashMap, path::PathBuf};
use thiserror::Error;
use tokio::sync::{OnceCell, RwLock};

use crate::models::operations::{
    CreateCollectionInput, DeleteCollectionInput, ListCollectionRequestsInput,
    ListCollectionRequestsOutput, ListCollectionsOutput, RenameCollectionInput,
};
use crate::models::types::{CollectionInfo, RequestInfo};
use crate::storage::state_db_manager::StateDbManagerImpl;
use crate::storage::{CollectionEntity, StateDbManager};

enum CollectionValue {
    Available((Collection, CollectionMetadata)),
    Leased,
}

type CollectionMap = HashMap<PathBuf, (Collection, CollectionMetadata)>;

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
    collections: OnceCell<RwLock<CollectionMap>>,
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

    async fn collections(&self) -> Result<&RwLock<CollectionMap>> {
        self.collections
            .get_or_try_init(|| async move {
                let mut collections = HashMap::new();

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

                    let collection = Collection::new(collection_path.clone(), self.fs.clone())?;
                    let metadata = CollectionMetadata {
                        name,
                        order: collection_data.order,
                    };

                    collections.insert(collection_path, (collection, metadata));
                }

                Ok(RwLock::new(collections))
            })
            .await
    }
}

impl Workspace {
    pub async fn list_collections(&self) -> Result<ListCollectionsOutput> {
        let collections = self.collections().await?;
        let collections_lock = collections.read().await;

        Ok(ListCollectionsOutput(
            collections_lock
                .iter()
                .map(|(path, (_, metadata))| CollectionInfo {
                    path: path.clone(),
                    name: metadata.name.clone(),
                    order: metadata.order,
                })
                .collect(),
        ))
    }

    pub async fn list_collection_requests(
        &self,
        input: ListCollectionRequestsInput,
    ) -> Result<ListCollectionRequestsOutput> {
        let collections = self.collections().await?;
        let collections_lock = collections.read().await;

        let (collection, _) = collections_lock
            .get(&input.path)
            .ok_or(anyhow::anyhow!("Collection not found"))?;

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

    pub async fn create_collection(&self, input: CreateCollectionInput) -> Result<()> {
        if input.name.trim().is_empty() {
            return Err(CollectionOperationError::EmptyName.into());
        }

        let full_path = input.path.join(&input.name);
        let collections = self
            .collections()
            .await
            .context("Failed to get collections")?;

        {
            let read_lock = collections.read().await;
            if read_lock.contains_key(&full_path) {
                return Err(CollectionOperationError::DuplicateName {
                    name: input.name,
                    path: full_path,
                }
                .into());
            }
        }

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

        {
            let collection = Collection::new(full_path.clone(), self.fs.clone())?;
            let metadata = CollectionMetadata {
                name: input.name,
                order: None,
            };

            let mut write_lock = collections.write().await;
            write_lock.insert(full_path, (collection, metadata));
        }

        Ok(txn.commit()?)
    }

    pub async fn rename_collection(&self, input: RenameCollectionInput) -> Result<()> {
        if input.new_name.trim().is_empty() {
            return Err(CollectionOperationError::EmptyName.into());
        }

        dbg!(1);

        let new_path = input
            .old_path
            .parent()
            .context("Parent directory not found")?
            .join(&input.new_name);

        let collections = self
            .collections()
            .await
            .context("Failed to get collections")?;

        dbg!(2);

        let target_collection_metadata = {
            let collections_lock = collections.read().await;

            let (_, target_collection_metadata) = collections_lock.get(&input.old_path).ok_or({
                let old_name = input
                    .old_path
                    .file_name()
                    .ok_or(anyhow!("Failed to get the old collection name"))?;

                CollectionOperationError::NonexistentCollection {
                    name: old_name.to_string_lossy().to_string(),
                    path: input.old_path.clone(),
                }
            })?;

            target_collection_metadata.clone()
        };

        dbg!(3);

        let collection_store = self.state_db_manager.collection_store();
        let (mut txn, table) = collection_store.begin_write()?;
        let key = input.old_path.to_string_lossy().to_string();

        table.remove(&mut txn, key.clone())?;
        table.insert(
            &mut txn,
            key,
            &CollectionEntity {
                order: target_collection_metadata.order,
            },
        )?;

        self.fs
            .rename(&input.old_path, &new_path, RenameOptions::default())
            .await?;

        dbg!(4);

        let mut collections_lock = collections.write().await;
        let (collection, metadata) = collections_lock.remove(&input.old_path).unwrap(); // This is safe because we checked for the existence of the collection
        collection
            .reset(new_path.clone())
            .context("Failed to reset the collection")?;

        collections_lock.insert(new_path, (collection, metadata));

        dbg!(5);

        Ok(txn.commit()?)
    }

    pub async fn delete_collection(&self, input: DeleteCollectionInput) -> Result<()> {
        let collections = self.collections().await?;
        let collection_store = self.state_db_manager.collection_store();

        let (mut txn, table) = collection_store.begin_write()?;
        table.remove(&mut txn, input.path.to_string_lossy().to_string())?;

        self.fs
            .remove_dir(
                &input.path,
                RemoveOptions {
                    recursive: true,
                    ignore_if_not_exists: true,
                },
            )
            .await?;

        {
            let mut write_lock = collections.write().await;
            write_lock.remove(&input.path);
        }

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

        let result = workspace
            .create_collection(CreateCollectionInput {
                name: collection_name.clone(),
                path: workspace_path.clone(),
            })
            .await;

        assert!(result.is_ok());

        // Clean up
        {
            workspace.truncate().unwrap();
            std::fs::remove_dir_all(workspace_path.join(collection_name)).unwrap();
        }
    }

    #[tokio::test]
    async fn test_rename_collection() {
        let (workspace_path, workspace) = setup_test_workspace().await;
        let old_name = random_collection_name();
        let new_name = "New Test Collection".to_string(); // random_collection_name();

        // Create a test collection
        workspace
            .create_collection(CreateCollectionInput {
                name: old_name.clone(),
                path: workspace_path.clone(),
            })
            .await
            .unwrap();

        // Rename collection
        let result = workspace
            .rename_collection(RenameCollectionInput {
                old_path: workspace_path.join(&old_name),
                new_name: new_name.clone(),
            })
            .await
            .unwrap();

        // assert!(result.is_ok());

        // Verify rename
        let collections = workspace.list_collections().await.unwrap();
        assert_eq!(collections.0.len(), 1);
        assert_eq!(collections.0[0].name, new_name);

        // Clean up
        {
            workspace.truncate().unwrap();
            std::fs::remove_dir_all(workspace_path.join(new_name)).unwrap();
        }
    }

    #[tokio::test]
    async fn test_delete_collection() {
        let (workspace_path, workspace) = setup_test_workspace().await;
        let collection_name = random_collection_name();

        // Create a test collection
        workspace
            .create_collection(CreateCollectionInput {
                name: collection_name.clone(),
                path: workspace_path.clone(),
            })
            .await
            .unwrap();

        // Delete collection
        let result = workspace
            .delete_collection(DeleteCollectionInput {
                path: workspace_path.join(&collection_name),
            })
            .await;

        assert!(result.is_ok());

        // Verify deletion
        let collections = workspace.list_collections().await.unwrap();
        assert_eq!(collections.0.len(), 0);

        // Clean up
        {
            workspace.truncate().unwrap();
        }
    }

    #[tokio::test]
    async fn test_list_collection_requests() {
        let (workspace_path, workspace) = setup_test_workspace().await;
        let collection_name = random_collection_name();
        let collection_path = workspace_path.join(&collection_name);

        // Create a test collection
        workspace
            .create_collection(CreateCollectionInput {
                name: collection_name.clone(),
                path: workspace_path.clone(),
            })
            .await
            .unwrap();

        // List requests (should be empty initially)
        let result = workspace
            .list_collection_requests(ListCollectionRequestsInput {
                path: collection_path.clone(),
            })
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().0.len(), 0);

        // Clean up
        {
            workspace.truncate().unwrap();
            std::fs::remove_dir_all(collection_path).unwrap();
        }
    }
}
