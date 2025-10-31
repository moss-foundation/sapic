pub mod sqlite;

use async_trait::async_trait;
use serde_json::Value as JsonValue;
use std::sync::Arc;

#[derive(Default, Clone)]
pub struct Capabilities {
    pub flushable: Option<Arc<dyn Flushable>>,
    pub optimizable: Option<Arc<dyn Optimizable>>,
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
pub trait KeyedStorage: Send + Sync {
    /// Upserts `value` at `key`.
    /// Writes to SQLite, then updates the in-memory cache (write-through).
    async fn put(&self, key: &str, value: JsonValue) -> joinerror::Result<()>;

    /// Gets `key` from the in-memory cache; on miss, reads from SQLite,
    /// caches the value, and returns it.
    async fn get(&self, key: &str) -> joinerror::Result<Option<JsonValue>>;

    /// Removes `key` from the in-memory cache and SQLite.
    async fn remove(&self, key: &str) -> joinerror::Result<()>;

    /// Upserts `values` at `keys`.
    /// Writes to SQLite, then updates the in-memory cache (write-through).
    async fn put_batch(&self, keys: &[&str], values: &[JsonValue]) -> joinerror::Result<()>;

    /// Gets `keys` from the in-memory cache; on miss, reads from SQLite,
    /// caches the values, and returns them.
    async fn get_batch(&self, keys: &[&str]) -> joinerror::Result<Vec<Option<JsonValue>>>;

    /// Removes `keys` from the in-memory cache and SQLite.
    async fn remove_batch(&self, keys: &[&str]) -> joinerror::Result<()>;
}
