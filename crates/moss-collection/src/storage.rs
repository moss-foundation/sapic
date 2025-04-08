pub mod request_store;
pub mod state_db_manager;

use anyhow::Result;
use moss_db::{bincode_table::BincodeTable, Transaction};
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use std::path::Path;
use moss_fs::FileSystem;
use crate::models::storage::RequestEntity;

pub(crate) type RequestStoreTable<'a> = BincodeTable<'a, String, RequestEntity>;

pub trait RequestStore: Send + Sync + 'static {
    fn begin_write(&self) -> Result<(Transaction, &RequestStoreTable)>;
    fn begin_read(&self) -> Result<(Transaction, &RequestStoreTable)>;
    fn scan(&self) -> Result<HashMap<PathBuf, RequestEntity>>;
}

#[async_trait::async_trait]
pub trait StateDbManager: Send + Sync + 'static {
    fn request_store(&self) -> Arc<dyn RequestStore>;
    async fn reset(&mut self, fs: Arc<dyn FileSystem>, new_path: &Path) -> Result<()>;
}
