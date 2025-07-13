pub mod collection_storage;
pub mod global_storage;
pub mod workspace_storage;

use std::sync::Arc;

use moss_applib::context::AnyAsyncContext;

use crate::{
    collection_storage::stores::{CollectionResourceStore, CollectionVariableStore},
    global_storage::stores::{GlobalItemStore, GlobalLogStore},
    storage::{Storage, Transactional},
    workspace_storage::stores::{WorkspaceItemStore, WorkspaceVariableStore},
};

pub trait GlobalStorage<Context: AnyAsyncContext>:
    Storage<Context> + Transactional<Context> + Send + Sync
{
    fn item_store(&self) -> Arc<dyn GlobalItemStore<Context>>;
    fn log_store(&self) -> Arc<dyn GlobalLogStore<Context>>;
}

pub trait WorkspaceStorage<Context: AnyAsyncContext>:
    Storage<Context> + Transactional<Context> + Send + Sync
{
    fn variable_store(&self) -> Arc<dyn WorkspaceVariableStore<Context>>;
    fn item_store(&self) -> Arc<dyn WorkspaceItemStore<Context>>;
}

pub trait CollectionStorage<Context: AnyAsyncContext>:
    Storage<Context> + Transactional<Context> + Send + Sync
{
    fn variable_store(&self) -> Arc<dyn CollectionVariableStore<Context>>;
    fn resource_store(&self) -> Arc<dyn CollectionResourceStore<Context>>;
}
