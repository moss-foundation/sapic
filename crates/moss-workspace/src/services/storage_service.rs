use anyhow::{Context as _, Result};
use moss_applib::ServiceMarker;
use moss_common::NanoId;
use moss_db::{DatabaseResult, Transaction, primitives::AnyValue};
use moss_storage::{
    WorkspaceStorage,
    primitives::segkey::SegKeyBuf,
    storage::operations::{
        GetItem, ListByPrefix, TransactionalPutItem, TransactionalRemoveByPrefix,
    },
    workspace_storage::WorkspaceStorageImpl,
};
use serde::{Serialize, de::DeserializeOwned};
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    path::Path,
    sync::Arc,
};

use crate::{
    models::primitives::{ActivitybarPosition, SidebarPosition},
    storage::{
        entities::state_store::{EditorGridStateEntity, EditorPanelStateEntity},
        segments::{self, SEGKEY_COLLECTION},
    },
};

pub struct StorageService {
    storage: Arc<dyn WorkspaceStorage>,
}

impl ServiceMarker for StorageService {}

impl StorageService {
    pub fn new(abs_path: &Path) -> Result<Self> {
        let storage = WorkspaceStorageImpl::new(&abs_path)
            .context("Failed to load the workspace state database")?;

        Ok(Self {
            storage: Arc::new(storage),
        })
    }

    pub(crate) fn begin_write(&self) -> Result<Transaction> {
        Ok(self.storage.begin_write()?)
    }

    // Items operations

    pub(crate) fn put_item_order_txn(
        &self,
        txn: &mut Transaction,
        id: &str,
        order: usize,
    ) -> Result<()> {
        let store = self.storage.item_store();

        let segkey = SEGKEY_COLLECTION.join(id.to_string()).join("order");
        TransactionalPutItem::put(store.as_ref(), txn, segkey, AnyValue::serialize(&order)?)?;

        Ok(())
    }

    pub(crate) fn put_expanded_items_txn<T: Serialize>(
        &self,
        txn: &mut Transaction,
        expanded_entries: &HashSet<T>,
    ) -> Result<()> {
        let store = self.storage.item_store();
        TransactionalPutItem::put(
            store.as_ref(),
            txn,
            segments::SEGKEY_EXPANDED_ITEMS.to_segkey_buf(),
            AnyValue::serialize(&expanded_entries)?,
        )?;

        Ok(())
    }

    pub(crate) fn get_expanded_items<T: DeserializeOwned>(&self) -> Result<HashSet<T>>
    where
        T: Eq + Hash,
    {
        let store = self.storage.item_store();
        let segkey = segments::SEGKEY_EXPANDED_ITEMS.to_segkey_buf();
        let value = GetItem::get(store.as_ref(), segkey)?;
        Ok(AnyValue::deserialize::<HashSet<T>>(&value)?)
    }

    pub(crate) fn list_items_metadata(
        &self,
        segkey_prefix: SegKeyBuf,
    ) -> DatabaseResult<HashMap<SegKeyBuf, AnyValue>> {
        let data = ListByPrefix::list_by_prefix(
            self.storage.item_store().as_ref(),
            segkey_prefix.to_string().as_str(),
        )?;

        Ok(data.into_iter().collect())
    }

    pub(crate) fn remove_item_metadata_txn(
        &self,
        txn: &mut Transaction,
        segkey_prefix: SegKeyBuf,
    ) -> DatabaseResult<()> {
        TransactionalRemoveByPrefix::remove_by_prefix(
            self.storage.item_store().as_ref(),
            txn,
            segkey_prefix.to_string().as_str(),
        )?;

        Ok(())
    }

    // Layout operations

    pub(crate) fn get_layout_cache(&self) -> Result<HashMap<SegKeyBuf, AnyValue>> {
        let store = self.storage.item_store();
        let segkey = segments::SEGKEY_LAYOUT.to_segkey_buf();
        let value = ListByPrefix::list_by_prefix(store.as_ref(), segkey.to_string().as_str())?;
        Ok(value.into_iter().collect())
    }

    pub(crate) fn put_sidebar_layout(
        &self,
        position: SidebarPosition,
        size: usize,
        visible: bool,
    ) -> Result<()> {
        let store = self.storage.item_store();
        let mut txn = self.begin_write()?;

        TransactionalPutItem::put(
            store.as_ref(),
            &mut txn,
            segments::SEGKEY_LAYOUT_SIDEBAR.join("position"),
            AnyValue::serialize(&position)?,
        )?;

        TransactionalPutItem::put(
            store.as_ref(),
            &mut txn,
            segments::SEGKEY_LAYOUT_SIDEBAR.join("size"),
            AnyValue::serialize(&size)?,
        )?;

        TransactionalPutItem::put(
            store.as_ref(),
            &mut txn,
            segments::SEGKEY_LAYOUT_SIDEBAR.join("visible"),
            AnyValue::serialize(&visible)?,
        )?;

        Ok(txn.commit()?)
    }

    pub(crate) fn put_panel_layout(&self, size: usize, visible: bool) -> Result<()> {
        let store = self.storage.item_store();
        let mut txn = self.begin_write()?;

        TransactionalPutItem::put(
            store.as_ref(),
            &mut txn,
            segments::SEGKEY_LAYOUT_PANEL.join("size"),
            AnyValue::serialize(&size)?,
        )?;

        TransactionalPutItem::put(
            store.as_ref(),
            &mut txn,
            segments::SEGKEY_LAYOUT_PANEL.join("visible"),
            AnyValue::serialize(&visible)?,
        )?;

        Ok(txn.commit()?)
    }

    pub(crate) fn put_activitybar_layout(
        &self,
        last_active_container_id: Option<String>,
        position: ActivitybarPosition,
    ) -> Result<()> {
        let store = self.storage.item_store();
        let mut txn = self.begin_write()?;

        if let Some(last_active_container_id) = last_active_container_id {
            TransactionalPutItem::put(
                store.as_ref(),
                &mut txn,
                segments::SEGKEY_LAYOUT_ACTIVITYBAR.join("lastActiveContainerId"),
                AnyValue::serialize(&last_active_container_id)?,
            )?
        }

        TransactionalPutItem::put(
            store.as_ref(),
            &mut txn,
            segments::SEGKEY_LAYOUT_ACTIVITYBAR.join("position"),
            AnyValue::serialize(&position)?,
        )?;

        Ok(txn.commit()?)
    }

    pub(crate) fn put_editor_layout(
        &self,
        grid: EditorGridStateEntity,
        panels: HashMap<String, EditorPanelStateEntity>,
        active_group: Option<String>,
    ) -> Result<()> {
        let store = self.storage.item_store();
        let mut txn = self.begin_write()?;

        TransactionalPutItem::put(
            store.as_ref(),
            &mut txn,
            segments::SEGKEY_LAYOUT_EDITOR.join("grid"),
            AnyValue::serialize(&grid)?,
        )?;

        TransactionalPutItem::put(
            store.as_ref(),
            &mut txn,
            segments::SEGKEY_LAYOUT_EDITOR.join("panels"),
            AnyValue::serialize(&panels)?,
        )?;

        if let Some(active_group) = active_group {
            TransactionalPutItem::put(
                store.as_ref(),
                &mut txn,
                segments::SEGKEY_LAYOUT_EDITOR.join("activeGroup"),
                AnyValue::serialize(&active_group)?,
            )?
        }

        Ok(txn.commit()?)
    }

    // HACK: This is a hack to get the storage service for testing purposes.
    // As soon as we switch to getting services by trait instead of by type,
    // we'll be able to move this method into the test service, TestStorageService.
    pub fn __storage(&self) -> &Arc<dyn WorkspaceStorage> {
        &self.storage
    }
}
