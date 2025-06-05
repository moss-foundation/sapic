use moss_db::{DatabaseClient, DatabaseResult, ReDbClient, Transaction, primitives::AnyValue};
use std::sync::Arc;

use crate::{
    global_storage::stores::AppLogCache,
    primitives::segkey::SegKeyBuf,
    storage::{
        SegBinTable,
        operations::{
            GetItem, ListByPrefix, PutItem, RemoveItem, Scan, TransactionalGetItem,
            TransactionalListByPrefix, TransactionalPutItem, TransactionalRemoveItem,
            TransactionalScan, TransactionalTruncate, Truncate,
        },
    },
};

pub struct AppLogCacheImpl {
    client: ReDbClient,
    table: Arc<SegBinTable>,
}

impl AppLogCacheImpl {
    pub fn new(client: ReDbClient, table: Arc<SegBinTable>) -> Self {
        Self { client, table }
    }
}

impl PutItem for AppLogCacheImpl {
    type Key = SegKeyBuf;
    type Entity = AnyValue;

    fn put(&self, key: Self::Key, entity: Self::Entity) -> DatabaseResult<()> {
        let mut txn = self.client.begin_write()?;
        self.table.insert(&mut txn, key, &entity)?;
        txn.commit()
    }
}

impl TransactionalPutItem for AppLogCacheImpl {
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

impl RemoveItem for AppLogCacheImpl {
    type Key = SegKeyBuf;
    type Entity = AnyValue;

    fn remove(&self, key: Self::Key) -> DatabaseResult<Self::Entity> {
        let mut txn = self.client.begin_write()?;
        let value = self.table.remove(&mut txn, key)?;
        txn.commit()?;
        Ok(value)
    }
}

impl TransactionalRemoveItem for AppLogCacheImpl {
    type Key = SegKeyBuf;
    type Entity = AnyValue;

    fn remove(&self, txn: &mut Transaction, key: Self::Key) -> DatabaseResult<Self::Entity> {
        self.table.remove(txn, key)
    }
}

impl GetItem for AppLogCacheImpl {
    type Key = SegKeyBuf;
    type Entity = AnyValue;

    fn get(&self, key: Self::Key) -> DatabaseResult<Self::Entity> {
        let read_txn = self.client.begin_read()?;
        self.table.read(&read_txn, key)
    }
}

impl TransactionalGetItem for AppLogCacheImpl {
    type Key = SegKeyBuf;
    type Entity = AnyValue;

    fn get(&self, txn: &Transaction, key: Self::Key) -> DatabaseResult<Self::Entity> {
        self.table.read(&txn, key)
    }
}

impl Truncate for AppLogCacheImpl {
    fn truncate(&self) -> DatabaseResult<()> {
        let mut write_txn = self.client.begin_write()?;
        self.table.truncate(&mut write_txn)?;
        write_txn.commit()
    }
}

impl TransactionalTruncate for AppLogCacheImpl {
    fn truncate(&self, txn: &mut Transaction) -> DatabaseResult<()> {
        self.table.truncate(txn)
    }
}

impl Scan for AppLogCacheImpl {
    type Key = SegKeyBuf;
    type Entity = AnyValue;

    fn scan(&self) -> DatabaseResult<Vec<(Self::Key, Self::Entity)>> {
        let read_txn = self.client.begin_read()?;
        self.table.scan(&read_txn).map(|res| res.collect())
    }
}

impl TransactionalScan for AppLogCacheImpl {
    type Key = SegKeyBuf;
    type Entity = AnyValue;

    fn scan(&self, txn: &Transaction) -> DatabaseResult<Vec<(Self::Key, Self::Entity)>> {
        self.table.scan(txn).map(|res| res.collect())
    }
}

impl AppLogCache for AppLogCacheImpl {}
