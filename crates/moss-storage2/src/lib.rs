pub mod adapters;
pub mod application_storage;
pub mod models;
pub mod project_storage;
pub mod workspace_storage;

use crate::models::primitives::StorageScope;
use async_trait::async_trait;
use sapic_core::context::AnyAsyncContext;
use serde_json::Value as JsonValue;
use std::{sync::Arc, time::Instant};

#[async_trait]
pub trait SubstoreManager: Send + Sync {
    async fn add_workspace(&self, workspace_id: Arc<String>) -> joinerror::Result<()>;
    async fn remove_workspace(&self, workspace_id: Arc<String>) -> joinerror::Result<()>;
    async fn add_project(
        &self,
        workspace_id: Arc<String>,
        project_id: Arc<String>,
    ) -> joinerror::Result<()>;
    async fn remove_project(
        &self,
        workspace_id: Arc<String>,
        project_id: Arc<String>,
    ) -> joinerror::Result<()>;
}

#[async_trait]
pub trait KvStorage: SubstoreManager + Send + Sync {
    async fn put(
        &self,
        ctx: &dyn AnyAsyncContext,
        scope: StorageScope,
        key: &str,
        value: JsonValue,
    ) -> joinerror::Result<()>;
    async fn get(
        &self,
        ctx: &dyn AnyAsyncContext,
        scope: StorageScope,
        key: &str,
    ) -> joinerror::Result<Option<JsonValue>>;
    async fn remove(
        &self,
        ctx: &dyn AnyAsyncContext,
        scope: StorageScope,
        key: &str,
    ) -> joinerror::Result<Option<JsonValue>>;

    async fn put_batch(
        &self,
        ctx: &dyn AnyAsyncContext,
        scope: StorageScope,
        items: &[(&str, JsonValue)],
    ) -> joinerror::Result<()>;
    async fn get_batch(
        &self,
        ctx: &dyn AnyAsyncContext,
        scope: StorageScope,
        keys: &[&str],
    ) -> joinerror::Result<Vec<(String, Option<JsonValue>)>>;
    async fn remove_batch(
        &self,
        ctx: &dyn AnyAsyncContext,
        scope: StorageScope,
        keys: &[&str],
    ) -> joinerror::Result<Vec<(String, Option<JsonValue>)>>;

    async fn get_batch_by_prefix(
        &self,
        ctx: &dyn AnyAsyncContext,
        scope: StorageScope,
        prefix: &str,
    ) -> joinerror::Result<Vec<(String, JsonValue)>>;

    async fn remove_batch_by_prefix(
        &self,
        ctx: &dyn AnyAsyncContext,
        scope: StorageScope,
        prefix: &str,
    ) -> joinerror::Result<Vec<(String, JsonValue)>>;

    async fn capabilities(self: Arc<Self>) -> Arc<dyn KvStorageCapabilities>;
}

pub enum FlushMode {
    Checkpoint,
    Force,
}

#[async_trait]
pub trait KvStorageCapabilities: Send + Sync {
    async fn last_checkpoint(&self) -> Option<Instant>;
    async fn flush(&self, mode: FlushMode) -> joinerror::Result<()>;
    async fn optimize(&self) -> joinerror::Result<()>;
}
