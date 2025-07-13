use async_trait::async_trait;
use moss_applib::context::AnyAsyncContext;
use moss_db::{DatabaseClient, DatabaseResult, ReDbClient, Transaction, primitives::AnyValue};
use std::sync::Arc;

use crate::{
    global_storage::stores::GlobalItemStore,
    primitives::segkey::SegKeyBuf,
    storage::{SegBinTable, operations::*},
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

#[async_trait]
impl<Context> ListByPrefix<Context> for GlobalItemStoreImpl
where
    Context: AnyAsyncContext,
{
    type Key = SegKeyBuf;
    type Entity = AnyValue;

    async fn list_by_prefix(
        &self,
        ctx: &Context,
        prefix: &str,
    ) -> DatabaseResult<Vec<(Self::Key, Self::Entity)>> {
        let read_txn = self.client.begin_read(ctx).await?;
        self.table.scan_by_prefix(ctx, &read_txn, prefix).await
    }
}

#[async_trait]
impl<Context> TransactionalListByPrefix<Context> for GlobalItemStoreImpl
where
    Context: AnyAsyncContext,
{
    type Key = SegKeyBuf;
    type Entity = AnyValue;

    async fn list_by_prefix(
        &self,
        ctx: &Context,
        txn: &Transaction,
        prefix: &str,
    ) -> DatabaseResult<Vec<(Self::Key, Self::Entity)>> {
        self.table.scan_by_prefix(ctx, txn, prefix).await
    }
}

#[async_trait]
impl<Context> PutItem<Context> for GlobalItemStoreImpl
where
    Context: AnyAsyncContext,
{
    type Key = SegKeyBuf;
    type Entity = AnyValue;

    async fn put(&self, ctx: &Context, key: Self::Key, entity: Self::Entity) -> DatabaseResult<()> {
        let mut txn = self.client.begin_write(ctx).await?;
        self.table.insert(ctx, &mut txn, key, &entity).await?;
        txn.commit()
    }
}

#[async_trait]
impl<Context> TransactionalPutItem<Context> for GlobalItemStoreImpl
where
    Context: AnyAsyncContext,
{
    type Key = SegKeyBuf;
    type Entity = AnyValue;

    async fn put(
        &self,
        ctx: &Context,
        txn: &mut Transaction,
        key: Self::Key,
        entity: Self::Entity,
    ) -> DatabaseResult<()> {
        self.table.insert(ctx, txn, key, &entity).await
    }
}

#[async_trait]
impl<Context> RemoveByPrefix<Context> for GlobalItemStoreImpl
where
    Context: AnyAsyncContext,
{
    type Key = SegKeyBuf;
    type Entity = AnyValue;

    async fn remove_by_prefix(
        &self,
        ctx: &Context,
        prefix: &str,
    ) -> DatabaseResult<Vec<(Self::Key, Self::Entity)>> {
        let mut write_txn = self.client.begin_write(ctx).await?;
        self.table
            .remove_by_prefix(ctx, &mut write_txn, prefix)
            .await
    }
}

#[async_trait]
impl<Context> TransactionalRemoveByPrefix<Context> for GlobalItemStoreImpl
where
    Context: AnyAsyncContext,
{
    type Key = SegKeyBuf;
    type Entity = AnyValue;

    async fn remove_by_prefix(
        &self,
        ctx: &Context,
        txn: &mut Transaction,
        prefix: &str,
    ) -> DatabaseResult<Vec<(Self::Key, Self::Entity)>> {
        self.table.remove_by_prefix(ctx, txn, prefix).await
    }
}

#[async_trait]
impl<Context> RemoveItem<Context> for GlobalItemStoreImpl
where
    Context: AnyAsyncContext,
{
    type Key = SegKeyBuf;
    type Entity = AnyValue;

    async fn remove(&self, ctx: &Context, key: Self::Key) -> DatabaseResult<Self::Entity> {
        let mut txn = self.client.begin_write(ctx).await?;
        let value = self.table.remove(ctx, &mut txn, key).await?;
        txn.commit()?;
        Ok(value)
    }
}

#[async_trait]
impl<Context> TransactionalRemoveItem<Context> for GlobalItemStoreImpl
where
    Context: AnyAsyncContext,
{
    type Key = SegKeyBuf;
    type Entity = AnyValue;

    async fn remove(
        &self,
        ctx: &Context,
        txn: &mut Transaction,
        key: Self::Key,
    ) -> DatabaseResult<Self::Entity> {
        self.table.remove(ctx, txn, key).await
    }
}

#[async_trait]
impl<Context> GetItem<Context> for GlobalItemStoreImpl
where
    Context: AnyAsyncContext,
{
    type Key = SegKeyBuf;
    type Entity = AnyValue;

    async fn get(&self, ctx: &Context, key: Self::Key) -> DatabaseResult<Self::Entity> {
        let read_txn = self.client.begin_read(ctx).await?;
        self.table.read(ctx, &read_txn, key).await
    }
}

#[async_trait]
impl<Context> TransactionalGetItem<Context> for GlobalItemStoreImpl
where
    Context: AnyAsyncContext,
{
    type Key = SegKeyBuf;
    type Entity = AnyValue;

    async fn get(
        &self,
        ctx: &Context,
        txn: &Transaction,
        key: Self::Key,
    ) -> DatabaseResult<Self::Entity> {
        self.table.read(ctx, txn, key).await
    }
}

impl<Context> GlobalItemStore<Context> for GlobalItemStoreImpl where Context: AnyAsyncContext {}
