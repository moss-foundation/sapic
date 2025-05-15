pub mod entities;
pub mod stores;

mod tables;

use anyhow::Result;
use async_trait::async_trait;
use entities::variable_store_entities::VariableEntity;
use moss_db::{
    DatabaseClient, ReDbClient,
    bincode_table::BincodeTable,
    common::{DatabaseError, Transaction},
    primitives::AnyValue,
};
use std::{collections::HashMap, path::Path, sync::Arc};
use stores::variable_store::VariableStoreImpl;
use tables::{ITEM_STORE, TABLE_VARIABLES};

use crate::{
    common::item_store::{ItemStore, store_impl::ItemStoreImpl},
    primitives::segkey::SegKeyBuf,
    storage::Transactional,
};

use super::WorkspaceStorage;

const WORKSPACE_STATE_DB_NAME: &str = "state.db";

// <environment_name>:<variable_name>
pub type VariableKey = String;
pub trait VariableStore: Send + Sync {
    fn list_variables(&self) -> Result<HashMap<VariableKey, VariableEntity>, DatabaseError>;
}

pub struct WorkspaceStorageImpl {
    db_client: ReDbClient,
    variable_store: Arc<dyn VariableStore>,
    item_store: Arc<dyn ItemStore<SegKeyBuf, AnyValue>>,
}

impl WorkspaceStorageImpl {
    pub fn new(path: &Path) -> Result<Self> {
        let db_client = ReDbClient::new(path.join(WORKSPACE_STATE_DB_NAME))?
            .with_table(&ITEM_STORE)?
            .with_table(&TABLE_VARIABLES)?;

        let variable_store = Arc::new(VariableStoreImpl::new(db_client.clone()));
        let item_store = Arc::new(ItemStoreImpl::new(db_client.clone(), ITEM_STORE));

        Ok(Self {
            db_client,
            variable_store,
            item_store,
        })
    }
}

#[async_trait]
impl Transactional for WorkspaceStorageImpl {
    async fn begin_write(&self) -> Result<Transaction, DatabaseError> {
        self.db_client.begin_write()
    }

    async fn begin_read(&self) -> Result<Transaction, DatabaseError> {
        self.db_client.begin_read()
    }
}

impl WorkspaceStorage for WorkspaceStorageImpl {
    fn variable_store(&self) -> Arc<dyn VariableStore> {
        self.variable_store.clone()
    }

    fn item_store(&self) -> Arc<dyn ItemStore<SegKeyBuf, AnyValue>> {
        self.item_store.clone()
    }
}
