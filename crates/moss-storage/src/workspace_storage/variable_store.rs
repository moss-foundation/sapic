use anyhow::Result;
use moss_db::{DatabaseError, ReDbClient, bincode_table::BincodeTable};
use std::collections::HashMap;

use super::{
    VariableKey, VariableStore, VariableStoreTable,
    entities::variable_store_entities::VariableEntity,
};

#[rustfmt::skip]
pub(in crate::workspace_storage) const TABLE_VARIABLES: BincodeTable<VariableKey, VariableEntity> = BincodeTable::new("variables");

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
