use crate::{
    common::VariableStore,
    primitives::segkey::SegKeyBuf,
    storage::{
        SegBinTable,
        operations::{
            GetItem, PutItem, RemoveItem, TransactionalGetItem, TransactionalPutItem,
            TransactionalRemoveItem,
        },
    },
};
use async_trait::async_trait;
use moss_db::{
    DatabaseClientWithContext, DatabaseResult, ReDbClient, Transaction, primitives::AnyValue,
};
use sapic_core::context::AnyAsyncContext;
use std::sync::Arc;

pub struct VariableStoreImpl {
    #[allow(unused)]
    client: ReDbClient,
    #[allow(dead_code)]
    table: Arc<SegBinTable>,
}

impl VariableStoreImpl {
    pub fn new(client: ReDbClient, table: Arc<SegBinTable>) -> Self {
        Self { client, table }
    }
}

#[async_trait]
impl<Context> PutItem<Context> for VariableStoreImpl
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
        write_txn.commit()?;

        Ok(())
    }
}

#[async_trait]
impl<Context> TransactionalPutItem<Context> for VariableStoreImpl
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
impl<Context> GetItem<Context> for VariableStoreImpl
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
impl<Context> TransactionalGetItem<Context> for VariableStoreImpl
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
impl<Context> RemoveItem<Context> for VariableStoreImpl
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
impl<Context> TransactionalRemoveItem<Context> for VariableStoreImpl
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
impl<Context> VariableStore<Context> for VariableStoreImpl
where
    Context: AnyAsyncContext,
{
    async fn begin_write(&self, ctx: &Context) -> joinerror::Result<Transaction> {
        Ok(self.client.begin_write_with_context(ctx).await?)
    }
}
