use crate::{
    models::primitives::EntryId,
    services::{AnyStorageService, StorageService},
};
use moss_applib::ServiceMarker;
use moss_db::{Transaction, primitives::AnyValue};
use moss_storage::{CollectionStorage, primitives::segkey::SegKeyBuf, storage::Storage};
use std::{collections::HashMap, sync::Arc};

pub struct StorageServiceForIntegrationTest {
    real: Arc<StorageService>,
}

impl StorageServiceForIntegrationTest {
    pub fn storage(&self) -> &Arc<dyn CollectionStorage> {
        &self.real.storage
    }

    pub fn real(&self) -> &Arc<StorageService> {
        &self.real
    }
}

impl ServiceMarker for StorageServiceForIntegrationTest {}

impl From<StorageService> for StorageServiceForIntegrationTest {
    fn from(value: StorageService) -> Self {
        Self {
            real: Arc::new(value),
        }
    }
}

impl AnyStorageService for StorageServiceForIntegrationTest {
    fn begin_write(&self) -> anyhow::Result<Transaction> {
        self.real.begin_write()
    }

    fn begin_read(&self) -> anyhow::Result<Transaction> {
        self.real.begin_read()
    }

    fn put_entry_order_txn(
        &self,
        txn: &mut Transaction,
        id: &EntryId,
        order: isize,
    ) -> anyhow::Result<()> {
        self.real.put_entry_order_txn(txn, id, order)
    }

    fn get_all_entry_keys(&self) -> anyhow::Result<HashMap<SegKeyBuf, AnyValue>> {
        self.real.get_all_entry_keys()
    }

    fn put_expanded_entries(&self, expanded_entries: Vec<EntryId>) -> anyhow::Result<()> {
        self.real.put_expanded_entries(expanded_entries)
    }

    fn put_expanded_entries_txn(
        &self,
        txn: &mut Transaction,
        expanded_entries: Vec<EntryId>,
    ) -> anyhow::Result<()> {
        self.real.put_expanded_entries_txn(txn, expanded_entries)
    }

    fn get_expanded_entries(&self) -> anyhow::Result<Vec<EntryId>> {
        self.real.get_expanded_entries()
    }
}
