use anyhow::Result;
use std::sync::Arc;

pub struct SledManager {
    collections_tree: Arc<sled::Tree>,
    db: sled::Db,
}

impl SledManager {
    pub fn new(db: sled::Db) -> Result<Self> {
        Ok(Self {
            collections_tree: Arc::new(db.open_tree("collections")?),
            db,
        })
    }

    pub fn collections_tree(&self) -> Arc<sled::Tree> {
        Arc::clone(&self.collections_tree)
    }
}
