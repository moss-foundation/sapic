pub mod collection_store;
pub mod environment_store;
pub mod layout_parts_state_store;
pub mod state_db_manager;

use anyhow::Result;
use moss_db::{
    bincode_table::BincodeTable,
    common::{AnyEntity, DatabaseError},
    Transaction,
};
use std::{collections::HashMap, path::PathBuf, sync::Arc};

use crate::models::{entities::*, types::EnvironmentName};

pub(crate) type CollectionStoreTable<'a> = BincodeTable<'a, String, CollectionEntity>;

pub trait CollectionStore: Send + Sync + 'static {
    fn begin_write(&self) -> Result<(Transaction, &CollectionStoreTable)>;
    fn begin_read(&self) -> Result<(Transaction, &CollectionStoreTable)>;
    fn scan(&self) -> Result<Vec<(PathBuf, CollectionEntity)>>;
}

pub(crate) type EnvironmentStoreTable<'a> = BincodeTable<'a, EnvironmentName, EnvironmentEntity>;

pub trait EnvironmentStore: Send + Sync + 'static {
    fn scan(&self) -> Result<HashMap<EnvironmentName, EnvironmentEntity>>;
}

pub(crate) type LayoutPartsStateStoreTable<'a> = BincodeTable<'a, String, AnyEntity>;

pub trait LayoutPartsStateStore: Send + Sync + 'static {
    fn get_sidebar_part_state(&self) -> Result<SidebarPartStateEntity, DatabaseError>;
    fn get_panel_part_state(&self) -> Result<PanelPartStateEntity, DatabaseError>;
    fn get_editor_part_state(&self) -> Result<EditorPartStateEntity, DatabaseError>;

    fn put_sidebar_part_state(&self, state: SidebarPartStateEntity) -> Result<(), DatabaseError>;
    fn put_panel_part_state(&self, state: PanelPartStateEntity) -> Result<(), DatabaseError>;
    fn put_editor_part_state(&self, state: EditorPartStateEntity) -> Result<(), DatabaseError>;

    fn delete_sidebar_part_state(&self) -> Result<(), DatabaseError>;
    fn delete_panel_part_state(&self) -> Result<(), DatabaseError>;
    fn delete_editor_part_state(&self) -> Result<(), DatabaseError>;
}

pub trait StateDbManager: Send + Sync + 'static {
    fn collection_store(&self) -> Arc<dyn CollectionStore>;
    fn environment_store(&self) -> Arc<dyn EnvironmentStore>;
    fn layout_parts_state_store(&self) -> Arc<dyn LayoutPartsStateStore>;
}
