pub mod sqlite;

use async_trait::async_trait;

// TODO: Split into:
// (a) a neutral KeyValueStore and
// (b) optional "infrastructure capabilities" (Flushable, Optimizable).

#[async_trait]
pub trait StorageAdapter: Send + Sync {
    async fn put(&self, key: &str, value: &str) -> joinerror::Result<()>;
    async fn get(&self, key: &str) -> joinerror::Result<String>;
    async fn remove(&self, key: &str) -> joinerror::Result<()>;

    async fn when_flushed(&self) -> joinerror::Result<()>;
    async fn flush(&self) -> joinerror::Result<()>;
    async fn optimize(&self) -> joinerror::Result<()>;
}
