use moss_applib::context::AnyAsyncContext;
use moss_db::ReDbClient;
use std::sync::Arc;

use crate::{common::VariableStore, storage::SegBinTable};

pub struct VariableStoreImpl {
    #[allow(unused)]
    client: ReDbClient,
    #[allow(dead_code)]
    table: Arc<SegBinTable>,
}

impl VariableStoreImpl {
    pub fn new(client: ReDbClient, table: Arc<SegBinTable>) -> Self {
        Self { client, table }
    }
}

impl<Context> VariableStore<Context> for VariableStoreImpl where Context: AnyAsyncContext {}
