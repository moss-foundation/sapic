pub mod collection_store;

pub mod request_store;
pub mod state_db_manager;

use anyhow::Result;
use moss_db::{bincode_table::BincodeTable, Transaction};
use std::{collections::HashMap, path::PathBuf, sync::Arc};

use crate::models::storage::{CollectionEntity, RequestEntity};

pub trait CollectionRequestSubstore: Send + Sync + 'static {}

pub(crate) type CollectionTable<'a> = BincodeTable<'a, String, CollectionEntity>;

pub trait CollectionStore: Send + Sync + 'static {
    fn begin_write(&self) -> Result<(Transaction, &CollectionTable)>;
    fn begin_read(&self) -> Result<(Transaction, &CollectionTable)>;
    fn get_all_items(&self) -> Result<Vec<(PathBuf, CollectionEntity)>>;
}

// ------------------------------------------------------------------------------------------------

pub(crate) type RequestStoreTable<'a> = BincodeTable<'a, String, RequestEntity>;

pub trait RequestStore: Send + Sync + 'static {
    fn begin_write(&self) -> Result<(Transaction, &RequestStoreTable)>;
    fn begin_read(&self) -> Result<(Transaction, &RequestStoreTable)>;
    fn scan(&self) -> Result<HashMap<PathBuf, RequestEntity>>;
}

pub trait StateDbManager: Send + Sync + 'static {
    fn request_store(&self) -> Arc<dyn RequestStore>;
}
