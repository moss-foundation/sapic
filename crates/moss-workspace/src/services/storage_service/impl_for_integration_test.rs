use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use moss_applib::ServiceMarker;
use moss_db::{DatabaseResult, Transaction, primitives::AnyValue};
use moss_storage::{WorkspaceStorage, primitives::segkey::SegKeyBuf};

use crate::{
    models::primitives::{ActivitybarPosition, CollectionId, SidebarPosition},
    services::{AnyStorageService, storage_service::StorageService},
    storage::entities::state_store::{EditorGridStateEntity, EditorPanelStateEntity},
};

// pub struct StorageServiceForIntegrationTest {
//     real: Arc<StorageService>,
// }

// impl StorageServiceForIntegrationTest {
//     pub fn storage(&self) -> &Arc<dyn WorkspaceStorage> {
//         &self.real.storage
//     }

//     pub fn real(&self) -> &Arc<StorageService> {
//         &self.real
//     }
// }

// impl ServiceMarker for StorageServiceForIntegrationTest {}

// impl From<StorageService> for StorageServiceForIntegrationTest {
//     fn from(real: StorageService) -> Self {
//         Self {
//             real: Arc::new(real),
//         }
//     }
// }

// impl AnyStorageService for StorageServiceForIntegrationTest {
//     fn begin_write(&self) -> anyhow::Result<Transaction> {
//         self.real.begin_write()
//     }

//     fn put_item_order_txn(
//         &self,
//         txn: &mut Transaction,
//         id: &str,
//         order: usize,
//     ) -> anyhow::Result<()> {
//         self.real.put_item_order_txn(txn, id, order)
//     }

//     fn put_expanded_items_txn(
//         &self,
//         txn: &mut Transaction,
//         expanded_entries: &HashSet<CollectionId>,
//     ) -> anyhow::Result<()> {
//         self.real.put_expanded_items_txn(txn, expanded_entries)
//     }

//     fn get_expanded_items(&self) -> anyhow::Result<HashSet<CollectionId>> {
//         self.real.get_expanded_items()
//     }

//     fn remove_item_metadata_txn(
//         &self,
//         txn: &mut Transaction,
//         segkey_prefix: SegKeyBuf,
//     ) -> DatabaseResult<()> {
//         self.real.remove_item_metadata_txn(txn, segkey_prefix)
//     }

//     fn list_items_metadata(
//         &self,
//         segkey_prefix: SegKeyBuf,
//     ) -> DatabaseResult<HashMap<SegKeyBuf, AnyValue>> {
//         self.real.list_items_metadata(segkey_prefix)
//     }

//     fn get_layout_cache(&self) -> anyhow::Result<HashMap<SegKeyBuf, AnyValue>> {
//         self.real.get_layout_cache()
//     }

//     fn put_sidebar_layout(
//         &self,
//         position: SidebarPosition,
//         size: usize,
//         visible: bool,
//     ) -> anyhow::Result<()> {
//         self.real.put_sidebar_layout(position, size, visible)
//     }

//     fn put_panel_layout(&self, size: usize, visible: bool) -> anyhow::Result<()> {
//         self.real.put_panel_layout(size, visible)
//     }

//     fn put_activitybar_layout(
//         &self,
//         last_active_container_id: Option<String>,
//         position: ActivitybarPosition,
//     ) -> anyhow::Result<()> {
//         self.real
//             .put_activitybar_layout(last_active_container_id, position)
//     }

//     fn put_editor_layout(
//         &self,
//         grid: EditorGridStateEntity,
//         panels: HashMap<String, EditorPanelStateEntity>,
//         active_group: Option<String>,
//     ) -> anyhow::Result<()> {
//         self.real.put_editor_layout(grid, panels, active_group)
//     }
// }
