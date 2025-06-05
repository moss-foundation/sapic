use moss_db::{DatabaseClient, DatabaseResult, ReDbClient, Transaction, primitives::AnyValue};
use std::sync::Arc;

use crate::{
    global_storage::stores::SessionLogCache,
    primitives::segkey::SegKeyBuf,
    storage::{
        SegBinTable,
        operations::{
            GetItem, ListByPrefix, PutItem, RemoveItem, TransactionalGetItem,
            TransactionalListByPrefix, TransactionalPutItem, TransactionalRemoveItem,
            TransactionalTruncate, Truncate,
        },
    },
};

pub struct SessionLogCacheImpl {
    client: ReDbClient,
    table: Arc<SegBinTable>,
}

impl SessionLogCacheImpl {
    pub fn new(client: ReDbClient, table: Arc<SegBinTable>) -> Self {
        Self { client, table }
    }
}

impl ListByPrefix for SessionLogCacheImpl {
    type Key = SegKeyBuf;
    type Entity = AnyValue;

    fn list_by_prefix(&self, prefix: &str) -> DatabaseResult<Vec<(Self::Key, Self::Entity)>> {
        let read_txn = self.client.begin_read()?;
        self.table.scan_by_prefix(&read_txn, prefix)
    }
}

impl TransactionalListByPrefix for SessionLogCacheImpl {
    type Key = SegKeyBuf;
    type Entity = AnyValue;

    fn list_by_prefix(
        &self,
        txn: &Transaction,
        prefix: &str,
    ) -> DatabaseResult<Vec<(Self::Key, Self::Entity)>> {
        self.table.scan_by_prefix(txn, prefix)
    }
}

impl PutItem for SessionLogCacheImpl {
    type Key = SegKeyBuf;
    type Entity = AnyValue;

    fn put(&self, key: Self::Key, entity: Self::Entity) -> DatabaseResult<()> {
        let mut txn = self.client.begin_write()?;
        self.table.insert(&mut txn, key, &entity)?;
        txn.commit()
    }
}

impl TransactionalPutItem for SessionLogCacheImpl {
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

impl RemoveItem for SessionLogCacheImpl {
    type Key = SegKeyBuf;
    type Entity = AnyValue;

    fn remove(&self, key: Self::Key) -> DatabaseResult<Self::Entity> {
        let mut txn = self.client.begin_write()?;
        let value = self.table.remove(&mut txn, key)?;
        txn.commit()?;
        Ok(value)
    }
}

impl TransactionalRemoveItem for SessionLogCacheImpl {
    type Key = SegKeyBuf;
    type Entity = AnyValue;

    fn remove(&self, txn: &mut Transaction, key: Self::Key) -> DatabaseResult<Self::Entity> {
        self.table.remove(txn, key)
    }
}

impl GetItem for SessionLogCacheImpl {
    type Key = SegKeyBuf;
    type Entity = AnyValue;

    fn get(&self, key: Self::Key) -> DatabaseResult<Self::Entity> {
        let read_txn = self.client.begin_read()?;
        self.table.read(&read_txn, key)
    }
}

impl TransactionalGetItem for SessionLogCacheImpl {
    type Key = SegKeyBuf;
    type Entity = AnyValue;

    fn get(&self, txn: &Transaction, key: Self::Key) -> DatabaseResult<Self::Entity> {
        self.table.read(&txn, key)
    }
}

impl Truncate for SessionLogCacheImpl {
    fn truncate(&self) -> DatabaseResult<()> {
        let mut write_txn = self.client.begin_write()?;
        self.table.truncate(&mut write_txn)
    }
}

impl TransactionalTruncate for SessionLogCacheImpl {
    fn truncate(&self, txn: &mut Transaction) -> DatabaseResult<()> {
        self.table.truncate(txn)
    }
}

impl SessionLogCache for SessionLogCacheImpl {}
