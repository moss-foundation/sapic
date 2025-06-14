pub mod collection_storage;
pub mod global_storage;
pub mod workspace_storage;

use std::sync::Arc;

use crate::{
    collection_storage::stores::{
        CollectionUnitStore, CollectionVariableStore, MixedStore as CollectionMixedStore,
    },
    global_storage::stores::GlobalItemStore,
    storage::{Storage, Transactional},
    workspace_storage::stores::{WorkspaceItemStore, WorkspaceVariableStore},
};

pub trait GlobalStorage: Storage + Transactional + Send + Sync {
    fn item_store(&self) -> Arc<dyn GlobalItemStore>;
}

pub trait WorkspaceStorage: Storage + Transactional + Send + Sync {
    fn variable_store(&self) -> Arc<dyn WorkspaceVariableStore>;
    fn item_store(&self) -> Arc<dyn WorkspaceItemStore>;
}

pub trait CollectionStorage: Storage + Transactional + Send + Sync {
    fn variable_store(&self) -> Arc<dyn CollectionVariableStore>;
    fn unit_store(&self) -> Arc<dyn CollectionUnitStore>;
    fn mixed_store(&self) -> Arc<dyn CollectionMixedStore>;
}
