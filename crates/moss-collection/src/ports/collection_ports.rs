use anyhow::Result;
use std::path::PathBuf;

use crate::models::indexing::IndexedCollection;

#[async_trait::async_trait]
pub trait CollectionIndexer: Send + Sync {
    async fn index(&self, path: &PathBuf) -> Result<IndexedCollection>;
}
