use crate::{
    models::primitives::{LogEntryId, WorkspaceId},
    storage::segments::{SEGKEY_LAST_ACTIVE_WORKSPACE, segkey_last_opened_at},
};
use anyhow::Result;
use moss_applib::{AppRuntime, ServiceMarker};
use moss_db::{DatabaseResult, Transaction, primitives::AnyValue};
use moss_storage::{
    GlobalStorage,
    global_storage::GlobalStorageImpl,
    primitives::segkey::{SegKey, SegKeyBuf},
    storage::operations::{
        GetItem, ListByPrefix, RemoveByPrefix, RemoveItem, TransactionalPutItem,
        TransactionalRemoveItem,
    },
};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};

pub struct StorageService<R: AppRuntime> {
    storage: Arc<dyn GlobalStorage<R::AsyncContext>>,
}

impl<R: AppRuntime> ServiceMarker for StorageService<R> {}

impl<R: AppRuntime> StorageService<R> {
    // HACK: This is a temporary hack to allow access to the storage to be used in the log service.
    // This should be removed once the log service is refactored to use the storage service.
    pub fn __storage(&self) -> Arc<dyn GlobalStorage<R::AsyncContext>> {
        self.storage.clone()
    }
}

impl<R: AppRuntime> StorageService<R> {
    pub fn new(abs_path: &Path) -> Result<Self> {
        let storage =
            Arc::new(GlobalStorageImpl::new(abs_path).expect("Failed to create global storage"));

        Ok(Self { storage })
    }

    pub(crate) async fn begin_write_with_context(
        &self,
        ctx: &R::AsyncContext,
    ) -> DatabaseResult<Transaction> {
        Ok(self.storage.begin_write_with_context(ctx).await?)
    }

    pub(crate) fn begin_write(&self) -> DatabaseResult<Transaction> {
        Ok(self.storage.begin_write()?)
    }

    pub(crate) async fn remove_last_active_workspace(
        &self,
        ctx: &R::AsyncContext,
    ) -> DatabaseResult<()> {
        let store = self.storage.item_store();

        RemoveItem::remove(
            store.as_ref(),
            ctx,
            SEGKEY_LAST_ACTIVE_WORKSPACE.to_segkey_buf(),
        )
        .await?;

        Ok(())
    }

    pub(crate) async fn get_last_active_workspace(
        &self,
        ctx: &R::AsyncContext,
    ) -> DatabaseResult<WorkspaceId> {
        let store = self.storage.item_store();
        let data = GetItem::get(
            store.as_ref(),
            ctx,
            SEGKEY_LAST_ACTIVE_WORKSPACE.to_segkey_buf(),
        )
        .await?;
        Ok(data.deserialize::<WorkspaceId>()?)
    }

    pub(crate) async fn put_last_active_workspace_txn(
        &self,
        ctx: &R::AsyncContext,
        txn: &mut Transaction,
        id: &WorkspaceId,
    ) -> DatabaseResult<()> {
        let store = self.storage.item_store();

        TransactionalPutItem::put_with_context(
            store.as_ref(),
            ctx,
            txn,
            SEGKEY_LAST_ACTIVE_WORKSPACE.to_segkey_buf(),
            AnyValue::serialize(&id)?,
        )
        .await?;

        Ok(())
    }

    pub(crate) async fn put_last_opened_at_txn(
        &self,
        ctx: &R::AsyncContext,
        txn: &mut Transaction,
        id: &WorkspaceId,
        timestamp: i64,
    ) -> DatabaseResult<()> {
        let store = self.storage.item_store();
        let segkey = segkey_last_opened_at(id);

        TransactionalPutItem::put_with_context(
            store.as_ref(),
            ctx,
            txn,
            segkey,
            AnyValue::serialize(&timestamp)?,
        )
        .await?;

        Ok(())
    }

    pub(crate) async fn list_all_by_prefix(
        &self,
        ctx: &R::AsyncContext,
        prefix: &str,
    ) -> DatabaseResult<HashMap<SegKeyBuf, AnyValue>> {
        let store = self.storage.item_store();

        let data = ListByPrefix::list_by_prefix(store.as_ref(), ctx, prefix).await?;

        Ok(data.into_iter().collect())
    }

    pub(crate) async fn remove_all_by_prefix(
        &self,
        ctx: &R::AsyncContext,
        prefix: &str,
    ) -> DatabaseResult<()> {
        let store = self.storage.item_store();

        RemoveByPrefix::remove_by_prefix(store.as_ref(), ctx, prefix).await?;

        Ok(())
    }

    pub(crate) async fn get_log_path(
        &self,
        ctx: &R::AsyncContext,
        log_id: &LogEntryId,
    ) -> DatabaseResult<PathBuf> {
        let segkey = SegKey::new(&log_id).to_segkey_buf();
        let store = self.storage.log_store();
        let data = GetItem::get(store.as_ref(), ctx, segkey).await?;
        Ok(data.deserialize::<PathBuf>()?)
    }

    pub(crate) async fn remove_log_path_txn(
        &self,
        ctx: &R::AsyncContext,
        txn: &mut Transaction,
        log_id: &LogEntryId,
    ) -> DatabaseResult<()> {
        let segkey = SegKey::new(&log_id).to_segkey_buf();
        let store = self.storage.log_store();
        TransactionalRemoveItem::remove(store.as_ref(), ctx, txn, segkey).await?;
        Ok(())
    }

    pub(crate) fn put_log_path_txn(
        &self,
        txn: &mut Transaction,
        log_id: &LogEntryId,
        path: PathBuf,
    ) -> DatabaseResult<()> {
        let segkey = SegKey::new(&log_id).to_segkey_buf();
        let store = self.storage.log_store();
        TransactionalPutItem::put(store.as_ref(), txn, segkey, AnyValue::serialize(&path)?)?;
        Ok(())
    }
}
