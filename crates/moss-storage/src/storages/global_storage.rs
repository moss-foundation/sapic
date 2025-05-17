pub mod entities;

mod tables;

use async_trait::async_trait;
use moss_db::{
    DatabaseClient, ReDbClient, Transaction, common::DatabaseError, primitives::AnyValue,
};
use std::{path::PathBuf, sync::Arc};
use tables::ITEM_STORE;

use crate::{
    common::item_store::{ItemStore, ItemStoreImpl},
    primitives::segkey::SegKeyBuf,
    storage::Transactional,
};

use super::GlobalStorage;

const DB_NAME: &str = "state.db";

pub struct GlobalStorageImpl {
    db_client: ReDbClient,
    item_store: Arc<dyn ItemStore<SegKeyBuf, AnyValue>>,
}

impl GlobalStorageImpl {
    pub fn new(path: &PathBuf) -> Result<Self, DatabaseError> {
        let db_client = ReDbClient::new(path.join(DB_NAME))?.with_table(&ITEM_STORE)?;

        let item_store = Arc::new(ItemStoreImpl::new(db_client.clone(), ITEM_STORE));

        Ok(Self {
            db_client,
            item_store,
        })
    }
}

#[async_trait]
impl Transactional for GlobalStorageImpl {
    async fn begin_write(&self) -> Result<Transaction, DatabaseError> {
        self.db_client.begin_write()
    }

    async fn begin_read(&self) -> Result<Transaction, DatabaseError> {
        self.db_client.begin_read()
    }
}

impl GlobalStorage for GlobalStorageImpl {
    fn item_store(&self) -> Arc<dyn ItemStore<SegKeyBuf, AnyValue>> {
        self.item_store.clone()
    }
}
