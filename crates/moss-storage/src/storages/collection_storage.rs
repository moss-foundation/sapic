pub mod entities;
pub mod stores;
mod tables;

use anyhow::Result;
use async_trait::async_trait;
use entities::variable_store_entities::VariableEntity;
use moss_db::primitives::AnyValue;
use moss_db::{DatabaseClient, DatabaseResult, ReDbClient, Transaction};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use stores::variable_store::VariableStoreImpl;
use tables::{TABLE_VARIABLES, UNIT_STORE};

use crate::{
    CollectionStorage,
    common::item_store::{ItemStore, ItemStoreImpl},
    primitives::segkey::SegKeyBuf,
    storage::Transactional,
};

const DB_NAME: &str = "state.db";

// <environment_name>:<variable_name>
pub trait VariableStore: Send + Sync {
    fn list_variables(&self) -> DatabaseResult<HashMap<SegKeyBuf, VariableEntity>>;
}

pub struct CollectionStorageImpl {
    db_client: ReDbClient,
    variable_store: Arc<dyn VariableStore>,
    unit_store: Arc<dyn ItemStore<SegKeyBuf, AnyValue>>,
}

impl CollectionStorageImpl {
    pub fn new(path: &Path) -> Result<Self> {
        let db_client = ReDbClient::new(path.join(DB_NAME))?
            .with_table(&UNIT_STORE)?
            .with_table(&TABLE_VARIABLES)?;

        let variable_store = Arc::new(VariableStoreImpl::new(db_client.clone()));
        let unit_store = Arc::new(ItemStoreImpl::new(db_client.clone(), UNIT_STORE));

        Ok(Self {
            db_client,
            variable_store,
            unit_store,
        })
    }
}

#[async_trait]
impl Transactional for CollectionStorageImpl {
    async fn begin_write(&self) -> DatabaseResult<Transaction> {
        self.db_client.begin_write()
    }

    async fn begin_read(&self) -> DatabaseResult<Transaction> {
        self.db_client.begin_read()
    }
}

impl CollectionStorage for CollectionStorageImpl {
    fn variable_store(&self) -> Arc<dyn VariableStore> {
        self.variable_store.clone()
    }

    fn unit_store(&self) -> Arc<dyn ItemStore<SegKeyBuf, AnyValue>> {
        self.unit_store.clone()
    }
}
