use moss_db::{DatabaseClient, DatabaseResult, ReDbClient, Transaction, primitives::AnyValue};
use std::sync::Arc;

use crate::{
    primitives::segkey::SegKeyBuf,
    storage::{SegBinTable, operations::*},
    storages::collection_storage::stores::CollectionResourceStore,
};

pub struct CollectionResourceStoreImpl {
    #[allow(dead_code)]
    client: ReDbClient,
    #[allow(dead_code)]
    table: Arc<SegBinTable>,
}

impl CollectionResourceStoreImpl {
    pub fn new(client: ReDbClient, table: Arc<SegBinTable>) -> Self {
        Self { client, table }
    }
}

impl ListByPrefix for CollectionResourceStoreImpl {
    type Key = SegKeyBuf;
    type Entity = AnyValue;

    fn list_by_prefix(&self, prefix: &str) -> DatabaseResult<Vec<(Self::Key, Self::Entity)>> {
        let read_txn = self.client.begin_read()?;
        self.table.scan_by_prefix(&read_txn, prefix)
    }
}

impl TransactionalListByPrefix for CollectionResourceStoreImpl {
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

impl PutItem for CollectionResourceStoreImpl {
    type Key = SegKeyBuf;
    type Entity = AnyValue;

    fn put(&self, key: Self::Key, entity: Self::Entity) -> DatabaseResult<()> {
        let mut write_txn = self.client.begin_write()?;
        self.table.insert(&mut write_txn, key, &entity)?;
        write_txn.commit()
    }
}

impl TransactionalPutItem for CollectionResourceStoreImpl {
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

impl GetItem for CollectionResourceStoreImpl {
    type Key = SegKeyBuf;
    type Entity = AnyValue;

    fn get(&self, key: Self::Key) -> DatabaseResult<Self::Entity> {
        let read_txn = self.client.begin_read()?;
        self.table.read(&read_txn, key)
    }
}

impl TransactionalGetItem for CollectionResourceStoreImpl {
    type Key = SegKeyBuf;
    type Entity = AnyValue;

    fn get(&self, txn: &Transaction, key: Self::Key) -> DatabaseResult<Self::Entity> {
        self.table.read(txn, key)
    }
}

impl RemoveItem for CollectionResourceStoreImpl {
    type Key = SegKeyBuf;
    type Entity = AnyValue;

    fn remove(&self, key: Self::Key) -> DatabaseResult<Self::Entity> {
        let mut write_txn = self.client.begin_write()?;
        let value = self.table.remove(&mut write_txn, key)?;
        write_txn.commit()?;
        Ok(value)
    }
}

impl TransactionalRemoveItem for CollectionResourceStoreImpl {
    type Key = SegKeyBuf;
    type Entity = AnyValue;

    fn remove(&self, txn: &mut Transaction, key: Self::Key) -> DatabaseResult<Self::Entity> {
        self.table.remove(txn, key)
    }
}

impl CollectionResourceStore for CollectionResourceStoreImpl {}
