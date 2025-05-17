pub mod entities;
pub mod stores;

mod tables;

use anyhow::Result;
use async_trait::async_trait;
use moss_db::{
    DatabaseClient, DatabaseResult, ReDbClient, common::Transaction, primitives::AnyValue,
};
use std::{collections::HashMap, path::Path, sync::Arc};
use stores::variable_store::VariableStoreImpl;
use tables::{ITEM_STORE, TABLE_VARIABLES};

use crate::{
    WorkspaceStorage,
    common::item_store::{ItemStore, ItemStoreImpl},
    primitives::segkey::SegKeyBuf,
    storage::Transactional,
};

const DB_NAME: &str = "state.db";

// <environment_name>:<variable_name>
pub trait VariableStore: Send + Sync {
    fn list_variables(&self) -> DatabaseResult<HashMap<SegKeyBuf, AnyValue>>;
}

pub struct WorkspaceStorageImpl {
    db_client: ReDbClient,
    variable_store: Arc<dyn VariableStore>,
    item_store: Arc<dyn ItemStore<SegKeyBuf, AnyValue>>,
}

impl WorkspaceStorageImpl {
    pub fn new(path: &Path) -> Result<Self> {
        let db_client = ReDbClient::new(path.join(DB_NAME))?
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
    async fn begin_write(&self) -> DatabaseResult<Transaction> {
        self.db_client.begin_write()
    }

    async fn begin_read(&self) -> DatabaseResult<Transaction> {
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

// struct WorkspaceStorageImpl {
//     inner: Storage,
// }

// impl WorkspaceStorage for WorkspaceStorageImpl {
//     fn variable_store(&self) -> Arc<dyn VariableStore> {
//         self.inner.table(TypeId::of::<VariableTable>())
//     }

//     fn item_store(&self) -> Arc<dyn ItemStore<SegKeyBuf, AnyValue>> {
//         self.inner.table(TypeId::of::<ItemTable>())
//     }
// }
