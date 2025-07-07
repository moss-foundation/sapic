use crate::storage::segments::{SEGKEY_LAST_ACTIVE_WORKSPACE, segkey_last_opened_at};
use anyhow::Result;
use moss_applib::ServiceMarker;
use moss_common::NanoId;
use moss_db::{DatabaseResult, Transaction, primitives::AnyValue};
use moss_storage::{
    GlobalStorage,
    global_storage::GlobalStorageImpl,
    primitives::segkey::SegKeyBuf,
    storage::operations::{
        GetItem, ListByPrefix, RemoveByPrefix, RemoveItem, TransactionalPutItem,
    },
};
use std::{collections::HashMap, path::Path, sync::Arc};

pub struct StorageService {
    storage: Arc<dyn GlobalStorage>,
}

impl ServiceMarker for StorageService {}

impl StorageService {
    // HACK: This is a temporary hack to allow access to the storage to be used in the log service.
    // This should be removed once the log service is refactored to use the storage service.
    pub fn __storage(&self) -> Arc<dyn GlobalStorage> {
        self.storage.clone()
    }
}

impl StorageService {
    pub fn new(abs_path: &Path) -> Result<Self> {
        let storage =
            Arc::new(GlobalStorageImpl::new(abs_path).expect("Failed to create global storage"));

        Ok(Self { storage })
    }

    pub(crate) fn begin_write(&self) -> DatabaseResult<Transaction> {
        Ok(self.storage.begin_write()?)
    }

    pub(crate) fn remove_last_active_workspace(&self) -> DatabaseResult<()> {
        let store = self.storage.item_store();

        RemoveItem::remove(store.as_ref(), SEGKEY_LAST_ACTIVE_WORKSPACE.to_segkey_buf())?;

        Ok(())
    }

    pub(crate) fn get_last_active_workspace(&self) -> DatabaseResult<String> {
        let store = self.storage.item_store();
        let data = GetItem::get(store.as_ref(), SEGKEY_LAST_ACTIVE_WORKSPACE.to_segkey_buf())?;
        Ok(data.deserialize::<String>()?)
    }

    pub(crate) fn put_last_active_workspace_txn(
        &self,
        txn: &mut Transaction,
        id: &NanoId,
    ) -> DatabaseResult<()> {
        let store = self.storage.item_store();

        TransactionalPutItem::put(
            store.as_ref(),
            txn,
            SEGKEY_LAST_ACTIVE_WORKSPACE.to_segkey_buf(),
            AnyValue::serialize(&id)?,
        )?;

        Ok(())
    }

    pub(crate) fn put_last_opened_at_txn(
        &self,
        txn: &mut Transaction,
        id: &NanoId,
        timestamp: i64,
    ) -> DatabaseResult<()> {
        let store = self.storage.item_store();

        TransactionalPutItem::put(
            store.as_ref(),
            txn,
            segkey_last_opened_at(id),
            AnyValue::serialize(&timestamp)?,
        )?;

        Ok(())
    }

    pub(crate) fn list_all_by_prefix(
        &self,
        prefix: &str,
    ) -> DatabaseResult<HashMap<SegKeyBuf, AnyValue>> {
        let store = self.storage.item_store();

        let data = ListByPrefix::list_by_prefix(store.as_ref(), prefix)?;

        Ok(data.into_iter().collect())
    }

    pub(crate) fn remove_all_by_prefix(&self, prefix: &str) -> DatabaseResult<()> {
        let store = self.storage.item_store();

        RemoveByPrefix::remove_by_prefix(store.as_ref(), prefix)?;

        Ok(())
    }
}
