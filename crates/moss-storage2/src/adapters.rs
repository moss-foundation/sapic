pub mod sqlite;

use async_trait::async_trait;
use sapic_core::context::AnyAsyncContext;
use serde_json::Value as JsonValue;
use std::{sync::Arc, time::Duration};

#[derive(Debug, Clone)]
pub struct Options {
    pub in_memory: Option<bool>,
    pub busy_timeout: Option<Duration>,
}

#[derive(Default, Clone)]
pub struct Capabilities {
    pub flushable: Option<Arc<dyn Flushable>>,
    pub optimizable: Option<Arc<dyn Optimizable>>,
    pub closable: Option<Arc<dyn Closable>>,
}

#[async_trait]
pub trait Optimizable: Send + Sync {
    /// Periodic heavy maintenance: refresh stats and reclaim space.
    /// Call rarely (e.g., after big deletes or on a schedule).
    async fn optimize(&self) -> joinerror::Result<()>;
}

#[async_trait]
pub trait Flushable: Send + Sync {
    /// Gentle WAL checkpoint for periodic idle maintenance.
    /// Use on a timer to keep WAL size reasonable.
    async fn checkpoint(&self) -> joinerror::Result<()>;

    /// Strong checkpoint intended for shutdown or context switches.
    /// Tries to truncate WAL to minimize startup cost.
    async fn flush(&self) -> joinerror::Result<()>;
}

#[async_trait]
pub trait Closable: Send + Sync {
    /// Close the underlying database connection
    /// Right now only useful for proper cleanup after tests
    async fn close(&self);
}

#[async_trait]
pub trait KeyedStorage: Send + Sync {
    /// Upserts `value` at `key`.
    /// Writes to SQLite, then updates the in-memory cache (write-through).
    async fn put(
        &self,
        ctx: &dyn AnyAsyncContext,
        key: &str,
        value: JsonValue,
    ) -> joinerror::Result<()>;

    /// Gets `key` from the in-memory cache; on miss, reads from SQLite,
    /// caches the value, and returns it.
    async fn get(
        &self,
        ctx: &dyn AnyAsyncContext,
        key: &str,
    ) -> joinerror::Result<Option<JsonValue>>;

    /// Removes `key` from the in-memory cache and SQLite.
    /// Returns the removed value.
    async fn remove(
        &self,
        ctx: &dyn AnyAsyncContext,
        key: &str,
    ) -> joinerror::Result<Option<JsonValue>>;

    /// Upserts `values` at `keys`.
    /// Writes to SQLite, then updates the in-memory cache (write-through).
    async fn put_batch(
        &self,
        ctx: &dyn AnyAsyncContext,
        items: &[(&str, JsonValue)],
    ) -> joinerror::Result<()>;

    /// Gets `keys` from the in-memory cache; on miss, reads from SQLite,
    /// caches the values, and returns them.
    async fn get_batch(
        &self,
        ctx: &dyn AnyAsyncContext,
        keys: &[&str],
    ) -> joinerror::Result<Vec<(String, Option<JsonValue>)>>;

    /// Removes `keys` from the in-memory cache and SQLite.
    /// Returns the removed values.
    async fn remove_batch(
        &self,
        ctx: &dyn AnyAsyncContext,
        keys: &[&str],
    ) -> joinerror::Result<Vec<(String, Option<JsonValue>)>>;

    /// Get `keys` based on the prefix; bypass the cache and read from SQLite to benefit from index
    /// caches the values, and returns them
    async fn get_batch_by_prefix(
        &self,
        ctx: &dyn AnyAsyncContext,
        prefix: &str,
    ) -> joinerror::Result<Vec<(String, JsonValue)>>;

    /// Remove `keys` based on the prefix from the in-memory
    /// Returns the removed values
    async fn remove_batch_by_prefix(
        &self,
        ctx: &dyn AnyAsyncContext,
        prefix: &str,
    ) -> joinerror::Result<Vec<(String, JsonValue)>>;
}
