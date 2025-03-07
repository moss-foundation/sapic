pub mod collection_metadata_store;
pub mod collection_request_substore;

pub use collection_metadata_store::SledCollectionMetadataStore;
pub use collection_request_substore::SledCollectionRequestSubstore;
use std::collections::HashMap;

use crate::models::storage::CollectionMetadataEntity;
use anyhow::Result;
use dashmap::DashMap;
use std::path::PathBuf;
use std::sync::RwLock;

#[cfg_attr(test, mockall::automock)]
pub trait CollectionMetadataStore: Send + Sync + 'static {
    fn get_all_items(&self) -> Result<Vec<(PathBuf, CollectionMetadataEntity)>>;
    fn put_collection_item(&self, path: PathBuf, item: CollectionMetadataEntity) -> Result<()>;
    fn remove_collection_item(&self, path: PathBuf) -> Result<CollectionMetadataEntity>;
}

#[cfg_attr(test, mockall::automock)]
pub trait CollectionRequestSubstore: Send + Sync + 'static {}
