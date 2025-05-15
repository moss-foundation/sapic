use moss_db::{
    DatabaseClient, DatabaseResult, ReDbClient, Transaction,
    primitives::{AnyKey, AnyValue},
};

use crate::{primitives::segkey::SegKeyBuf, storage::operations::*};

use super::{ItemStore, ItemStoreTable};

pub struct ItemStoreImpl {
    client: ReDbClient,
    table: ItemStoreTable<'static>,
}

impl ItemStoreImpl {
    pub fn new(client: ReDbClient, table: ItemStoreTable<'static>) -> Self {
        Self { client, table }
    }
}

impl ItemStore<SegKeyBuf, AnyValue> for ItemStoreImpl {}

impl TransactionalPutItem for ItemStoreImpl {
    type Key = SegKeyBuf;
    type Entity = AnyValue;

    fn put(
        &self,
        txn: &mut Transaction,
        key: Self::Key,
        entity: Self::Entity,
    ) -> DatabaseResult<()> {
        self.table.insert(txn, key, &entity)?;
        Ok(())
    }
}

impl TransactionalRemoveItem for ItemStoreImpl {
    type Key = SegKeyBuf;

    fn remove(&self, txn: &mut Transaction, key: Self::Key) -> DatabaseResult<()> {
        self.table.remove(txn, key)?;
        Ok(())
    }
}

impl PutItem for ItemStoreImpl {
    type Key = SegKeyBuf;
    type Entity = AnyValue;

    fn put(&self, key: Self::Key, entity: Self::Entity) -> DatabaseResult<()> {
        let mut txn = self.client.begin_write()?;
        self.table.insert(&mut txn, key, &entity)?;
        txn.commit()?;
        Ok(())
    }
}

impl RemoveItem for ItemStoreImpl {
    type Key = SegKeyBuf;

    fn remove(&self, key: Self::Key) -> DatabaseResult<()> {
        let mut txn = self.client.begin_write()?;
        self.table.remove(&mut txn, key)?;
        txn.commit()?;
        Ok(())
    }
}

impl TransactionalGetItem for ItemStoreImpl {
    type Key = SegKeyBuf;
    type Entity = AnyValue;

    fn get_item(&self, txn: &mut Transaction, key: Self::Key) -> DatabaseResult<Self::Entity> {
        self.table.read(txn, key)
    }
}

impl GetItem for ItemStoreImpl {
    type Key = SegKeyBuf;
    type Entity = AnyValue;

    fn get_item(&self, key: Self::Key) -> DatabaseResult<Self::Entity> {
        let mut txn = self.client.begin_read()?;
        let result = self.table.read(&mut txn, key)?;
        txn.commit()?;

        Ok(result)
    }
}

impl TransactionalListByPrefix for ItemStoreImpl {
    type Key = SegKeyBuf;
    type Entity = AnyValue;

    fn list_by_prefix(
        &self,
        txn: &mut Transaction,
        prefix: &str,
    ) -> DatabaseResult<Vec<(Self::Key, Self::Entity)>> {
        self.table.scan_by_prefix(txn, prefix)
    }
}

impl ListByPrefix for ItemStoreImpl {
    type Key = SegKeyBuf;
    type Entity = AnyValue;

    fn list_by_prefix(&self, prefix: &str) -> DatabaseResult<Vec<(Self::Key, Self::Entity)>> {
        let mut txn = self.client.begin_read()?;
        let result = self.table.scan_by_prefix(&mut txn, prefix)?;
        txn.commit()?;

        Ok(result)
    }
}

impl TransactionalRekeyItem for ItemStoreImpl {
    type Key = SegKeyBuf;
    type Entity = AnyValue;

    fn rekey(
        &self,
        txn: &mut Transaction,
        old_key: Self::Key,
        new_key: Self::Key,
    ) -> DatabaseResult<()> {
        self.table.rekey(txn, old_key, new_key)
    }
}

impl RekeyItem for ItemStoreImpl {
    type Key = SegKeyBuf;
    type Entity = AnyValue;

    fn rekey(&self, old_key: Self::Key, new_key: Self::Key) -> DatabaseResult<()> {
        let mut txn = self.client.begin_write()?;
        self.table.rekey(&mut txn, old_key, new_key)?;
        txn.commit()?;
        Ok(())
    }
}
