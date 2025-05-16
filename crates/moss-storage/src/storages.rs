pub mod collection_storage;
pub mod common;
pub mod global_storage;
pub mod workspace_storage;

use crate::{
    collection_storage::VariableStore as CollectionVariableStore, common::item_store::ItemStore,
    primitives::segkey::SegKeyBuf, storage::Transactional,
    workspace_storage::VariableStore as WorkspaceVariableStore,
};

use moss_db::primitives::AnyValue;
use std::sync::Arc;

pub trait GlobalStorage: Transactional + Send + Sync {
    fn item_store(&self) -> Arc<dyn ItemStore<SegKeyBuf, AnyValue>>;
}

pub trait WorkspaceStorage: Transactional + Send + Sync {
    fn variable_store(&self) -> Arc<dyn WorkspaceVariableStore>;
    fn item_store(&self) -> Arc<dyn ItemStore<SegKeyBuf, AnyValue>>;
}

pub trait CollectionStorage: Transactional + Send + Sync {
    fn variable_store(&self) -> Arc<dyn CollectionVariableStore>;
    fn unit_store(&self) -> Arc<dyn ItemStore<SegKeyBuf, AnyValue>>;
}
