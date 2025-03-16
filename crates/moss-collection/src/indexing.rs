pub mod indexer;

use anyhow::Result;
use std::path::PathBuf;

use crate::models::indexing::IndexedCollection;

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait Indexer: Send + Sync {
    async fn index(&self, path: &PathBuf) -> Result<IndexedCollection>;
}
