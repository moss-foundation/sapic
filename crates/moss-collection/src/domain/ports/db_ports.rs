use anyhow::Result;

use crate::domain::models::storage::CollectionMetadataEntity;

pub trait CollectionMetadataStore: Send + Sync + 'static {
    fn get_all_items(&self) -> Result<Vec<(String, CollectionMetadataEntity)>>;
    fn put_collection_item(&self, source: String, item: CollectionMetadataEntity) -> Result<()>;
    fn remove_collection_item(&self, source: String) -> Result<()>;
}

pub trait CollectionRequestSubstore: Send + Sync + 'static {}
