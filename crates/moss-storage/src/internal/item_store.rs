pub mod store_impl;

use moss_db::{AnyEntity, DatabaseResult, Transaction, bincode_table::BincodeTable};

pub trait TransactionalGetItem: Send + Sync {
    type Key;
    type Entity;

    fn get_item(&self, txn: &mut Transaction, key: Self::Key) -> DatabaseResult<Self::Entity>;
}

pub trait TransactionalListByPrefix: Send + Sync {
    type Key;
    type Entity;

    fn list_by_prefix(
        &self,
        txn: &mut Transaction,
        prefix: &str,
    ) -> DatabaseResult<Vec<(Self::Key, Self::Entity)>>;
}

pub trait GetItem: Send + Sync {
    type Key;
    type Entity;

    fn get_item(&self, key: Self::Key) -> DatabaseResult<Self::Entity>;
}

pub trait ListByPrefix: Send + Sync {
    type Key;
    type Entity;

    fn list_by_prefix(&self, prefix: &str) -> DatabaseResult<Vec<(Self::Key, Self::Entity)>>;
}

pub trait TransactionalPutItem: Send + Sync {
    type Key;
    type Entity;

    fn put(
        &self,
        txn: &mut Transaction,
        key: Self::Key,
        entity: Self::Entity,
    ) -> DatabaseResult<()>;
}

pub trait TransactionalRemoveItem: Send + Sync {
    type Key;

    fn remove(&self, txn: &mut Transaction, key: Self::Key) -> DatabaseResult<()>;
}

pub trait PutItem: Send + Sync {
    type Key;
    type Entity;

    fn put(&self, key: Self::Key, entity: Self::Entity) -> DatabaseResult<()>;
}

pub trait RemoveItem: Send + Sync {
    type Key;

    fn remove(&self, key: Self::Key) -> DatabaseResult<()>;
}

pub(crate) type ItemStoreTable<'a> = BincodeTable<'a, Vec<u8>, AnyEntity>;

pub trait ItemStore<K, E>:
    TransactionalGetItem<Key = K, Entity = E>
    + GetItem<Key = K, Entity = E>
    + TransactionalListByPrefix<Key = K, Entity = E>
    + ListByPrefix<Key = K, Entity = E>
    + TransactionalPutItem<Key = K, Entity = E>
    + PutItem<Key = K, Entity = E>
    + TransactionalRemoveItem<Key = K>
    + RemoveItem<Key = K>
    + Send
    + Sync
{
}
