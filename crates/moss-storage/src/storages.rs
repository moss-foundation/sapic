pub mod common;
pub mod global_storage;
pub mod workspace_storage;

use moss_db::primitives::AnyValue;
use std::sync::Arc;
use workspace_storage::VariableStore;

use crate::{common::item_store::ItemStore, primitives::segkey::SegKeyBuf, storage::Transactional};

pub trait GlobalStorage: Transactional + Send + Sync {
    fn item_store(&self) -> Arc<dyn ItemStore<SegKeyBuf, AnyValue>>;
}

pub trait WorkspaceStorage: Transactional + Send + Sync {
    fn variable_store(&self) -> Arc<dyn VariableStore>;
    fn item_store(&self) -> Arc<dyn ItemStore<SegKeyBuf, AnyValue>>;
}
