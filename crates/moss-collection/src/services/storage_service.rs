#[cfg(any(test, feature = "integration-tests"))]
pub mod impl_for_integration_test;

use anyhow::{Context as _, Result};
use async_trait::async_trait;
use moss_applib::ServiceMarker;
use moss_db::{Transaction, primitives::AnyValue};
use moss_storage::{
    CollectionStorage,
    collection_storage::CollectionStorageImpl,
    primitives::segkey::SegKeyBuf,
    storage::operations::{GetItem, ListByPrefix, TransactionalPutItem},
};
use std::{collections::HashMap, path::Path, sync::Arc};

use crate::{models::primitives::EntryId, services::AnyStorageService, storage::segments};

pub struct StorageService {
    storage: Arc<dyn CollectionStorage>,
}

impl ServiceMarker for StorageService {}

#[async_trait]
impl AnyStorageService for StorageService {
    fn begin_write(&self) -> Result<Transaction> {
        Ok(self.storage.begin_write()?)
    }

    #[allow(dead_code)]
    fn begin_read(&self) -> Result<Transaction> {
        Ok(self.storage.begin_read()?)
    }

    fn put_entry_order_txn(&self, txn: &mut Transaction, id: &EntryId, order: isize) -> Result<()> {
        let store = self.storage.resource_store();

        let segkey = segments::segkey_entry_order(&id);
        TransactionalPutItem::put(store.as_ref(), txn, segkey, AnyValue::serialize(&order)?)?;

        Ok(())
    }

    fn get_all_entry_keys(&self) -> Result<HashMap<SegKeyBuf, AnyValue>> {
        let store = self.storage.resource_store();
        let value = ListByPrefix::list_by_prefix(
            store.as_ref(),
            &segments::SEGKEY_RESOURCE_ENTRY.to_segkey_buf().to_string(),
        )?;

        Ok(value.into_iter().collect())
    }

    fn put_expanded_entries(&self, expanded_entries: Vec<EntryId>) -> Result<()> {
        let mut txn = self.begin_write()?;
        self.put_expanded_entries_txn(&mut txn, expanded_entries)?;
        txn.commit()?;

        Ok(())
    }

    fn put_expanded_entries_txn(
        &self,
        txn: &mut Transaction,
        expanded_entries: Vec<EntryId>,
    ) -> Result<()> {
        let store = self.storage.resource_store();
        TransactionalPutItem::put(
            store.as_ref(),
            txn,
            segments::SEGKEY_EXPANDED_ENTRIES.to_segkey_buf(),
            AnyValue::serialize(&expanded_entries)?,
        )?;

        Ok(())
    }

    fn get_expanded_entries(&self) -> Result<Vec<EntryId>> {
        let store = self.storage.resource_store();
        let segkey = segments::SEGKEY_EXPANDED_ENTRIES.to_segkey_buf();
        let value = GetItem::get(store.as_ref(), segkey)?;
        Ok(AnyValue::deserialize::<Vec<EntryId>>(&value)?)
    }
}

impl StorageService {
    pub fn new(abs_path: &Path) -> Result<Self> {
        debug_assert!(abs_path.is_absolute());

        let storage = CollectionStorageImpl::new(&abs_path).context(format!(
            "Failed to open the collection {} state database",
            abs_path.display()
        ))?;

        Ok(Self {
            storage: Arc::new(storage),
        })
    }
}
