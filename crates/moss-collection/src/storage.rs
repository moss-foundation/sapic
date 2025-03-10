pub mod collection_metadata_store;
pub mod collection_request_substore;
pub mod collection_store;

pub use collection_metadata_store::SledCollectionMetadataStore;
pub use collection_request_substore::SledCollectionRequestSubstore;

use anyhow::Result;
use moss_db::{bincode_table::BincodeTable, Transaction};
use std::{future::Future, path::PathBuf, pin::Pin};

use crate::models::storage::CollectionMetadataEntity;

#[cfg_attr(test, mockall::automock)]
pub trait CollectionMetadataStore: Send + Sync + 'static {
    fn get_all_items(&self) -> Result<Vec<(PathBuf, CollectionMetadataEntity)>>;
    fn put_collection_item(&self, path: PathBuf, item: CollectionMetadataEntity) -> Result<()>;
    fn remove_collection_item(&self, path: PathBuf) -> Result<CollectionMetadataEntity>;
}

#[cfg_attr(test, mockall::automock)]
pub trait CollectionRequestSubstore: Send + Sync + 'static {}

pub(crate) type CollectionTable<'a> = BincodeTable<'a, String, CollectionMetadataEntity>;

pub trait CollectionStore: Send + Sync + 'static {
    // async fn create_collection(
    //     &self,
    //     f: Box<
    //         dyn FnOnce(
    //                 Transaction,
    //                 CollectionTable,
    //             ) -> Pin<Box<dyn Future<Output = Result<()>> + Send>>
    //             + Send,
    //     >,
    // ) -> Result<()>;

    fn begin_write(&self) -> Result<(Transaction, &CollectionTable)>;
}
