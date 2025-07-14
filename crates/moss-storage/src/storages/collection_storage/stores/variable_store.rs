use moss_applib::context::AnyAsyncContext;
use moss_db::ReDbClient;
use std::sync::Arc;

use crate::{storage::SegBinTable, storages::collection_storage::stores::CollectionVariableStore};

pub struct CollectionVariableStoreImpl {
    #[allow(dead_code)]
    client: ReDbClient,
    #[allow(dead_code)]
    table: Arc<SegBinTable>,
}

impl CollectionVariableStoreImpl {
    pub fn new(client: ReDbClient, table: Arc<SegBinTable>) -> Self {
        Self { client, table }
    }
}

impl<Context> CollectionVariableStore<Context> for CollectionVariableStoreImpl where
    Context: AnyAsyncContext
{
}
