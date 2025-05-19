use async_trait::async_trait;
use moss_db::DatabaseResult;
use serde_json::Value as JsonValue;
use std::any::TypeId;
use std::collections::HashMap;
use std::path::Path;
use std::pin::Pin;
use std::sync::Arc;

use crate::storage::Transactional;
use crate::storage::table::Table;

mod new_resettable_storage;
mod new_storage;

#[async_trait]
pub trait Dump {
    async fn dump(&self) -> DatabaseResult<HashMap<String, JsonValue>>;
}

#[async_trait]
pub trait NewStorage: Transactional + Dump + Send + Sync {
    async fn table(&self, id: &TypeId) -> DatabaseResult<Arc<dyn Table>>;
}

#[async_trait]
pub trait NewReset {
    async fn reset(
        &self,
        path: &Path,
        after_drop: Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>>, // TODO: change to DatabaseResult
    ) -> anyhow::Result<()>; // TODO: change to DatabaseResult
}

#[async_trait]
pub trait NewResettableStorage: NewStorage + NewReset {}
