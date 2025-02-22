use anyhow::Result;
use std::path::PathBuf;

use crate::models::storage::CollectionMetadataEntity;

pub trait CollectionMetadataStore: Send + Sync + 'static {
    fn get_all_items(&self) -> Result<Vec<(PathBuf, CollectionMetadataEntity)>>;
    fn put_collection_item(&self, path: PathBuf, item: CollectionMetadataEntity) -> Result<()>;
    fn remove_collection_item(&self, path: PathBuf) -> Result<()>;
}

pub trait CollectionRequestSubstore: Send + Sync + 'static {}
