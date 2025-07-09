use std::sync::Arc;

use moss_applib::ServiceMarker;
use moss_storage::WorkspaceStorage;

use crate::services::{AnyStorageService, storage_service::StorageService};

pub struct TestStorageService {
    real: Arc<StorageService>,
}

impl TestStorageService {
    pub fn storage(&self) -> &Arc<dyn WorkspaceStorage> {
        &self.real.storage
    }

    pub fn real(&self) -> &Arc<StorageService> {
        &self.real
    }
}

impl ServiceMarker for TestStorageService {}

impl From<StorageService> for TestStorageService {
    fn from(real: StorageService) -> Self {
        Self {
            real: Arc::new(real),
        }
    }
}

impl AnyStorageService for TestStorageService {
    fn begin_write(&self) -> anyhow::Result<moss_db::Transaction> {
        self.real.begin_write()
    }

    fn put_item_order_txn(
        &self,
        txn: &mut moss_db::Transaction,
        id: &str,
        order: usize,
    ) -> anyhow::Result<()> {
        self.real.put_item_order_txn(txn, id, order)
    }

    fn put_expanded_items_txn(
        &self,
        txn: &mut moss_db::Transaction,
        expanded_entries: &std::collections::HashSet<crate::models::primitives::CollectionId>,
    ) -> anyhow::Result<()> {
        self.real.put_expanded_items_txn(txn, expanded_entries)
    }

    fn get_expanded_items(
        &self,
    ) -> anyhow::Result<std::collections::HashSet<crate::models::primitives::CollectionId>> {
        self.real.get_expanded_items()
    }

    fn remove_item_metadata_txn(
        &self,
        txn: &mut moss_db::Transaction,
        segkey_prefix: moss_storage::primitives::segkey::SegKeyBuf,
    ) -> moss_db::DatabaseResult<()> {
        self.real.remove_item_metadata_txn(txn, segkey_prefix)
    }

    fn list_items_metadata(
        &self,
        segkey_prefix: moss_storage::primitives::segkey::SegKeyBuf,
    ) -> moss_db::DatabaseResult<
        std::collections::HashMap<
            moss_storage::primitives::segkey::SegKeyBuf,
            moss_db::primitives::AnyValue,
        >,
    > {
        self.real.list_items_metadata(segkey_prefix)
    }

    fn get_layout_cache(
        &self,
    ) -> anyhow::Result<
        std::collections::HashMap<
            moss_storage::primitives::segkey::SegKeyBuf,
            moss_db::primitives::AnyValue,
        >,
    > {
        self.real.get_layout_cache()
    }
}
