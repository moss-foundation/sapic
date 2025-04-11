pub mod request_store;
pub mod state_db_manager;

use anyhow::Result;
use moss_db::{bincode_table::BincodeTable, Transaction};
use std::{collections::HashMap, path::PathBuf, sync::Arc};

use crate::models::storage::RequestEntity;

pub(crate) type RequestStoreTable<'a> = BincodeTable<'a, String, RequestEntity>;

pub trait RequestStore: Send + Sync + 'static {
    fn begin_write(&self) -> Result<(Transaction, &RequestStoreTable)>;
    fn begin_read(&self) -> Result<(Transaction, &RequestStoreTable)>;
    fn scan(&self) -> Result<HashMap<PathBuf, RequestEntity>>;
}

pub trait StateDbManager: Send + Sync + 'static {
    fn reload(&self, path: PathBuf, after_drop: Box<dyn FnOnce() -> Result<()>>) -> Result<()>;
    fn request_store(&self) -> Arc<dyn RequestStore>;
}
