pub mod collection_storage;
pub mod global_storage;
pub mod workspace_storage;

use std::{any::TypeId, sync::Arc};

use crate::{
    collection_storage::stores::{CollectionUnitStore, CollectionVariableStore},
    global_storage::stores::{
        AppLogCache, GlobalItemStore, SessionLogCache, sessionlog_cache::SessionLogCacheImpl,
    },
    storage::{Storage, Transactional},
    workspace_storage::stores::{WorkspaceItemStore, WorkspaceVariableStore},
};

pub trait GlobalStorage: Storage + Transactional + Send + Sync {
    fn item_store(&self) -> Arc<dyn GlobalItemStore>;
    fn applog_cache(&self) -> Arc<dyn AppLogCache>;
    fn sessionlog_cache(&self) -> Arc<dyn SessionLogCache>;
}

pub trait WorkspaceStorage: Storage + Transactional + Send + Sync {
    fn variable_store(&self) -> Arc<dyn WorkspaceVariableStore>;
    fn item_store(&self) -> Arc<dyn WorkspaceItemStore>;
}

pub trait CollectionStorage: Storage + Transactional + Send + Sync {
    fn variable_store(&self) -> Arc<dyn CollectionVariableStore>;
    fn unit_store(&self) -> Arc<dyn CollectionUnitStore>;
}
