use anyhow::{Context, Result};
use moss_fs::ports::FileSystem;
use patricia_tree::PatriciaMap;
use std::{path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

use crate::{
    indexing::{indexer::IndexerImpl, Indexer},
    models::collection::RequestType,
    storage::{state_db_manager::StateDbManagerImpl, StateDbManager},
};

#[derive(Clone, Debug)]
pub struct CollectionMetadata {
    pub name: String,
    pub order: Option<usize>,
}

pub struct CollectionRequestData {
    pub name: String,
    pub request_path_str: String,
    pub order: Option<usize>,
    pub typ: RequestType,
}

type RequestMap = PatriciaMap<Arc<CollectionRequestData>>;

pub struct Collection {
    path: PathBuf,
    indexer: Arc<dyn Indexer>,
    requests: RwLock<RequestMap>,
    state_db_manager: Arc<dyn StateDbManager>,
}

impl Collection {
    pub fn new(path: PathBuf, fs: Arc<dyn FileSystem>) -> Result<Self> {
        let state_db_manager_impl = StateDbManagerImpl::new(&path).context(format!(
            "Failed to open the collection {} state database",
            path.display()
        ))?;

        Ok(Self {
            path,
            indexer: Arc::new(IndexerImpl::new(fs)),
            requests: RwLock::new(PatriciaMap::new()),
            state_db_manager: Arc::new(state_db_manager_impl),
        })
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn reset(&mut self, new_path: PathBuf) -> Result<()> {
        self.path = new_path;

        Ok(())
    }

    pub async fn list_requests(&self) -> Result<&RwLock<RequestMap>> {
        let indexed_collection = self.indexer.index(&self.path).await?;
        let requests = self.state_db_manager.request_store().scan()?;

        let mut request_map = PatriciaMap::new();
        for (raw_request_path, indexed_request_entry) in indexed_collection.requests {
            let request_path_str = match String::from_utf8(raw_request_path.clone()) {
                Ok(value) => value,
                Err(_err) => {
                    // TODO: log error

                    continue;
                }
            };

            let entity = requests.get(&request_path_str);
            let data = CollectionRequestData {
                name: indexed_request_entry.name,
                request_path_str,
                order: entity.and_then(|e| e.order),
                typ: indexed_request_entry.typ.unwrap(), // FIXME: get rid of Option type for typ
            };

            request_map.insert(raw_request_path, Arc::new(data));
        }

        self.requests.write().await.extend(request_map);
        Ok(&self.requests)
    }
}
