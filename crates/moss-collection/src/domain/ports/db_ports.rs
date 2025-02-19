use crate::domain::models::CollectionDetails;
use anyhow::Result;

pub trait CollectionStore: Send + Sync + 'static {
    fn put_collection_item(&self, item: CollectionDetails) -> Result<()>;
    fn remove_collection_item(&self, source: String) -> Result<()>;
}

pub trait CollectionRequestSubstore: Send + Sync + 'static {}
