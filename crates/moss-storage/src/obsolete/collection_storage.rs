pub mod entities;
pub mod request_store;
pub mod state_store;

mod storage_impl;
use std::sync::Arc;

pub use storage_impl::*;

use async_trait::async_trait;

use crate::storage::{ResettableStorage, Transactional};

#[async_trait]
pub trait CollectionStorage: ResettableStorage + Transactional + Send + Sync {
    async fn request_store(&self) -> Arc<dyn RequestStore>;
    async fn state_store(&self) -> Arc<dyn StateStore>;
}
