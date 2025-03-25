pub mod collection_store;
pub mod environment_store;
pub mod state_db_manager;

use anyhow::Result;
use moss_db::{bincode_table::BincodeTable, Transaction};
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
