use moss_db::primitives::AnyValue;
use moss_db::{DatabaseClient, DatabaseResult, ReDbClient, Transaction};
use std::sync::Arc;

use crate::primitives::segkey::SegKeyBuf;
use crate::storage::SegBinTable;
use crate::storage::operations::{ListByPrefix, PutItem, RemoveItem, TransactionalGetItem};
use crate::workspace_storage::stores::WorkspaceItemStore;

pub struct WorkspaceItemStoreImpl {
    client: ReDbClient,
    table: Arc<SegBinTable>,
}

impl WorkspaceItemStoreImpl {
    pub fn new(client: ReDbClient, table: Arc<SegBinTable>) -> Self {
        Self { client, table }
    }
}

impl ListByPrefix for WorkspaceItemStoreImpl {
    type Key = SegKeyBuf;
    type Entity = AnyValue;

    fn list_by_prefix(&self, prefix: &str) -> DatabaseResult<Vec<(Self::Key, Self::Entity)>> {
        let read_txn = self.client.begin_read()?;
        self.table.scan_by_prefix(&read_txn, prefix)
    }
}

impl PutItem for WorkspaceItemStoreImpl {
    type Key = SegKeyBuf;
    type Entity = AnyValue;

    fn put(&self, key: Self::Key, entity: Self::Entity) -> DatabaseResult<()> {
        let mut write_txn = self.client.begin_write()?;
        self.table.insert(&mut write_txn, key, &entity)
    }
}

impl TransactionalGetItem for WorkspaceItemStoreImpl {
    type Key = SegKeyBuf;
    type Entity = AnyValue;

    fn get_item(&self, txn: &mut Transaction, key: Self::Key) -> DatabaseResult<Self::Entity> {
        self.table.read(txn, key)
    }
}

impl RemoveItem for WorkspaceItemStoreImpl {
    type Key = SegKeyBuf;

    fn remove(&self, key: Self::Key) -> DatabaseResult<()> {
        let mut write_txn = self.client.begin_write()?;
        self.table.remove(&mut write_txn, key)?;
        Ok(())
    }
}

impl WorkspaceItemStore for WorkspaceItemStoreImpl {}
