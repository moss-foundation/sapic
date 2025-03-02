pub mod collection_metadata_store;
pub mod collection_request_substore;

pub use collection_metadata_store::SledCollectionMetadataStore;
pub use collection_request_substore::SledCollectionRequestSubstore;

use anyhow::Result;
use std::path::PathBuf;

use crate::models::storage::CollectionMetadataEntity;

#[cfg_attr(test, mockall::automock)]
pub trait CollectionMetadataStore: Send + Sync + 'static {
    fn get_all_items(&self) -> Result<Vec<(PathBuf, CollectionMetadataEntity)>>;
    fn put_collection_item(&self, path: PathBuf, item: CollectionMetadataEntity) -> Result<()>;
    fn remove_collection_item(&self, path: PathBuf) -> Result<()>;
}

#[cfg_attr(test, mockall::automock)]
pub trait CollectionRequestSubstore: Send + Sync + 'static {}
