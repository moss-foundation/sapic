pub mod collection_storage;
pub mod common;
pub mod global_storage;
pub mod workspace_storage;

use async_trait::async_trait;
use moss_db::primitives::AnyValue;
use std::sync::Arc;

use crate::storage::ResettableStorage;
use crate::{
    collection_storage::VariableStore as CollectionVariableStore, common::item_store::ItemStore,
    primitives::segkey::SegKeyBuf, storage::Transactional,
    workspace_storage::VariableStore as WorkspaceVariableStore,
};

pub trait GlobalStorage: Transactional + Send + Sync {
    fn item_store(&self) -> Arc<dyn ItemStore<SegKeyBuf, AnyValue>>;
}

pub trait WorkspaceStorage: Transactional + Send + Sync {
    fn variable_store(&self) -> Arc<dyn WorkspaceVariableStore>;
    fn item_store(&self) -> Arc<dyn ItemStore<SegKeyBuf, AnyValue>>;
}

#[async_trait]
pub trait CollectionStorage: ResettableStorage + Transactional + Send + Sync {
    async fn variable_store(&self) -> Arc<dyn CollectionVariableStore>;
    async fn unit_store(&self) -> Arc<dyn ItemStore<SegKeyBuf, AnyValue>>;
}
