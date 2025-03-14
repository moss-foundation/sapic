use anyhow::Result;
use moss_fs::ports::FileSystem;
use patricia_tree::PatriciaMap;
use std::{path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

use crate::{
    indexing::{indexer::IndexingService, CollectionIndexer},
    models::collection::RequestType,
    storage::{state_db_manager::StateDbManagerImpl, StateDbManager},
};

pub struct CollectionMetadata {
    pub name: String,
    pub order: Option<usize>,
}

pub struct CollectionRequestData {
    pub name: String,
    pub order: Option<usize>,
    pub typ: RequestType,
}

pub struct Collection {
    path: PathBuf,
    state_db_manager: Arc<dyn StateDbManager>,
    indexer: Arc<dyn CollectionIndexer>,
    requests: RwLock<PatriciaMap<Arc<CollectionRequestData>>>,
}

impl Collection {
    pub fn new(path: PathBuf, fs: Arc<dyn FileSystem>) -> Result<Self> {
        let state_db_manager = StateDbManagerImpl::new(&path)?;

        Ok(Self {
            path,
            state_db_manager: Arc::new(state_db_manager),
            indexer: Arc::new(IndexingService::new(fs)),
            requests: RwLock::new(PatriciaMap::new()),
        })
    }

    pub async fn describe(&self) -> Result<()> {
        let indexed_collection = self.indexer.index(&self.path).await?;
        let requests = self.state_db_manager.request_store().scan()?;

        for (raw_request_path, indexed_request_entry) in indexed_collection.requests {
            let request_path_str = match String::from_utf8(raw_request_path.clone()) {
                Ok(value) => value,
                Err(err) => {
                    // TODO: log error

                    continue;
                }
            };

            let entity = requests.get(&request_path_str);
            let data = CollectionRequestData {
                name: indexed_request_entry.name,
                order: entity.map(|e| e.order),
                typ: indexed_request_entry.typ.unwrap(), // FIXME: get rid of Option type for typ
            };

            self.requests
                .write()
                .await
                .insert(raw_request_path, Arc::new(data));

            // self.requests.write().insert(raw_request_path, Arc::new(data));
        }

        Ok(())
    }
}
