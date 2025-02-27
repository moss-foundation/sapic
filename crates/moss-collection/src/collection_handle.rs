use anyhow::Result;
use moss_fs::ports::FileSystem;
use parking_lot::RwLock;
use patricia_tree::PatriciaMap;
use std::{path::PathBuf, sync::Arc};

use crate::{request_handle::RequestHandle, storage::CollectionRequestSubstore};

pub(crate) struct CollectionState {
    pub name: String,
    pub order: Option<usize>,
    pub requests: RwLock<PatriciaMap<Arc<RequestHandle>>>,
}

impl CollectionState {
    pub fn new(name: String, order: Option<usize>) -> Self {
        Self {
            name,
            order,
            requests: RwLock::new(PatriciaMap::new()),
        }
    }

    pub fn get_request_handle_or_init(
        &self,
        key: &[u8],
        f: impl FnOnce() -> RequestHandle,
    ) -> Arc<RequestHandle> {
        {
            let read_guard = self.requests.read();
            if let Some(entry) = read_guard.get(key) {
                return Arc::clone(&entry);
            }
        }

        let mut write_guard = self.requests.write();
        if let Some(entry) = write_guard.get(key) {
            return Arc::clone(&entry);
        }

        let entry = Arc::new(f());
        write_guard.insert(key, Arc::clone(&entry));
        entry
    }
}

pub struct CollectionHandle {
    fs: Arc<dyn FileSystem>,
    store: Arc<dyn CollectionRequestSubstore>,
    state: Arc<CollectionState>,
}

pub struct CreateRequestInput {
    name: String,
}

impl CollectionHandle {
    pub fn new(
        fs: Arc<dyn FileSystem>,
        store: Arc<dyn CollectionRequestSubstore>,
        state: Arc<CollectionState>,
    ) -> Self {
        Self { fs, store, state }
    }

    pub fn state(&self) -> Arc<CollectionState> {
        Arc::clone(&self.state)
    }

    pub fn create_request(
        &self,
        collection_path: &PathBuf,
        input: CreateRequestInput,
    ) -> Result<()> {
        unimplemented!()
    }
}
