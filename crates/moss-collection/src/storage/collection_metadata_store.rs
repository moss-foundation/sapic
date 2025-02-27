use anyhow::Result;
use std::{path::PathBuf, sync::Arc};

use crate::models::storage::CollectionMetadataEntity;

use super::CollectionMetadataStore;

pub struct SledCollectionMetadataStore {
    tree: Arc<sled::Tree>,
}

impl SledCollectionMetadataStore {
    pub fn new(tree: Arc<sled::Tree>) -> Self {
        Self { tree }
    }
}

impl CollectionMetadataStore for SledCollectionMetadataStore {
    fn get_all_items(&self) -> Result<Vec<(PathBuf, CollectionMetadataEntity)>> {
        let mut result = Vec::new();

        for iter_result in self.tree.iter() {
            let (key, value) = iter_result?;

            result.push((
                PathBuf::from(String::from_utf8_lossy(&key).to_string()), // Not sure if this is the best way to transform it.
                bincode::deserialize::<CollectionMetadataEntity>(&value)?,
            ));
        }

        Ok(result)
    }

    fn put_collection_item(&self, path: PathBuf, item: CollectionMetadataEntity) -> Result<()> {
        let value = bincode::serialize(&item)?;
        self.tree.insert(path.to_string_lossy().as_bytes(), value)?;

        Ok(())
    }

    fn remove_collection_item(&self, path: PathBuf) -> Result<()> {
        self.tree.remove(path.to_string_lossy().as_bytes())?;

        Ok(())
    }
}
