pub mod collection_storage;
pub mod global_storage;
pub mod workspace_storage;

use std::sync::Arc;

use crate::{
    collection_storage::stores::{
        CollectionResourceStore, CollectionVariableStore, MixedStore as CollectionMixedStore,
    },
    global_storage::stores::{GlobalItemStore, GlobalLogStore},
    storage::{Storage, Transactional},
    workspace_storage::stores::{WorkspaceItemStore, WorkspaceVariableStore},
};

pub trait GlobalStorage: Storage + Transactional + Send + Sync {
    fn item_store(&self) -> Arc<dyn GlobalItemStore>;
    fn log_store(&self) -> Arc<dyn GlobalLogStore>;
}

pub trait WorkspaceStorage: Storage + Transactional + Send + Sync {
    fn variable_store(&self) -> Arc<dyn WorkspaceVariableStore>;
    fn item_store(&self) -> Arc<dyn WorkspaceItemStore>;
}

pub trait CollectionStorage: Storage + Transactional + Send + Sync {
    fn variable_store(&self) -> Arc<dyn CollectionVariableStore>;
    fn resource_store(&self) -> Arc<dyn CollectionResourceStore>;
    // fn mixed_store(&self) -> Arc<dyn CollectionMixedStore>;
}
