use moss_db::primitives::AnyValue;
use moss_db::{DatabaseClient, DatabaseResult, ReDbClient, Transaction};
use std::sync::Arc;

use crate::global_storage::stores::GlobalItemStore;
use crate::primitives::segkey::SegKeyBuf;
use crate::storage::SegBinTable;
use crate::storage::operations::{
    ListByPrefix, PutItem, RemoveItem, TransactionalListByPrefix, TransactionalPutItem,
    TransactionalRemoveItem,
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
    type Entity = AnyValue;

    fn list_by_prefix(&self, prefix: &str) -> DatabaseResult<Vec<(Self::Key, Self::Entity)>> {
        let read_txn = self.client.begin_read()?;
        self.table.scan_by_prefix(&read_txn, prefix)
    }
}

impl TransactionalListByPrefix for GlobalItemStoreImpl {
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

impl PutItem for GlobalItemStoreImpl {
    type Key = SegKeyBuf;
    type Entity = AnyValue;

    fn put(&self, key: Self::Key, entity: Self::Entity) -> DatabaseResult<()> {
        let mut txn = self.client.begin_write()?;
        self.table.insert(&mut txn, key, &entity)
    }
}

impl TransactionalPutItem for GlobalItemStoreImpl {
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
impl RemoveItem for GlobalItemStoreImpl {
    type Key = SegKeyBuf;

    fn remove(&self, key: Self::Key) -> DatabaseResult<()> {
        let mut txn = self.client.begin_read()?;
        self.table.remove(&mut txn, key)?;
        Ok(())
    }
}

impl TransactionalRemoveItem for GlobalItemStoreImpl {
    type Key = SegKeyBuf;

    fn remove(&self, txn: &mut Transaction, key: Self::Key) -> DatabaseResult<()> {
        self.table.remove(txn, key)?;
        Ok(())
    }
}

impl GlobalItemStore for GlobalItemStoreImpl {}
