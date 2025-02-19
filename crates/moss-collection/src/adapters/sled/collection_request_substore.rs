use std::sync::Arc;

use crate::domain::ports::db_ports::CollectionRequestSubstore;

pub struct SledCollectionRequestSubstore {
    tree: Arc<sled::Tree>,
}

impl SledCollectionRequestSubstore {
    pub fn new(tree: Arc<sled::Tree>) -> Self {
        Self { tree }
    }
}

impl CollectionRequestSubstore for SledCollectionRequestSubstore {}
