use anyhow::{Context as _, Result};
use moss_applib::ServiceMarker;
use moss_db::{Transaction, primitives::AnyValue};
use moss_storage::{
    CollectionStorage,
    collection_storage::CollectionStorageImpl,
    primitives::segkey::SegKeyBuf,
    storage::operations::{GetItem, ListByPrefix, TransactionalPutItem},
};
use serde::{Serialize, de::DeserializeOwned};
use std::{hash::Hash, path::Path, sync::Arc};
use uuid::Uuid;

use crate::storage::segments;

pub struct StorageService {
    storage: Arc<dyn CollectionStorage>,
}

impl ServiceMarker for StorageService {}

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

    pub(crate) fn begin_write(&self) -> Result<Transaction> {
        Ok(self.storage.begin_write()?)
    }

    #[allow(dead_code)]
    pub(crate) fn begin_read(&self) -> Result<Transaction> {
        Ok(self.storage.begin_read()?)
    }

    pub(crate) fn put_entry_order_txn(
        &self,
        txn: &mut Transaction,
        id: Uuid,
        order: usize,
    ) -> Result<()> {
        let store = self.storage.resource_store();

        let segkey = segments::segkey_entry_order(&id.to_string());
        TransactionalPutItem::put(store.as_ref(), txn, segkey, AnyValue::from(order))?;

        Ok(())
    }

    pub(crate) fn get_all_entry_keys(&self) -> Result<impl Iterator<Item = (SegKeyBuf, AnyValue)>> {
        let store = self.storage.resource_store();
        let value = ListByPrefix::list_by_prefix(
            store.as_ref(),
            &segments::SEGKEY_RESOURCE_ENTRY.to_segkey_buf().to_string(),
        )?;

        Ok(value.into_iter())
    }

    pub(crate) fn put_expanded_entries<T: Serialize>(
        &self,
        expanded_entries: Vec<T>,
    ) -> Result<()> {
        let mut txn = self.begin_write()?;
        self.put_expanded_entries_txn(&mut txn, expanded_entries)?;
        txn.commit()?;

        Ok(())
    }

    pub(crate) fn put_expanded_entries_txn<T: Serialize>(
        &self,
        txn: &mut Transaction,
        expanded_entries: Vec<T>,
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

    pub(crate) fn get_expanded_entries<T: DeserializeOwned>(&self) -> Result<Vec<T>>
    where
        T: Eq + Hash,
    {
        let store = self.storage.resource_store();
        let segkey = segments::SEGKEY_EXPANDED_ENTRIES.to_segkey_buf();
        let value = GetItem::get(store.as_ref(), segkey)?;
        Ok(AnyValue::deserialize::<Vec<T>>(&value)?)
    }
}
