pub mod entities;
pub mod workbench_store;

mod storage_impl;
use std::sync::Arc;

pub use storage_impl::*;

use crate::storage::Transactional;

pub trait GlobalStorage: Transactional + Send + Sync {
    fn workspaces_store(&self) -> Arc<dyn WorkbenchStore>;
}
