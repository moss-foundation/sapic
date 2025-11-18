use moss_db::{DatabaseResult, Transaction};
use sapic_core::context::AnyAsyncContext;

use async_trait::async_trait;

#[async_trait]
pub trait TransactionalGetItem<Context: AnyAsyncContext>: Send + Sync {
    type Key;
    type Entity;

    async fn get(
        &self,
        ctx: &Context,
        txn: &Transaction,
        key: Self::Key,
    ) -> DatabaseResult<Self::Entity>;
}

#[async_trait]
pub trait TransactionalListByPrefix<Context: AnyAsyncContext>: Send + Sync {
    type Key;
    type Entity;

    async fn list_by_prefix(
        &self,
        ctx: &Context,
        txn: &Transaction,
        prefix: &str,
    ) -> DatabaseResult<Vec<(Self::Key, Self::Entity)>>;
}

#[async_trait]
pub trait GetItem<Context: AnyAsyncContext>: Send + Sync {
    type Key;
    type Entity;

    async fn get(&self, ctx: &Context, key: Self::Key) -> DatabaseResult<Self::Entity>;
}

#[async_trait]
pub trait ListByPrefix<Context: AnyAsyncContext>: Send + Sync {
    type Key;
    type Entity;

    async fn list_by_prefix(
        &self,
        ctx: &Context,
        prefix: &str,
    ) -> DatabaseResult<Vec<(Self::Key, Self::Entity)>>;
}

#[async_trait]
pub trait RemoveByPrefix<Context: AnyAsyncContext>: Send + Sync {
    type Key;
    type Entity;

    async fn remove_by_prefix(
        &self,
        ctx: &Context,
        prefix: &str,
    ) -> DatabaseResult<Vec<(Self::Key, Self::Entity)>>;
}

#[async_trait]
pub trait TransactionalRemoveByPrefix<Context: AnyAsyncContext>: Send + Sync {
    type Key;
    type Entity;

    async fn remove_by_prefix(
        &self,
        ctx: &Context,
        txn: &mut Transaction,
        prefix: &str,
    ) -> DatabaseResult<Vec<(Self::Key, Self::Entity)>>;
}

#[async_trait]
pub trait TransactionalPutItem<Context: AnyAsyncContext>: Send + Sync {
    type Key;
    type Entity;

    fn put(
        &self,
        txn: &mut Transaction,
        key: Self::Key,
        entity: Self::Entity,
    ) -> DatabaseResult<()>;

    async fn put_with_context(
        &self,
        ctx: &Context,
        txn: &mut Transaction,
        key: Self::Key,
        entity: Self::Entity,
    ) -> DatabaseResult<()>;
}

#[async_trait]
pub trait TransactionalRemoveItem<Context: AnyAsyncContext>: Send + Sync {
    type Key;
    type Entity;

    async fn remove(
        &self,
        ctx: &Context,
        txn: &mut Transaction,
        key: Self::Key,
    ) -> DatabaseResult<Self::Entity>;
}

#[async_trait]
pub trait PutItem<Context: AnyAsyncContext>: Send + Sync {
    type Key;
    type Entity;

    async fn put(&self, ctx: &Context, key: Self::Key, entity: Self::Entity) -> DatabaseResult<()>;
}

#[async_trait]
pub trait RemoveItem<Context: AnyAsyncContext>: Send + Sync {
    type Key;
    type Entity;

    async fn remove(&self, ctx: &Context, key: Self::Key) -> DatabaseResult<Self::Entity>;
}

#[async_trait]
pub trait TransactionalRekeyItem<Context: AnyAsyncContext>: Send + Sync {
    type Key;
    type Entity;

    async fn rekey(
        &self,
        ctx: &Context,
        txn: &mut Transaction,
        old_key: Self::Key,
        new_key: Self::Key,
    ) -> DatabaseResult<()>;
}

#[async_trait]
pub trait RekeyItem<Context: AnyAsyncContext>: Send + Sync {
    type Key;
    type Entity;

    async fn rekey(
        &self,
        ctx: &Context,
        old_key: Self::Key,
        new_key: Self::Key,
    ) -> DatabaseResult<()>;
}

#[async_trait]
pub trait Truncate<Context: AnyAsyncContext>: Send + Sync {
    async fn truncate(&self, ctx: &Context) -> DatabaseResult<()>;
}

#[async_trait]
pub trait TransactionalTruncate<Context: AnyAsyncContext>: Send + Sync {
    async fn truncate(&self, ctx: &Context, txn: &mut Transaction) -> DatabaseResult<()>;
}

#[async_trait]
pub trait Scan<Context: AnyAsyncContext>: Send + Sync {
    type Key;
    type Entity;

    async fn scan(&self, ctx: &Context) -> DatabaseResult<Vec<(Self::Key, Self::Entity)>>;
}

#[async_trait]
pub trait TransactionalScan<Context: AnyAsyncContext>: Send + Sync {
    type Key;
    type Entity;

    async fn scan(
        &self,
        ctx: &Context,
        txn: &Transaction,
    ) -> DatabaseResult<Vec<(Self::Key, Self::Entity)>>;
}
