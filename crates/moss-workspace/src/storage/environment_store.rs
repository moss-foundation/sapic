use anyhow::Result;
use moss_db::{bincode_table::BincodeTable, ReDbClient};
use std::collections::HashMap;

use crate::models::{entities::EnvironmentEntity, types::EnvironmentName};

use super::{EnvironmentStore, EnvironmentStoreTable};

#[rustfmt::skip]
pub(super) const TABLE_ENVIRONMENTS: BincodeTable<String, EnvironmentEntity> = BincodeTable::new("environments");

pub struct EnvironmentStoreImpl {
    client: ReDbClient,
    table: EnvironmentStoreTable<'static>,
}

impl EnvironmentStoreImpl {
    pub fn new(client: ReDbClient) -> Self {
        Self {
            client,
            table: TABLE_ENVIRONMENTS,
        }
    }
}

impl EnvironmentStore for EnvironmentStoreImpl {
    fn scan(&self) -> Result<HashMap<EnvironmentName, EnvironmentEntity>> {
        todo!()
    }
}
