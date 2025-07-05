use moss_db::{DatabaseResult, Transaction};

pub trait TransactionalGetItem: Send + Sync {
    type Key;
    type Entity;

    fn get(&self, txn: &Transaction, key: Self::Key) -> DatabaseResult<Self::Entity>;
}

pub trait TransactionalListByPrefix: Send + Sync {
    type Key;
    type Entity;

    fn list_by_prefix(
        &self,
        txn: &Transaction,
        prefix: &str,
    ) -> DatabaseResult<Vec<(Self::Key, Self::Entity)>>;
}

pub trait GetItem: Send + Sync {
    type Key;
    type Entity;

    fn get(&self, key: Self::Key) -> DatabaseResult<Self::Entity>;
}

pub trait ListByPrefix: Send + Sync {
    type Key;
    type Entity;

    fn list_by_prefix(&self, prefix: &str) -> DatabaseResult<Vec<(Self::Key, Self::Entity)>>;
}

pub trait RemoveByPrefix: Send + Sync {
    type Key;
    type Entity;

    fn remove_by_prefix(&self, prefix: &str) -> DatabaseResult<Vec<(Self::Key, Self::Entity)>>;
}

pub trait TransactionalRemoveByPrefix: Send + Sync {
    type Key;
    type Entity;

    fn remove_by_prefix(
        &self,
        txn: &mut Transaction,
        prefix: &str,
    ) -> DatabaseResult<Vec<(Self::Key, Self::Entity)>>;
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
    type Entity;

    fn remove(&self, txn: &mut Transaction, key: Self::Key) -> DatabaseResult<Self::Entity>;
}

pub trait PutItem: Send + Sync {
    type Key;
    type Entity;

    fn put(&self, key: Self::Key, entity: Self::Entity) -> DatabaseResult<()>;
}

pub trait RemoveItem: Send + Sync {
    type Key;
    type Entity;

    fn remove(&self, key: Self::Key) -> DatabaseResult<Self::Entity>;
}

pub trait TransactionalRekeyItem: Send + Sync {
    type Key;
    type Entity;

    fn rekey(
        &self,
        txn: &mut Transaction,
        old_key: Self::Key,
        new_key: Self::Key,
    ) -> DatabaseResult<()>;
}

pub trait RekeyItem: Send + Sync {
    type Key;
    type Entity;

    fn rekey(&self, old_key: Self::Key, new_key: Self::Key) -> DatabaseResult<()>;
}

pub trait Truncate: Send + Sync {
    fn truncate(&self) -> DatabaseResult<()>;
}

pub trait TransactionalTruncate: Send + Sync {
    fn truncate(&self, txn: &mut Transaction) -> DatabaseResult<()>;
}

pub trait Scan: Send + Sync {
    type Key;
    type Entity;
    fn scan(&self) -> DatabaseResult<Vec<(Self::Key, Self::Entity)>>;
}

pub trait TransactionalScan: Send + Sync {
    type Key;
    type Entity;
    fn scan(&self, txn: &Transaction) -> DatabaseResult<Vec<(Self::Key, Self::Entity)>>;
}
