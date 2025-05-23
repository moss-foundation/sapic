use moss_db::ReDbClient;
use std::sync::Arc;

use crate::storage::SegBinTable;
use crate::storages::collection_storage::stores::CollectionUnitStore;

pub struct CollectionUnitStoreImpl {
    #[allow(dead_code)]
    client: ReDbClient,
    #[allow(dead_code)]
    table: Arc<SegBinTable>,
}

impl CollectionUnitStoreImpl {
    pub fn new(client: ReDbClient, table: Arc<SegBinTable>) -> Self {
        Self { client, table }
    }
}

impl CollectionUnitStore for CollectionUnitStoreImpl {}
