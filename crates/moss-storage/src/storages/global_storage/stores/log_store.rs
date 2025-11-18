use async_trait::async_trait;
use moss_db::{
    DatabaseClientWithContext, DatabaseResult, ReDbClient, Transaction, primitives::AnyValue,
};
use sapic_core::context::AnyAsyncContext;
use std::sync::Arc;

use crate::{
    global_storage::stores::GlobalLogStore,
    primitives::segkey::SegKeyBuf,
    storage::{
        SegBinTable,
        operations::{
            GetItem, PutItem, RemoveItem, TransactionalGetItem, TransactionalPutItem,
            TransactionalRemoveItem,
        },
    },
};

pub struct GlobalLogStoreImpl {
    client: ReDbClient,
    table: Arc<SegBinTable>,
}

impl GlobalLogStoreImpl {
    pub fn new(client: ReDbClient, table: Arc<SegBinTable>) -> Self {
        Self { client, table }
    }
}

#[async_trait]
impl<Context> PutItem<Context> for GlobalLogStoreImpl
where
    Context: AnyAsyncContext,
{
    type Key = SegKeyBuf;
    type Entity = AnyValue;

    async fn put(&self, ctx: &Context, key: Self::Key, entity: Self::Entity) -> DatabaseResult<()> {
        let mut txn = self.client.begin_write_with_context(ctx).await?;
        self.table
            .insert_with_context(ctx, &mut txn, key, &entity)
            .await?;
        txn.commit()
    }
}

#[async_trait]
impl<Context> TransactionalPutItem<Context> for GlobalLogStoreImpl
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
impl<Context> RemoveItem<Context> for GlobalLogStoreImpl
where
    Context: AnyAsyncContext,
{
    type Key = SegKeyBuf;
    type Entity = AnyValue;

    async fn remove(&self, ctx: &Context, key: Self::Key) -> DatabaseResult<Self::Entity> {
        let mut txn = self.client.begin_write_with_context(ctx).await?;
        let value = self.table.remove_with_context(ctx, &mut txn, key).await?;
        txn.commit()?;
        Ok(value)
    }
}

#[async_trait]
impl<Context> TransactionalRemoveItem<Context> for GlobalLogStoreImpl
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
impl<Context> GetItem<Context> for GlobalLogStoreImpl
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
impl<Context> TransactionalGetItem<Context> for GlobalLogStoreImpl
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

impl<Context> GlobalLogStore<Context> for GlobalLogStoreImpl where Context: AnyAsyncContext {}
