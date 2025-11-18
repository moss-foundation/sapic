use async_trait::async_trait;
use moss_db::{
    DatabaseClientWithContext, DatabaseResult, ReDbClient, Transaction, primitives::AnyValue,
};
use sapic_core::context::AnyAsyncContext;
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

#[async_trait]
impl<Context> ListByPrefix<Context> for CollectionResourceStoreImpl
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
        let read_txn = self.client.begin_read_with_context(ctx).await?;
        self.table
            .scan_by_prefix_with_context(ctx, &read_txn, prefix)
            .await
    }
}

#[async_trait]
impl<Context> TransactionalListByPrefix<Context> for CollectionResourceStoreImpl
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
        self.table
            .scan_by_prefix_with_context(ctx, txn, prefix)
            .await
    }
}

#[async_trait]
impl<Context> PutItem<Context> for CollectionResourceStoreImpl
where
    Context: AnyAsyncContext,
{
    type Key = SegKeyBuf;
    type Entity = AnyValue;

    async fn put(&self, ctx: &Context, key: Self::Key, entity: Self::Entity) -> DatabaseResult<()> {
        let mut write_txn = self.client.begin_write_with_context(ctx).await?;
        self.table
            .insert_with_context(ctx, &mut write_txn, key, &entity)
            .await?;
        write_txn.commit()
    }
}

#[async_trait]
impl<Context> TransactionalPutItem<Context> for CollectionResourceStoreImpl
where
    Context: AnyAsyncContext,
{
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

    async fn put_with_context(
        &self,
        ctx: &Context,
        txn: &mut Transaction,
        key: Self::Key,
        entity: Self::Entity,
    ) -> DatabaseResult<()> {
        self.table.insert_with_context(ctx, txn, key, &entity).await
    }
}

#[async_trait]
impl<Context> GetItem<Context> for CollectionResourceStoreImpl
where
    Context: AnyAsyncContext,
{
    type Key = SegKeyBuf;
    type Entity = AnyValue;

    async fn get(&self, ctx: &Context, key: Self::Key) -> DatabaseResult<Self::Entity> {
        let read_txn = self.client.begin_read_with_context(ctx).await?;
        self.table.read_with_context(ctx, &read_txn, key).await
    }
}

#[async_trait]
impl<Context> TransactionalGetItem<Context> for CollectionResourceStoreImpl
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
        self.table.read_with_context(ctx, txn, key).await
    }
}

#[async_trait]
impl<Context> RemoveItem<Context> for CollectionResourceStoreImpl
where
    Context: AnyAsyncContext,
{
    type Key = SegKeyBuf;
    type Entity = AnyValue;

    async fn remove(&self, ctx: &Context, key: Self::Key) -> DatabaseResult<Self::Entity> {
        let mut write_txn = self.client.begin_write_with_context(ctx).await?;
        let value = self
            .table
            .remove_with_context(ctx, &mut write_txn, key)
            .await?;
        write_txn.commit()?;
        Ok(value)
    }
}

#[async_trait]
impl<Context> TransactionalRemoveItem<Context> for CollectionResourceStoreImpl
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
        self.table.remove_with_context(ctx, txn, key).await
    }
}

#[async_trait]
impl<Context> RemoveByPrefix<Context> for CollectionResourceStoreImpl
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
        let mut txn = self.client.begin_write_with_context(ctx).await?;
        self.table
            .remove_by_prefix_with_context(ctx, &mut txn, prefix)
            .await
    }
}

impl<Context: AnyAsyncContext> CollectionResourceStore<Context> for CollectionResourceStoreImpl {}
