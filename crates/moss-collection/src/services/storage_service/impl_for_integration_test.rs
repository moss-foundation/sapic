use async_trait::async_trait;
use moss_applib::{AppRuntime, ServiceMarker};
use moss_db::{Transaction, primitives::AnyValue};
use moss_storage::{CollectionStorage, primitives::segkey::SegKeyBuf};
use std::{collections::HashMap, sync::Arc};

use crate::{
    models::primitives::EntryId,
    services::{AnyStorageService, StorageService},
};

pub struct StorageServiceForIntegrationTest<R: AppRuntime> {
    real: Arc<StorageService<R>>,
}

impl<R: AppRuntime> StorageServiceForIntegrationTest<R> {
    pub fn storage(&self) -> &Arc<dyn CollectionStorage<R::AsyncContext>> {
        &self.real.storage
    }

    pub fn real(&self) -> &Arc<StorageService<R>> {
        &self.real
    }
}

impl<R: AppRuntime> ServiceMarker for StorageServiceForIntegrationTest<R> {}

impl<R: AppRuntime> From<StorageService<R>> for StorageServiceForIntegrationTest<R> {
    fn from(value: StorageService<R>) -> Self {
        Self {
            real: Arc::new(value),
        }
    }
}

#[async_trait]
impl<R: AppRuntime> AnyStorageService<R> for StorageServiceForIntegrationTest<R> {
    async fn begin_write(&self, ctx: &R::AsyncContext) -> anyhow::Result<Transaction> {
        self.real.begin_write(ctx).await
    }

    async fn begin_read(&self, ctx: &R::AsyncContext) -> anyhow::Result<Transaction> {
        self.real.begin_read(ctx).await
    }

    async fn put_entry_order_txn(
        &self,
        ctx: &R::AsyncContext,
        txn: &mut Transaction,
        id: &EntryId,
        order: isize,
    ) -> anyhow::Result<()> {
        self.real.put_entry_order_txn(ctx, txn, id, order).await
    }

    async fn get_all_entry_keys(
        &self,
        ctx: &R::AsyncContext,
    ) -> anyhow::Result<HashMap<SegKeyBuf, AnyValue>> {
        self.real.get_all_entry_keys(ctx).await
    }

    async fn put_expanded_entries(
        &self,
        ctx: &R::AsyncContext,
        expanded_entries: Vec<EntryId>,
    ) -> anyhow::Result<()> {
        self.real.put_expanded_entries(ctx, expanded_entries).await
    }

    async fn put_expanded_entries_txn(
        &self,
        ctx: &R::AsyncContext,
        txn: &mut Transaction,
        expanded_entries: Vec<EntryId>,
    ) -> anyhow::Result<()> {
        self.real
            .put_expanded_entries_txn(ctx, txn, expanded_entries)
            .await
    }

    async fn get_expanded_entries(&self, ctx: &R::AsyncContext) -> anyhow::Result<Vec<EntryId>> {
        self.real.get_expanded_entries(ctx).await
    }
}
