use async_trait::async_trait;
use moss_db::{
    DatabaseClientWithContext, DatabaseResult, ReDbClient, Transaction, primitives::AnyValue,
};
use sapic_core::context::AnyAsyncContext;
use std::sync::Arc;

use crate::{
    primitives::segkey::SegKeyBuf,
    storage::{SegBinTable, TransactionalWithContext, operations::*},
    workspace_storage::stores::WorkspaceItemStore,
};

pub struct WorkspaceItemStoreImpl {
    client: ReDbClient,
    table: Arc<SegBinTable>,
}

impl WorkspaceItemStoreImpl {
    pub fn new(client: ReDbClient, table: Arc<SegBinTable>) -> Self {
        Self { client, table }
    }
}

#[async_trait]
impl<Context> TransactionalWithContext<Context> for WorkspaceItemStoreImpl
where
    Context: AnyAsyncContext,
{
    async fn begin_write_with_context(&self, ctx: &Context) -> DatabaseResult<Transaction> {
        self.client.begin_write_with_context(ctx).await
    }

    async fn begin_read_with_context(&self, ctx: &Context) -> DatabaseResult<Transaction> {
        self.client.begin_read_with_context(ctx).await
    }
}

#[async_trait]
impl<Context> ListByPrefix<Context> for WorkspaceItemStoreImpl
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
impl<Context> TransactionalListByPrefix<Context> for WorkspaceItemStoreImpl
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
impl<Context> RemoveByPrefix<Context> for WorkspaceItemStoreImpl
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
        let mut write_txn = self.client.begin_write_with_context(ctx).await?;
        let result = self
            .table
            .remove_by_prefix_with_context(ctx, &mut write_txn, prefix)
            .await?;
        write_txn.commit()?;
        Ok(result)
    }
}

#[async_trait]
impl<Context> TransactionalRemoveByPrefix<Context> for WorkspaceItemStoreImpl
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
        self.table
            .remove_by_prefix_with_context(ctx, txn, prefix)
            .await
    }
}

#[async_trait]
impl<Context> PutItem<Context> for WorkspaceItemStoreImpl
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
impl<Context> TransactionalPutItem<Context> for WorkspaceItemStoreImpl
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
impl<Context> GetItem<Context> for WorkspaceItemStoreImpl
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
impl<Context> TransactionalGetItem<Context> for WorkspaceItemStoreImpl
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
impl<Context> RemoveItem<Context> for WorkspaceItemStoreImpl
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
impl<Context> TransactionalRemoveItem<Context> for WorkspaceItemStoreImpl
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

impl<Context> WorkspaceItemStore<Context> for WorkspaceItemStoreImpl where Context: AnyAsyncContext {}
