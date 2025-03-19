pub mod collection_store;
pub mod state_db_manager;

use anyhow::Result;
use moss_db::{bincode_table::BincodeTable, Transaction};
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, sync::Arc};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CollectionEntity {
    pub order: Option<usize>,
}

pub(crate) type CollectionStoreTable<'a> = BincodeTable<'a, String, CollectionEntity>;

pub trait CollectionStore: Send + Sync + 'static {
    fn begin_write(&self) -> Result<(Transaction, &CollectionStoreTable)>;
    fn begin_read(&self) -> Result<(Transaction, &CollectionStoreTable)>;
    fn scan(&self) -> Result<Vec<(PathBuf, CollectionEntity)>>;
}

pub trait StateDbManager: Send + Sync + 'static {
    fn collection_store(&self) -> Arc<dyn CollectionStore>;
}
