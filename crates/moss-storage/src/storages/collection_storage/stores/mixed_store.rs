use std::sync::Arc;

use moss_db::{DatabaseClient, DatabaseResult, ReDbClient, Transaction, primitives::AnyValue};

use crate::{
    collection_storage::stores::MixedStore,
    primitives::segkey::SegKeyBuf,
    storage::{SegBinTable, operations::*},
};

pub struct MixedStoreImpl {
    client: ReDbClient,
    table: Arc<SegBinTable>,
}

impl MixedStoreImpl {
    pub fn new(client: ReDbClient, table: Arc<SegBinTable>) -> Self {
        Self { client, table }
    }
}

impl ListByPrefix for MixedStoreImpl {
    type Key = SegKeyBuf;
    type Entity = AnyValue;

    fn list_by_prefix(&self, prefix: &str) -> DatabaseResult<Vec<(Self::Key, Self::Entity)>> {
        let read_txn = self.client.begin_read()?;
        self.table.scan_by_prefix(&read_txn, prefix)
    }
}

impl TransactionalListByPrefix for MixedStoreImpl {
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

impl PutItem for MixedStoreImpl {
    type Key = SegKeyBuf;
    type Entity = AnyValue;

    fn put(&self, key: Self::Key, entity: Self::Entity) -> DatabaseResult<()> {
        let mut write_txn = self.client.begin_write()?;
        self.table.insert(&mut write_txn, key, &entity)?;
        write_txn.commit()
    }
}

impl TransactionalPutItem for MixedStoreImpl {
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

impl GetItem for MixedStoreImpl {
    type Key = SegKeyBuf;
    type Entity = AnyValue;

    fn get(&self, key: Self::Key) -> DatabaseResult<Self::Entity> {
        let read_txn = self.client.begin_read()?;
        self.table.read(&read_txn, key)
    }
}

impl TransactionalGetItem for MixedStoreImpl {
    type Key = SegKeyBuf;
    type Entity = AnyValue;

    fn get(&self, txn: &Transaction, key: Self::Key) -> DatabaseResult<Self::Entity> {
        self.table.read(txn, key)
    }
}

impl RemoveItem for MixedStoreImpl {
    type Key = SegKeyBuf;
    type Entity = AnyValue;

    fn remove(&self, key: Self::Key) -> DatabaseResult<Self::Entity> {
        let mut write_txn = self.client.begin_write()?;
        let value = self.table.remove(&mut write_txn, key)?;
        write_txn.commit()?;
        Ok(value)
    }
}

impl TransactionalRemoveItem for MixedStoreImpl {
    type Key = SegKeyBuf;
    type Entity = AnyValue;

    fn remove(&self, txn: &mut Transaction, key: Self::Key) -> DatabaseResult<Self::Entity> {
        self.table.remove(txn, key)
    }
}

impl MixedStore for MixedStoreImpl {}
