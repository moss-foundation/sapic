use moss_db::primitives::AnyValue;
use moss_db::{DatabaseResult, ReDbClient};
use std::collections::HashMap;

use crate::collection_storage::{TABLE_VARIABLES, VariableStore, tables::VariableStoreTable};
use crate::primitives::segkey::SegKeyBuf;

pub struct VariableStoreImpl {
    #[allow(dead_code)] // TODO: remove this, when we have a use for it
    client: ReDbClient,
    #[allow(dead_code)] // TODO: remove this, when we have a use for it
    table: VariableStoreTable<'static>,
}

impl VariableStoreImpl {
    pub fn new(client: ReDbClient) -> Self {
        Self {
            client,
            table: TABLE_VARIABLES,
        }
    }
}

impl VariableStore for VariableStoreImpl {
    fn list_variables(&self) -> DatabaseResult<HashMap<SegKeyBuf, AnyValue>> {
        todo!()
    }
}
