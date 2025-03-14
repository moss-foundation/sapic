use anyhow::Result;
use moss_collection::collection::{Collection, CollectionMetadata};
use moss_collection::indexing::indexer::IndexingService;
use moss_collection::indexing::CollectionIndexer;
use moss_fs::ports::FileSystem;
use std::sync::Arc;
use std::{collections::HashMap, path::PathBuf};
use tokio::sync::{OnceCell, RwLock};

use crate::models::operations::{DescribeCollectionOutput, ListCollectionsOutput};
use crate::models::types::CollectionInfo;
use crate::storage::state_db_manager::StateDbManagerImpl;
use crate::storage::StateDbManager;

type CollectionMap = HashMap<PathBuf, (Collection, CollectionMetadata)>;

pub struct Workspace {
    fs: Arc<dyn FileSystem>,
    state_db_manager: Arc<dyn StateDbManager>,
    collections: OnceCell<RwLock<CollectionMap>>,
}

impl Workspace {
    pub fn new(path: PathBuf, fs: Arc<dyn FileSystem>) -> Result<Self> {
        let state_db_manager = StateDbManagerImpl::new(&path)?;

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

    pub async fn describe_collection(&self, path: PathBuf) -> Result<DescribeCollectionOutput> {
        let collections = self.collections().await?;
        let collections_lock = collections.read().await;

        let (collection, _) = collections_lock
            .get(&path)
            .ok_or(anyhow::anyhow!("Collection not found"))?;

        todo!()
    }
}
