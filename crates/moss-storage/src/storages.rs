pub mod collection_storage;
pub mod common;
pub mod global_storage;
pub mod workspace_storage;

use std::sync::Arc;

use sapic_core::context::AnyAsyncContext;

use crate::{
    collection_storage::stores::CollectionResourceStore,
    common::VariableStore,
    global_storage::stores::{GlobalItemStore, GlobalLogStore},
    storage::{Storage, Transactional, TransactionalWithContext},
    workspace_storage::stores::WorkspaceItemStore,
};

pub trait GlobalStorage<Context: AnyAsyncContext>:
    Storage<Context> + TransactionalWithContext<Context> + Transactional + Send + Sync
{
    fn item_store(&self) -> Arc<dyn GlobalItemStore<Context>>;
    fn log_store(&self) -> Arc<dyn GlobalLogStore<Context>>;
}

pub trait WorkspaceStorage<Context: AnyAsyncContext>:
    Storage<Context> + TransactionalWithContext<Context> + Send + Sync
{
    fn variable_store(&self) -> Arc<dyn VariableStore<Context>>;
    fn item_store(&self) -> Arc<dyn WorkspaceItemStore<Context>>;
}

pub trait CollectionStorage<Context: AnyAsyncContext>:
    Storage<Context> + TransactionalWithContext<Context> + Send + Sync
{
    fn variable_store(&self) -> Arc<dyn VariableStore<Context>>;
    fn resource_store(&self) -> Arc<dyn CollectionResourceStore<Context>>;
}
