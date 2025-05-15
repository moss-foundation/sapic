use anyhow::Result;
use moss_db::{DatabaseError, ReDbClient};
use std::collections::HashMap;

use crate::workspace_storage::{
    TABLE_VARIABLES, VariableKey, VariableStore, entities::variable_store_entities::VariableEntity,
    tables::VariableStoreTable,
};

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
    fn list_variables(&self) -> Result<HashMap<VariableKey, VariableEntity>, DatabaseError> {
        todo!()
    }
}
