use anyhow::Result;
use std::sync::Arc;

use crate::domain::{models::CollectionDetails, ports::db_ports::CollectionStore};

pub struct SledCollectionStore {
    tree: Arc<sled::Tree>,
}

impl SledCollectionStore {
    pub fn new(tree: Arc<sled::Tree>) -> Self {
        Self { tree }
    }
}

impl CollectionStore for SledCollectionStore {
    fn put_collection_item(&self, item: CollectionDetails) -> Result<()> {
        let value = bincode::serialize(&item)?;
        self.tree.insert(item.source().as_bytes(), value)?;

        Ok(())
    }

    fn remove_collection_item(&self, source: String) -> Result<()> {
        self.tree.remove(source)?;

        Ok(())
    }
}
