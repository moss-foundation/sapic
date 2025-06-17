use moss_db::{DatabaseClient, DatabaseResult, ReDbClient, Transaction, primitives::AnyValue};
use std::sync::Arc;

use crate::{
    GlobalStorage,
    global_storage::stores::GlobalLogStore,
    primitives::segkey::SegKeyBuf,
    storage::{
        SegBinTable,
        operations::{
            GetItem, PutItem, RemoveItem, TransactionalGetItem, TransactionalPutItem,
            TransactionalRemoveItem,
        },
    },
};

pub struct GlobalLogStoreImpl {
    client: ReDbClient,
    table: Arc<SegBinTable>,
}

impl GlobalLogStoreImpl {
    pub fn new(client: ReDbClient, table: Arc<SegBinTable>) -> Self {
        Self { client, table }
    }
}

impl PutItem for GlobalLogStoreImpl {
    type Key = SegKeyBuf;
    type Entity = AnyValue;

    fn put(&self, key: Self::Key, entity: Self::Entity) -> DatabaseResult<()> {
        let mut txn = self.client.begin_write()?;
        self.table.insert(&mut txn, key, &entity)?;
        txn.commit()
    }
}

impl TransactionalPutItem for GlobalLogStoreImpl {
    type Key = SegKeyBuf;
    type Entity = AnyValue;

    fn put(
        &self,
        txn: &mut Transaction,
        key: Self::Key,
        entity: Self::Entity,
    ) -> DatabaseResult<()> {
        self.table.insert(txn, key, &entity)
    }
}

impl RemoveItem for GlobalLogStoreImpl {
    type Key = SegKeyBuf;
    type Entity = AnyValue;

    fn remove(&self, key: Self::Key) -> DatabaseResult<Self::Entity> {
        let mut txn = self.client.begin_write()?;
        let value = self.table.remove(&mut txn, key)?;
        txn.commit()?;
        Ok(value)
    }
}

impl TransactionalRemoveItem for GlobalLogStoreImpl {
    type Key = SegKeyBuf;
    type Entity = AnyValue;

    fn remove(&self, txn: &mut Transaction, key: Self::Key) -> DatabaseResult<Self::Entity> {
        self.table.remove(txn, key)
    }
}

impl GetItem for GlobalLogStoreImpl {
    type Key = SegKeyBuf;
    type Entity = AnyValue;

    fn get(&self, key: Self::Key) -> DatabaseResult<Self::Entity> {
        let read_txn = self.client.begin_read()?;
        self.table.read(&read_txn, key)
    }
}

impl TransactionalGetItem for GlobalLogStoreImpl {
    type Key = SegKeyBuf;
    type Entity = AnyValue;

    fn get(&self, txn: &Transaction, key: Self::Key) -> DatabaseResult<Self::Entity> {
        self.table.read(&txn, key)
    }
}

impl GlobalLogStore for GlobalLogStoreImpl {}
