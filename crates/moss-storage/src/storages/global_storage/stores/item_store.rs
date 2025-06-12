use moss_db::{
    DatabaseClient, DatabaseResult, ReDbClient, Transaction, anyvalue_enum::AnyValueEnum,
};
use std::sync::Arc;

use crate::{
    global_storage::stores::GlobalItemStore,
    primitives::segkey::SegKeyBuf,
    storage::{
        SegBinTable,
        operations::{
            GetItem, ListByPrefix, PutItem, RemoveItem, TransactionalGetItem,
            TransactionalListByPrefix, TransactionalPutItem, TransactionalRemoveItem,
        },
    },
};

pub struct GlobalItemStoreImpl {
    client: ReDbClient,
    table: Arc<SegBinTable>,
}

impl GlobalItemStoreImpl {
    pub fn new(client: ReDbClient, table: Arc<SegBinTable>) -> Self {
        Self { client, table }
    }
}

impl ListByPrefix for GlobalItemStoreImpl {
    type Key = SegKeyBuf;
    type Entity = AnyValueEnum;

    fn list_by_prefix(&self, prefix: &str) -> DatabaseResult<Vec<(Self::Key, Self::Entity)>> {
        let read_txn = self.client.begin_read()?;
        self.table.scan_by_prefix(&read_txn, prefix)
    }
}

impl TransactionalListByPrefix for GlobalItemStoreImpl {
    type Key = SegKeyBuf;
    type Entity = AnyValueEnum;

    fn list_by_prefix(
        &self,
        txn: &Transaction,
        prefix: &str,
    ) -> DatabaseResult<Vec<(Self::Key, Self::Entity)>> {
        self.table.scan_by_prefix(txn, prefix)
    }
}

impl PutItem for GlobalItemStoreImpl {
    type Key = SegKeyBuf;
    type Entity = AnyValueEnum;

    fn put(&self, key: Self::Key, entity: Self::Entity) -> DatabaseResult<()> {
        let mut txn = self.client.begin_write()?;
        self.table.insert(&mut txn, key, &entity)?;
        txn.commit()
    }
}

impl TransactionalPutItem for GlobalItemStoreImpl {
    type Key = SegKeyBuf;
    type Entity = AnyValueEnum;

    fn put(
        &self,
        txn: &mut Transaction,
        key: Self::Key,
        entity: Self::Entity,
    ) -> DatabaseResult<()> {
        self.table.insert(txn, key, &entity)
    }
}
impl RemoveItem for GlobalItemStoreImpl {
    type Key = SegKeyBuf;
    type Entity = AnyValueEnum;

    fn remove(&self, key: Self::Key) -> DatabaseResult<Self::Entity> {
        let mut txn = self.client.begin_write()?;
        let value = self.table.remove(&mut txn, key)?;
        txn.commit()?;
        Ok(value)
    }
}

impl TransactionalRemoveItem for GlobalItemStoreImpl {
    type Key = SegKeyBuf;
    type Entity = AnyValueEnum;

    fn remove(&self, txn: &mut Transaction, key: Self::Key) -> DatabaseResult<Self::Entity> {
        self.table.remove(txn, key)
    }
}

impl GetItem for GlobalItemStoreImpl {
    type Key = SegKeyBuf;
    type Entity = AnyValueEnum;

    fn get(&self, key: Self::Key) -> DatabaseResult<Self::Entity> {
        let read_txn = self.client.begin_read()?;
        self.table.read(&read_txn, key)
    }
}

impl TransactionalGetItem for GlobalItemStoreImpl {
    type Key = SegKeyBuf;
    type Entity = AnyValueEnum;

    fn get(&self, txn: &Transaction, key: Self::Key) -> DatabaseResult<Self::Entity> {
        self.table.read(&txn, key)
    }
}

impl GlobalItemStore for GlobalItemStoreImpl {}
