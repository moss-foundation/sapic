use std::collections::HashMap;
use super::request_store::TABLE_REQUESTS;
use super::{request_store::RequestStoreImpl, RequestStore, RequestStoreTable, StateDbManager};
use anyhow::{anyhow, Result};
use moss_db::{ReDbClient, Transaction};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use async_trait::async_trait;
use moss_fs::{FileSystem, RenameOptions};
use arc_swap::ArcSwap;
use crate::models::storage::RequestEntity;

const COLLECTION_STATE_DB_NAME: &str = "state.db";

// ArcSwap only supports sized types
pub struct RequestStoreWrapper(Arc<dyn RequestStore>);

pub struct StateDbManagerImpl {
    path: ArcSwap<PathBuf>,
    request_store: ArcSwap<RequestStoreWrapper>,
}

// This is used to make temporary swap work
pub struct DummyRequestStore {}

impl DummyRequestStore {
    pub fn new() -> Self {
        Self {}
    }
}

impl RequestStore for DummyRequestStore {
    fn begin_write(&self) -> Result<(Transaction, &RequestStoreTable)> {
        Err(anyhow!("Cannot write on a dummy request store"))
    }

    fn begin_read(&self) -> Result<(Transaction, &RequestStoreTable)> {
        Err(anyhow!("Cannot read on a dummy request store"))
    }

    fn scan(&self) -> Result<HashMap<PathBuf, RequestEntity>> {
        Err(anyhow!("Cannot scan on a dummy request store"))
    }
}

fn generate_request_store(path: &Path) -> Result<Arc<RequestStoreWrapper>> {
    let db_client =
        ReDbClient::new(path.join(COLLECTION_STATE_DB_NAME))?
            .with_table(&TABLE_REQUESTS)?;

    let request_store = Arc::new(RequestStoreImpl::new(db_client));
    Ok(Arc::new(RequestStoreWrapper(request_store)))
}

impl StateDbManagerImpl {
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let request_store = generate_request_store(path.as_ref())?;
        let path = path.as_ref().to_path_buf();
        Ok(Self {
            path: ArcSwap::new(Arc::new(path)),
            request_store: ArcSwap::new(request_store),
        })
    }
}

#[async_trait::async_trait]
impl StateDbManager for StateDbManagerImpl {
    fn request_store(&self) -> Arc<dyn RequestStore> {
        self.request_store.load_full().0.clone()
    }

    // Temporarily drop the db for fs renaming
    async fn reset(&self, fs: Arc<dyn FileSystem>, new_path: &Path) -> Result<()> {
        let old_store = self.request_store.swap(
            Arc::new(RequestStoreWrapper(Arc::new(DummyRequestStore::new()))),
        );
        std::mem::drop(old_store);

        let old_path = self.path.swap(Arc::new(new_path.to_path_buf()));
        fs.rename(&old_path, &new_path, RenameOptions::default()).await?;

        let new_store = generate_request_store(new_path)?;
        self.request_store.store(new_store);

        Ok(())
    }

}
