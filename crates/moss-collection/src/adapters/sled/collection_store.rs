use anyhow::Result;
use std::sync::Arc;

use crate::domain::{
    models::storage::CollectionMetadataEntity, ports::db_ports::CollectionMetadataStore,
};

pub struct SledCollectionMetadataStore {
    tree: Arc<sled::Tree>,
}

impl SledCollectionMetadataStore {
    pub fn new(tree: Arc<sled::Tree>) -> Self {
        Self { tree }
    }
}

impl CollectionMetadataStore for SledCollectionMetadataStore {
    fn get_all_items(&self) -> Result<Vec<(String, CollectionMetadataEntity)>> {
        let mut result = Vec::new();

        for iter_result in self.tree.iter() {
            let (key, value) = iter_result?;
            result.push((
                String::from_utf8_lossy(&key).to_string(),
                bincode::deserialize::<CollectionMetadataEntity>(&value)?,
            ));
        }

        Ok(result)
    }

    fn put_collection_item(&self, source: String, item: CollectionMetadataEntity) -> Result<()> {
        let value = bincode::serialize(&item)?;
        self.tree.insert(source, value)?;

        Ok(())
    }

    fn remove_collection_item(&self, source: String) -> Result<()> {
        self.tree.remove(source)?;

        Ok(())
    }
}
