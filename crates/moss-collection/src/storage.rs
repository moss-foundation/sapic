pub mod collection_request_substore;
pub mod collection_store;

pub use collection_request_substore::SledCollectionRequestSubstore;

use anyhow::Result;
use moss_db::{bincode_table::BincodeTable, Transaction};
use std::path::PathBuf;

use crate::models::storage::CollectionEntity;


#[cfg_attr(test, mockall::automock)]
pub trait CollectionRequestSubstore: Send + Sync + 'static {}

pub(crate) type CollectionTable<'a> = BincodeTable<'a, String, CollectionEntity>;

pub trait CollectionStore: Send + Sync + 'static {
    fn begin_write(&self) -> Result<(Transaction, &CollectionTable)>;
    fn begin_read(&self) -> Result<(Transaction, &CollectionTable)>;
    fn get_all_items(&self) -> Result<Vec<(PathBuf, CollectionEntity)>>;
}
