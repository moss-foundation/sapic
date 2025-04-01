pub mod collection_store;
pub mod environment_store;
pub mod global_db_manager;
pub mod state_db_manager;
pub mod workspace_store;

use anyhow::Result;
use moss_db::{bincode_table::BincodeTable, Transaction};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf, sync::Arc};

use crate::models::{
    entities::{CollectionEntity, EnvironmentEntity},
    types::EnvironmentName,
};

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

pub trait StateDbManager: Send + Sync + 'static {
    fn collection_store(&self) -> Arc<dyn CollectionStore>;
    fn environment_store(&self) -> Arc<dyn EnvironmentStore>;
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WorkspaceEntity {
    // FIXME: What should we store here?
}

pub(crate) type WorkspaceStoreTable<'a> = BincodeTable<'a, String, WorkspaceEntity>;

pub trait WorkspaceStore: Send + Sync + 'static {
    fn begin_write(&self) -> Result<(Transaction, &WorkspaceStoreTable)>;
    fn begin_read(&self) -> Result<(Transaction, &WorkspaceStoreTable)>;
    fn scan(&self) -> Result<Vec<(PathBuf, WorkspaceEntity)>>;
}

pub trait GlobalDbManager: Send + Sync + 'static {
    fn workspace_store(&self) -> Arc<dyn WorkspaceStore>;
}
