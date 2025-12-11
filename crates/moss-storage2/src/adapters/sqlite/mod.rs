use crate::adapters::{Closable, Flushable, KeyedStorage, Optimizable, Options};
use async_trait::async_trait;
use joinerror::ResultExt;
use moss_logging::session;
use sapic_core::{
    context,
    context::{AnyAsyncContext, ContextResultExt},
};
use serde_json::Value as JsonValue;
use sqlx::{
    Row, SqlitePool,
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions},
};
use std::{collections::HashMap, path::Path, str::FromStr, sync::Arc, time::Duration};
use tokio::sync::RwLock;

const DEFAULT_BUSY_TIMEOUT: Duration = Duration::from_secs(5);
const DEFAULT_IN_MEMORY: bool = false;

pub struct SqliteStorageOptions {
    pub in_memory: bool,
    pub busy_timeout: Duration,
}

impl Into<SqliteStorageOptions> for Options {
    fn into(self) -> SqliteStorageOptions {
        SqliteStorageOptions {
            in_memory: self.in_memory.unwrap_or(DEFAULT_IN_MEMORY),
            busy_timeout: self.busy_timeout.unwrap_or(DEFAULT_BUSY_TIMEOUT),
        }
    }
}

impl Default for SqliteStorageOptions {
    fn default() -> Self {
        Self {
            in_memory: DEFAULT_IN_MEMORY,
            busy_timeout: DEFAULT_BUSY_TIMEOUT,
        }
    }
}

pub struct SqliteStorage {
    cache: RwLock<HashMap<String, JsonValue>>,
    pool: SqlitePool,
}

impl SqliteStorage {
    pub async fn new(
        path: impl AsRef<Path>,
        options: Option<SqliteStorageOptions>,
    ) -> joinerror::Result<Arc<Self>> {
        if path.as_ref().exists() {
            let _ = std::fs::copy(&path.as_ref(), path.as_ref().with_extension("bak"));
        }

        let opts = options.unwrap_or_default();
        let url = format!("sqlite://{}", path.as_ref().display());
        let options = SqliteConnectOptions::from_str(&url)
            .join_err::<()>("failed to create connect options")?
            .in_memory(opts.in_memory)
            .create_if_missing(true)
            .journal_mode(SqliteJournalMode::Wal)
            .synchronous(sqlx::sqlite::SqliteSynchronous::Normal)
            .busy_timeout(opts.busy_timeout)
            .foreign_keys(true);

        // TODO: This should solve most lock related issues but is not the most performant approach
        // We might consider https://github.com/launchbadge/sqlx/issues/459
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect_with(options)
            .await
            .join_err::<()>("failed to open database")?;

        {
            sqlx::query("PRAGMA foreign_keys=ON;")
                .execute(&pool)
                .await?;
            sqlx::query("PRAGMA synchronous=NORMAL;")
                .execute(&pool)
                .await?;
            sqlx::query("PRAGMA busy_timeout=5000;")
                .execute(&pool)
                .await?;
        }

        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .join_err::<()>("failed to run migrations")?;

        let mut cache = HashMap::new();
        if let Ok(rows) = sqlx::query("SELECT key, value FROM kv")
            .fetch_all(&pool)
            .await
        {
            for row in rows {
                let key: String = row.get("key");
                let value: Vec<u8> = row.get("value");

                let value: JsonValue = match serde_json::from_slice(&value) {
                    Ok(value) => value,
                    Err(err) => {
                        session::trace!("failed to deserialize value: {}", err.to_string());
                        continue;
                    }
                };

                cache.insert(key, value);
            }
        } else {
            session::error!("failed to fetch database cache");
        };

        Ok(Arc::new(Self {
            pool,
            cache: RwLock::new(cache),
        }))
    }
}
#[async_trait]
impl KeyedStorage for SqliteStorage {
    async fn put(
        &self,
        ctx: &dyn AnyAsyncContext,
        key: &str,
        value: JsonValue,
    ) -> joinerror::Result<()> {
        context::abortable::<_, _, joinerror::Error, _>(ctx, async {
            let s = serde_json::to_string(&value).join_err::<()>("failed to serialize value")?;

            sqlx::query(
                r#"
        INSERT INTO kv (key, value) VALUES (?, ?)
        ON CONFLICT(key) DO UPDATE SET value=excluded.value
    "#,
            )
            .bind(key)
            .bind(s)
            .execute(&self.pool)
            .await
            .join_err::<()>("failed to insert value")?;

            self.cache.write().await.insert(key.to_string(), value);
            Ok(())
        })
        .await
        .join_err_bare()
    }

    async fn get(
        &self,
        ctx: &dyn AnyAsyncContext,
        key: &str,
    ) -> joinerror::Result<Option<JsonValue>> {
        let response = context::abortable::<_, _, joinerror::Error, _>(ctx, async {
            if let Some(v) = self.cache.read().await.get(key) {
                return Ok(Some(v.clone()));
            }

            if let Some(row) = sqlx::query("SELECT value FROM kv WHERE key = ?")
                .bind(key)
                .fetch_optional(&self.pool)
                .await?
            {
                let bytes: Vec<u8> = row.get("value");
                let value: JsonValue =
                    serde_json::from_slice(&bytes).join_err::<()>("failed to deserialize value")?;
                self.cache
                    .write()
                    .await
                    .insert(key.to_string(), value.clone());
                Ok(Some(value))
            } else {
                Ok(None)
            }
        })
        .await
        .join_err_bare()?;

        Ok(response)
    }

    async fn remove(
        &self,
        ctx: &dyn AnyAsyncContext,
        key: &str,
    ) -> joinerror::Result<Option<JsonValue>> {
        let response = context::abortable::<_, _, joinerror::Error, _>(ctx, async {
            let row = sqlx::query(
                r#"
            DELETE FROM kv
            WHERE key = ?
            RETURNING value
            "#,
            )
            .bind(key)
            .fetch_optional(&self.pool)
            .await
            .join_err::<()>("failed to delete and return value")?;

            let value = row.map(|r| {
                let raw: String = r.get("value");
                serde_json::from_str::<JsonValue>(&raw).unwrap_or(JsonValue::Null)
            });

            if value.is_some() {
                self.cache.write().await.remove(key);
            }

            Ok(value)
        })
        .await
        .join_err_bare()?;

        Ok(response)
    }

    async fn put_batch(
        &self,
        ctx: &dyn AnyAsyncContext,
        items: &[(&str, JsonValue)],
    ) -> joinerror::Result<()> {
        context::abortable::<_, _, joinerror::Error, _>(ctx, async {
            if items.is_empty() {
                return Ok(());
            }

            let mut txn = self
                .pool
                .begin()
                .await
                .join_err::<()>("failed to begin transaction")?;

            for (key, value) in items {
                let s =
                    serde_json::to_string(&value).join_err::<()>("failed to serialize value")?;

                sqlx::query(
                    r#"
                INSERT INTO kv (key, value) VALUES (?, ?)
                ON CONFLICT(key) DO UPDATE SET value=excluded.value
            "#,
                )
                .bind(key)
                .bind(s)
                .execute(&mut *txn)
                .await
                .join_err::<()>("failed to insert value")?;

                self.cache
                    .write()
                    .await
                    .insert((*key).to_string(), value.clone());
            }

            txn.commit()
                .await
                .join_err::<()>("failed to commit transaction")?;

            Ok(())
        })
        .await
        .join_err_bare()
    }

    async fn get_batch(
        &self,
        ctx: &dyn AnyAsyncContext,
        keys: &[&str],
    ) -> joinerror::Result<Vec<(String, Option<JsonValue>)>> {
        let response = context::abortable::<_, _, joinerror::Error, _>(ctx, async {
            if keys.is_empty() {
                return Ok(vec![]);
            }

            let cache = self.cache.read().await;
            let mut cache_hits: HashMap<String, JsonValue> = HashMap::new();
            let mut keys_to_fetch = Vec::new();

            for key in keys {
                if let Some(value) = cache.get(*key) {
                    cache_hits.insert((*key).to_string(), value.clone());
                } else {
                    keys_to_fetch.push(*key);
                }
            }
            drop(cache);

            if keys_to_fetch.is_empty() {
                let mut result = Vec::with_capacity(keys.len());
                for key in keys {
                    result.push((key.to_string(), cache_hits.get(*key).cloned()));
                }
                return Ok(result);
            }

            let json_keys = serde_json::to_string(&keys_to_fetch).unwrap();
            let rows = sqlx::query(
                r#"
            SELECT key, value FROM kv
            WHERE key IN (SELECT value FROM json_each(?))
            "#,
            )
            .bind(json_keys)
            .fetch_all(&self.pool)
            .await
            .join_err::<()>("failed to fetch values")?;

            let mut db_results: HashMap<String, JsonValue> = HashMap::with_capacity(rows.len());
            {
                let mut cache = self.cache.write().await;
                for row in rows {
                    let key: String = row.get("key");
                    let bytes: Vec<u8> = row.get("value");
                    if let Ok(value) = serde_json::from_slice::<JsonValue>(&bytes) {
                        db_results.insert(key.clone(), value.clone());
                        cache.insert(key, value);
                    }
                }
            }

            let mut result = Vec::with_capacity(keys.len());
            for key in keys {
                let value = cache_hits
                    .get(*key)
                    .or_else(|| db_results.get(*key))
                    .cloned();
                result.push((key.to_string(), value));
            }

            Ok(result)
        })
        .await
        .join_err_bare()?;

        Ok(response)
    }

    async fn remove_batch(
        &self,
        ctx: &dyn AnyAsyncContext,
        keys: &[&str],
    ) -> joinerror::Result<Vec<(String, Option<JsonValue>)>> {
        let response = context::abortable::<_, _, joinerror::Error, _>(ctx, async {
            if keys.is_empty() {
                return Ok(vec![]);
            }

            let mut txn = self
                .pool
                .begin()
                .await
                .join_err::<()>("failed to begin transaction")?;

            let json_keys = serde_json::to_string(&keys).unwrap();
            let rows = sqlx::query(
                r#"
            DELETE FROM kv
            WHERE key IN (SELECT value FROM json_each(?))
            RETURNING key, value
            "#,
            )
            .bind(json_keys)
            .fetch_all(&mut *txn)
            .await
            .join_err::<()>("failed to delete and return values")?;

            txn.commit()
                .await
                .join_err::<()>("failed to commit transaction")?;

            let mut removed_map: HashMap<String, JsonValue> = HashMap::with_capacity(rows.len());
            for row in rows {
                let key: String = row.get("key");
                let raw_value: String = row.get("value");
                if let Ok(value) = serde_json::from_str::<JsonValue>(&raw_value) {
                    removed_map.insert(key, value);
                }
            }

            {
                let mut cache = self.cache.write().await;
                for key in keys {
                    cache.remove(*key);
                }
            }

            let mut result = Vec::with_capacity(keys.len());
            for key in keys {
                let value = removed_map.remove(*key);
                result.push((key.to_string(), value));
            }

            Ok(result)
        })
        .await
        .join_err_bare()?;

        Ok(response)
    }

    async fn get_batch_by_prefix(
        &self,
        ctx: &dyn AnyAsyncContext,
        prefix: &str,
    ) -> joinerror::Result<Vec<(String, JsonValue)>> {
        let response = context::abortable::<_, _, joinerror::Error, _>(ctx, async {
            let pattern = format!("{prefix}%");
            let rows = sqlx::query(
                r#"
            SELECT key, value FROM kv
            WHERE key LIKE ?
            "#,
            )
            .bind(pattern)
            .fetch_all(&self.pool)
            .await
            .join_err::<()>("failed to fetch values")?;

            let mut db_results: HashMap<String, JsonValue> = HashMap::with_capacity(rows.len());

            {
                let mut cache = self.cache.write().await;
                for row in rows {
                    let key: String = row.get("key");
                    let bytes: Vec<u8> = row.get("value");
                    if let Ok(value) = serde_json::from_slice::<JsonValue>(&bytes) {
                        db_results.insert(key.clone(), value.clone());
                        cache.insert(key, value);
                    }
                }
            }
            Ok(db_results.into_iter().collect())
        })
        .await
        .join_err_bare()?;

        Ok(response)
    }

    async fn remove_batch_by_prefix(
        &self,
        ctx: &dyn AnyAsyncContext,
        prefix: &str,
    ) -> joinerror::Result<Vec<(String, JsonValue)>> {
        let response = context::abortable::<_, _, joinerror::Error, _>(ctx, async {
            let pattern = format!("{prefix}%");

            let mut txn = self
                .pool
                .begin()
                .await
                .join_err::<()>("failed to begin transaction")?;

            let rows = sqlx::query(
                r#"
            DELETE FROM kv
            WHERE key LIKE ?
            RETURNING key, value
            "#,
            )
            .bind(pattern)
            .fetch_all(&mut *txn)
            .await
            .join_err::<()>("failed to delete and return values")?;

            txn.commit()
                .await
                .join_err::<()>("failed to commit transaction")?;

            let mut removed_map: HashMap<String, JsonValue> = HashMap::with_capacity(rows.len());
            for row in rows {
                let key: String = row.get("key");
                let raw_value: String = row.get("value");
                if let Ok(value) = serde_json::from_str::<JsonValue>(&raw_value) {
                    removed_map.insert(key, value);
                }
            }

            {
                let mut cache = self.cache.write().await;
                for key in removed_map.keys() {
                    cache.remove(key);
                }
            }

            Ok(removed_map.into_iter().collect())
        })
        .await
        .join_err_bare()?;

        Ok(response)
    }
}

#[async_trait]
impl Flushable for SqliteStorage {
    async fn checkpoint(&self) -> joinerror::Result<()> {
        sqlx::query("PRAGMA wal_checkpoint(PASSIVE);")
            .execute(&self.pool)
            .await
            .join_err::<()>("wal_checkpoint(PASSIVE) failed")?;

        Ok(())
    }

    async fn flush(&self) -> joinerror::Result<()> {
        sqlx::query("PRAGMA wal_checkpoint(TRUNCATE);")
            .execute(&self.pool)
            .await
            .join_err::<()>("wal_checkpoint(TRUNCATE) failed")?;

        sqlx::query("PRAGMA synchronous=NORMAL;")
            .execute(&self.pool)
            .await
            .ok();

        Ok(())
    }
}

#[async_trait]
impl Optimizable for SqliteStorage {
    async fn optimize(&self) -> joinerror::Result<()> {
        sqlx::query("PRAGMA optimize;")
            .execute(&self.pool)
            .await
            .join_err::<()>("PRAGMA optimize failed")?;

        sqlx::query("VACUUM;")
            .execute(&self.pool)
            .await
            .join_err::<()>("VACUUM failed")?;

        Ok(())
    }
}

#[async_trait]
impl Closable for SqliteStorage {
    async fn close(&self) {
        self.pool.close().await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::KeyedStorage;
    use sapic_core::context::ArcContext;
    use serde_json::Value as JsonValue;
    use std::path::PathBuf;

    async fn create_in_memory_storage() -> (Arc<SqliteStorage>, ArcContext) {
        let ctx = ArcContext::background();
        let temp_path = PathBuf::from(":memory:");
        let storage = SqliteStorage::new(
            temp_path,
            Some(SqliteStorageOptions {
                in_memory: true,
                busy_timeout: Duration::from_secs(5),
            }),
        )
        .await
        .expect("failed to create in-memory storage");

        (storage, ctx)
    }

    // Put tests
    #[tokio::test]
    async fn test_put_happy_path() {
        let (storage, ctx) = create_in_memory_storage().await;

        let value = JsonValue::String("test_value".to_string());
        storage.put(&ctx, "test_key", value.clone()).await.unwrap();

        let retrieved = storage.get(&ctx, "test_key").await.unwrap();
        assert_eq!(retrieved, Some(value));
    }

    #[tokio::test]
    async fn test_put_overwrite_existing() {
        let (storage, ctx) = create_in_memory_storage().await;

        let initial_value = JsonValue::String("initial".to_string());
        storage
            .put(&ctx, "test_key", initial_value.clone())
            .await
            .unwrap();

        let new_value = JsonValue::Number(42.into());
        storage
            .put(&ctx, "test_key", new_value.clone())
            .await
            .unwrap();

        let retrieved = storage.get(&ctx, "test_key").await.unwrap();
        assert_eq!(retrieved, Some(new_value));
        assert_ne!(retrieved, Some(initial_value));
    }

    #[tokio::test]
    async fn test_put_complex_json_value() {
        let (storage, ctx) = create_in_memory_storage().await;

        let complex_value = serde_json::json!({
            "string": "value",
            "number": 42,
            "boolean": true,
            "array": [1, 2, 3],
            "nested": {
                "key": "value"
            }
        });
        storage
            .put(&ctx, "complex_key", complex_value.clone())
            .await
            .unwrap();

        let retrieved = storage.get(&ctx, "complex_key").await.unwrap();
        assert_eq!(retrieved, Some(complex_value));
    }

    #[tokio::test]
    async fn test_put_empty_string_key() {
        let (storage, ctx) = create_in_memory_storage().await;

        let value = JsonValue::String("value".to_string());
        storage.put(&ctx, "", value.clone()).await.unwrap();

        let retrieved = storage.get(&ctx, "").await.unwrap();
        assert_eq!(retrieved, Some(value));
    }

    #[tokio::test]
    async fn test_put_special_characters_in_key() {
        let (storage, ctx) = create_in_memory_storage().await;

        let special_keys = [
            "key.with.dots",
            "key-with-dashes",
            "key_with_underscores",
            "key/with/slashes",
            "key@with@symbols",
            "key with spaces",
        ];

        for (i, key) in special_keys.iter().enumerate() {
            let value = JsonValue::Number(i.into());
            storage.put(&ctx, key, value.clone()).await.unwrap();

            let retrieved = storage.get(&ctx, key).await.unwrap();
            assert_eq!(retrieved, Some(value));
        }
    }

    #[tokio::test]
    async fn test_put_large_value() {
        let (storage, ctx) = create_in_memory_storage().await;

        let large_array: Vec<JsonValue> = (0..10000).map(|i| JsonValue::Number(i.into())).collect();
        let large_value = JsonValue::Array(large_array);

        storage
            .put(&ctx, "large_key", large_value.clone())
            .await
            .unwrap();

        let retrieved = storage.get(&ctx, "large_key").await.unwrap();
        assert_eq!(retrieved, Some(large_value));
    }

    #[tokio::test]
    async fn test_put_null_value() {
        let (storage, ctx) = create_in_memory_storage().await;

        let null_value = JsonValue::Null;
        storage
            .put(&ctx, "null_key", null_value.clone())
            .await
            .unwrap();

        let retrieved = storage.get(&ctx, "null_key").await.unwrap();
        assert_eq!(retrieved, Some(null_value));
    }

    #[tokio::test]
    async fn test_put_multiple_keys() {
        let (storage, ctx) = create_in_memory_storage().await;

        storage
            .put(&ctx, "key1", JsonValue::String("value1".to_string()))
            .await
            .unwrap();
        storage
            .put(&ctx, "key2", JsonValue::Number(2.into()))
            .await
            .unwrap();
        storage
            .put(&ctx, "key3", JsonValue::Bool(true))
            .await
            .unwrap();

        assert_eq!(
            storage.get(&ctx, "key1").await.unwrap(),
            Some(JsonValue::String("value1".to_string()))
        );
        assert_eq!(
            storage.get(&ctx, "key2").await.unwrap(),
            Some(JsonValue::Number(2.into()))
        );
        assert_eq!(
            storage.get(&ctx, "key3").await.unwrap(),
            Some(JsonValue::Bool(true))
        );
    }

    #[tokio::test]
    async fn test_put_unicode_key() {
        let (storage, ctx) = create_in_memory_storage().await;

        let unicode_key = "ключ_🎉_🔑";
        let value = JsonValue::String("unicode_value".to_string());
        storage.put(&ctx, unicode_key, value.clone()).await.unwrap();

        let retrieved = storage.get(&ctx, unicode_key).await.unwrap();
        assert_eq!(retrieved, Some(value));
    }

    // Get tests
    #[tokio::test]
    async fn test_get_happy_path() {
        let (storage, ctx) = create_in_memory_storage().await;

        let value = JsonValue::String("test_value".to_string());
        storage.put(&ctx, "test_key", value.clone()).await.unwrap();

        let retrieved = storage.get(&ctx, "test_key").await.unwrap();
        assert_eq!(retrieved, Some(value));
    }

    #[tokio::test]
    async fn test_get_nonexistent_key() {
        let (storage, ctx) = create_in_memory_storage().await;

        let retrieved = storage.get(&ctx, "nonexistent_key").await.unwrap();
        assert_eq!(retrieved, None);
    }

    #[tokio::test]
    async fn test_get_cache_hit() {
        let (storage, ctx) = create_in_memory_storage().await;

        let value = JsonValue::String("cached_value".to_string());
        storage.put(&ctx, "cache_key", value.clone()).await.unwrap();

        let first = storage.get(&ctx, "cache_key").await.unwrap();
        assert_eq!(first, Some(value.clone()));

        let second = storage.get(&ctx, "cache_key").await.unwrap();
        assert_eq!(second, Some(value));
    }

    #[tokio::test]
    async fn test_get_reads_from_db_when_cache_miss() {
        let (storage, ctx) = create_in_memory_storage().await;

        // Insert data directly into database, bypassing cache
        let value = JsonValue::String("db_value".to_string());
        let serialized = serde_json::to_string(&value).unwrap();
        sqlx::query("INSERT INTO kv (key, value) VALUES (?, ?)")
            .bind("db_key")
            .bind(serialized)
            .execute(&storage.pool)
            .await
            .unwrap();

        // Clear cache to ensure cache miss
        storage.cache.write().await.clear();

        // First get should be a cache miss (reads from DB)
        let retrieved1 = storage.get(&ctx, "db_key").await.unwrap();
        assert_eq!(retrieved1, Some(value.clone()));

        // Second get should be a cache hit (reads from cache)
        let retrieved2 = storage.get(&ctx, "db_key").await.unwrap();
        assert_eq!(retrieved2, Some(value));
    }

    #[tokio::test]
    async fn test_get_empty_string_key() {
        let (storage, ctx) = create_in_memory_storage().await;

        let value = JsonValue::String("empty_key_value".to_string());
        storage.put(&ctx, "", value.clone()).await.unwrap();

        let retrieved = storage.get(&ctx, "").await.unwrap();
        assert_eq!(retrieved, Some(value));
    }

    #[tokio::test]
    async fn test_get_different_value_types() {
        let (storage, ctx) = create_in_memory_storage().await;

        let test_cases = vec![
            ("string_key", JsonValue::String("string".to_string())),
            ("number_key", JsonValue::Number(42.into())),
            (
                "float_key",
                JsonValue::Number(serde_json::Number::from_f64(3.14).unwrap()),
            ),
            ("bool_true_key", JsonValue::Bool(true)),
            ("bool_false_key", JsonValue::Bool(false)),
            ("null_key", JsonValue::Null),
            (
                "array_key",
                JsonValue::Array(vec![
                    JsonValue::Number(1.into()),
                    JsonValue::Number(2.into()),
                ]),
            ),
            ("object_key", serde_json::json!({"nested": "value"})),
        ];

        for (key, value) in &test_cases {
            storage.put(&ctx, key, value.clone()).await.unwrap();
        }

        for (key, expected_value) in &test_cases {
            let retrieved = storage.get(&ctx, key).await.unwrap();
            assert_eq!(
                retrieved,
                Some(expected_value.clone()),
                "Failed for key: {}",
                key
            );
        }
    }

    #[tokio::test]
    async fn test_get_after_remove() {
        let (storage, ctx) = create_in_memory_storage().await;

        storage
            .put(&ctx, "remove_key", JsonValue::String("value".to_string()))
            .await
            .unwrap();
        storage.remove(&ctx, "remove_key").await.unwrap();

        let retrieved = storage.get(&ctx, "remove_key").await.unwrap();
        assert_eq!(retrieved, None);
    }

    #[tokio::test]
    async fn test_get_concurrent_access() {
        let (storage, ctx) = create_in_memory_storage().await;

        for i in 0..100 {
            storage
                .put(&ctx, &format!("key_{}", i), JsonValue::Number(i.into()))
                .await
                .unwrap();
        }

        let mut handles = vec![];
        for i in 0..100 {
            let storage_clone = storage.clone();
            let handle = {
                let ctx = ctx.clone();
                tokio::spawn(async move {
                    storage_clone
                        .get(&ctx, &format!("key_{}", i))
                        .await
                        .unwrap()
                })
            };

            handles.push(handle);
        }

        for (i, handle) in handles.into_iter().enumerate() {
            let result = handle.await.unwrap();
            assert_eq!(result, Some(JsonValue::Number(i.into())));
        }
    }

    // Remove tests
    #[tokio::test]
    async fn test_remove_happy_path() {
        let (storage, ctx) = create_in_memory_storage().await;

        let value = JsonValue::String("to_be_removed".to_string());
        storage
            .put(&ctx, "remove_key", value.clone())
            .await
            .unwrap();

        let removed = storage.remove(&ctx, "remove_key").await.unwrap();
        assert_eq!(removed, Some(value));

        let retrieved = storage.get(&ctx, "remove_key").await.unwrap();
        assert_eq!(retrieved, None);
    }

    #[tokio::test]
    async fn test_remove_nonexistent_key() {
        let (storage, ctx) = create_in_memory_storage().await;

        let removed = storage.remove(&ctx, "nonexistent_key").await.unwrap();
        assert_eq!(removed, None);
    }

    #[tokio::test]
    async fn test_remove_returns_correct_value() {
        let (storage, ctx) = create_in_memory_storage().await;

        let test_cases = vec![
            ("string_key", JsonValue::String("string_value".to_string())),
            ("number_key", JsonValue::Number(42.into())),
            ("bool_key", JsonValue::Bool(true)),
            ("null_key", JsonValue::Null),
            (
                "array_key",
                JsonValue::Array(vec![JsonValue::Number(1.into())]),
            ),
            ("object_key", serde_json::json!({"key": "value"})),
        ];

        for (key, value) in &test_cases {
            storage.put(&ctx, key, value.clone()).await.unwrap();
            let removed = storage.remove(&ctx, key).await.unwrap();
            assert_eq!(removed, Some(value.clone()), "Failed for key: {}", key);
        }
    }

    #[tokio::test]
    async fn test_remove_removes_from_cache() {
        let (storage, ctx) = create_in_memory_storage().await;

        let value = JsonValue::String("cached_value".to_string());
        storage.put(&ctx, "cache_key", value.clone()).await.unwrap();
        let _ = storage.get(&ctx, "cache_key").await.unwrap();
        storage.remove(&ctx, "cache_key").await.unwrap();

        let retrieved = storage.get(&ctx, "cache_key").await.unwrap();
        assert_eq!(retrieved, None);
    }

    #[tokio::test]
    async fn test_remove_empty_string_key() {
        let (storage, ctx) = create_in_memory_storage().await;

        let value = JsonValue::String("empty_key_value".to_string());
        storage.put(&ctx, "", value.clone()).await.unwrap();

        let removed = storage.remove(&ctx, "").await.unwrap();
        assert_eq!(removed, Some(value));

        let retrieved = storage.get(&ctx, "").await.unwrap();
        assert_eq!(retrieved, None);
    }

    #[tokio::test]
    async fn test_remove_multiple_keys_independently() {
        let (storage, ctx) = create_in_memory_storage().await;

        storage
            .put(&ctx, "key1", JsonValue::String("value1".to_string()))
            .await
            .unwrap();
        storage
            .put(&ctx, "key2", JsonValue::String("value2".to_string()))
            .await
            .unwrap();
        storage
            .put(&ctx, "key3", JsonValue::String("value3".to_string()))
            .await
            .unwrap();

        let removed = storage.remove(&ctx, "key2").await.unwrap();
        assert_eq!(removed, Some(JsonValue::String("value2".to_string())));

        assert_eq!(
            storage.get(&ctx, "key1").await.unwrap(),
            Some(JsonValue::String("value1".to_string()))
        );
        assert_eq!(
            storage.get(&ctx, "key3").await.unwrap(),
            Some(JsonValue::String("value3".to_string()))
        );
        assert_eq!(storage.get(&ctx, "key2").await.unwrap(), None);
    }

    #[tokio::test]
    async fn test_remove_already_removed_key() {
        let (storage, ctx) = create_in_memory_storage().await;

        storage
            .put(&ctx, "key", JsonValue::String("value".to_string()))
            .await
            .unwrap();
        storage.remove(&ctx, "key").await.unwrap();

        let removed = storage.remove(&ctx, "key").await.unwrap();
        assert_eq!(removed, None);
    }

    #[tokio::test]
    async fn test_remove_after_overwrite() {
        let (storage, ctx) = create_in_memory_storage().await;

        storage
            .put(&ctx, "key", JsonValue::String("initial".to_string()))
            .await
            .unwrap();

        let new_value = JsonValue::Number(42.into());
        storage.put(&ctx, "key", new_value.clone()).await.unwrap();

        let removed = storage.remove(&ctx, "key").await.unwrap();
        assert_eq!(removed, Some(new_value));
    }

    #[tokio::test]
    async fn test_remove_concurrent_access() {
        let (storage, ctx) = create_in_memory_storage().await;

        for i in 0..50 {
            storage
                .put(&ctx, &format!("key_{}", i), JsonValue::Number(i.into()))
                .await
                .unwrap();
        }

        let mut handles = vec![];
        for i in 0..50 {
            let storage_clone = storage.clone();
            let handle = {
                let ctx = ctx.clone();
                tokio::spawn(async move {
                    storage_clone
                        .remove(&ctx, &format!("key_{}", i))
                        .await
                        .unwrap()
                })
            };
            handles.push(handle);
        }

        for (i, handle) in handles.into_iter().enumerate() {
            let result = handle.await.unwrap();
            assert_eq!(result, Some(JsonValue::Number(i.into())));
        }

        for i in 0..50 {
            assert_eq!(
                storage.get(&ctx, &format!("key_{}", i)).await.unwrap(),
                None
            );
        }
    }

    // Put batch tests
    #[tokio::test]
    async fn test_put_batch_happy_path() {
        let (storage, ctx) = create_in_memory_storage().await;

        let items = vec![
            ("key1", JsonValue::String("value1".to_string())),
            ("key2", JsonValue::Number(2.into())),
            ("key3", JsonValue::Bool(true)),
        ];

        storage.put_batch(&ctx, &items).await.unwrap();

        assert_eq!(
            storage.get(&ctx, "key1").await.unwrap(),
            Some(JsonValue::String("value1".to_string()))
        );
        assert_eq!(
            storage.get(&ctx, "key2").await.unwrap(),
            Some(JsonValue::Number(2.into()))
        );
        assert_eq!(
            storage.get(&ctx, "key3").await.unwrap(),
            Some(JsonValue::Bool(true))
        );
    }

    #[tokio::test]
    async fn test_put_batch_empty() {
        let (storage, ctx) = create_in_memory_storage().await;

        let items: Vec<(&str, JsonValue)> = vec![];
        storage.put_batch(&ctx, &items).await.unwrap();
    }

    #[tokio::test]
    async fn test_put_batch_overwrite_existing() {
        let (storage, ctx) = create_in_memory_storage().await;

        storage
            .put(&ctx, "key1", JsonValue::String("initial1".to_string()))
            .await
            .unwrap();
        storage
            .put(&ctx, "key2", JsonValue::String("initial2".to_string()))
            .await
            .unwrap();

        let items = vec![
            ("key1", JsonValue::String("new1".to_string())),
            ("key2", JsonValue::String("new2".to_string())),
            ("key3", JsonValue::String("new3".to_string())),
        ];

        storage.put_batch(&ctx, &items).await.unwrap();

        assert_eq!(
            storage.get(&ctx, "key1").await.unwrap(),
            Some(JsonValue::String("new1".to_string()))
        );
        assert_eq!(
            storage.get(&ctx, "key2").await.unwrap(),
            Some(JsonValue::String("new2".to_string()))
        );
        assert_eq!(
            storage.get(&ctx, "key3").await.unwrap(),
            Some(JsonValue::String("new3".to_string()))
        );
    }

    #[tokio::test]
    async fn test_put_batch_mixed_update_and_insert() {
        let (storage, ctx) = create_in_memory_storage().await;

        storage
            .put(&ctx, "existing1", JsonValue::String("old1".to_string()))
            .await
            .unwrap();
        storage
            .put(&ctx, "existing2", JsonValue::String("old2".to_string()))
            .await
            .unwrap();

        let items = vec![
            ("existing1", JsonValue::String("updated1".to_string())),
            ("existing2", JsonValue::String("updated2".to_string())),
            ("new1", JsonValue::String("new_value1".to_string())),
            ("new2", JsonValue::Number(42.into())),
        ];

        storage.put_batch(&ctx, &items).await.unwrap();

        assert_eq!(
            storage.get(&ctx, "existing1").await.unwrap(),
            Some(JsonValue::String("updated1".to_string()))
        );
        assert_eq!(
            storage.get(&ctx, "existing2").await.unwrap(),
            Some(JsonValue::String("updated2".to_string()))
        );
        assert_eq!(
            storage.get(&ctx, "new1").await.unwrap(),
            Some(JsonValue::String("new_value1".to_string()))
        );
        assert_eq!(
            storage.get(&ctx, "new2").await.unwrap(),
            Some(JsonValue::Number(42.into()))
        );
    }

    #[tokio::test]
    async fn test_put_batch_large_batch() {
        let (storage, ctx) = create_in_memory_storage().await;

        let mut key_strings = Vec::new();
        let mut items = Vec::new();
        for i in 0..1000 {
            let key = format!("key_{}", i);
            key_strings.push(key);
        }
        for (i, key) in key_strings.iter().enumerate() {
            items.push((key.as_str(), JsonValue::Number(i.into())));
        }

        storage.put_batch(&ctx, &items).await.unwrap();

        assert_eq!(
            storage.get(&ctx, "key_0").await.unwrap(),
            Some(JsonValue::Number(0.into()))
        );
        assert_eq!(
            storage.get(&ctx, "key_500").await.unwrap(),
            Some(JsonValue::Number(500.into()))
        );
        assert_eq!(
            storage.get(&ctx, "key_999").await.unwrap(),
            Some(JsonValue::Number(999.into()))
        );
    }

    #[tokio::test]
    async fn test_put_batch_complex_values() {
        let (storage, ctx) = create_in_memory_storage().await;

        let items = vec![
            (
                "complex1",
                serde_json::json!({
                    "nested": {
                        "array": [1, 2, 3],
                        "object": {"key": "value"}
                    }
                }),
            ),
            (
                "complex2",
                JsonValue::Array(vec![
                    JsonValue::String("a".to_string()),
                    JsonValue::Number(1.into()),
                    JsonValue::Bool(true),
                ]),
            ),
        ];

        storage.put_batch(&ctx, &items).await.unwrap();

        let retrieved1 = storage.get(&ctx, "complex1").await.unwrap();
        assert!(retrieved1.is_some());
        assert_eq!(retrieved1, Some(items[0].1.clone()));

        let retrieved2 = storage.get(&ctx, "complex2").await.unwrap();
        assert_eq!(retrieved2, Some(items[1].1.clone()));
    }

    #[tokio::test]
    async fn test_put_batch_duplicate_keys_in_batch() {
        let (storage, ctx) = create_in_memory_storage().await;

        let items = vec![
            ("duplicate_key", JsonValue::String("first".to_string())),
            ("duplicate_key", JsonValue::String("second".to_string())),
            ("duplicate_key", JsonValue::String("third".to_string())),
        ];

        storage.put_batch(&ctx, &items).await.unwrap();

        assert_eq!(
            storage.get(&ctx, "duplicate_key").await.unwrap(),
            Some(JsonValue::String("third".to_string()))
        );
    }

    #[tokio::test]
    async fn test_put_batch_special_characters_in_keys() {
        let (storage, ctx) = create_in_memory_storage().await;

        let items = vec![
            ("key.with.dots", JsonValue::String("dots".to_string())),
            ("key-with-dashes", JsonValue::String("dashes".to_string())),
            (
                "key_with_underscores",
                JsonValue::String("underscores".to_string()),
            ),
            ("key/with/slashes", JsonValue::String("slashes".to_string())),
            ("key@with@symbols", JsonValue::String("symbols".to_string())),
        ];

        storage.put_batch(&ctx, &items).await.unwrap();

        for (key, expected_value) in &items {
            assert_eq!(
                storage.get(&ctx, key).await.unwrap(),
                Some(expected_value.clone())
            );
        }
    }

    #[tokio::test]
    async fn test_put_batch_null_values() {
        let (storage, ctx) = create_in_memory_storage().await;

        let items = vec![
            ("null_key1", JsonValue::Null),
            ("null_key2", JsonValue::Null),
        ];

        storage.put_batch(&ctx, &items).await.unwrap();

        assert_eq!(
            storage.get(&ctx, "null_key1").await.unwrap(),
            Some(JsonValue::Null)
        );
        assert_eq!(
            storage.get(&ctx, "null_key2").await.unwrap(),
            Some(JsonValue::Null)
        );
    }

    // Get batch tests
    #[tokio::test]
    async fn test_get_batch_happy_path() {
        let (storage, ctx) = create_in_memory_storage().await;

        storage
            .put(&ctx, "key1", JsonValue::String("value1".to_string()))
            .await
            .unwrap();
        storage
            .put(&ctx, "key2", JsonValue::Number(2.into()))
            .await
            .unwrap();
        storage
            .put(&ctx, "key3", JsonValue::Bool(true))
            .await
            .unwrap();

        let keys = vec!["key1", "key2", "key3"];
        let results = storage.get_batch(&ctx, &keys).await.unwrap();

        assert_eq!(results.len(), 3);
        assert_eq!(
            results[0],
            (
                "key1".to_string(),
                Some(JsonValue::String("value1".to_string()))
            )
        );
        assert_eq!(
            results[1],
            ("key2".to_string(), Some(JsonValue::Number(2.into())))
        );
        assert_eq!(
            results[2],
            ("key3".to_string(), Some(JsonValue::Bool(true)))
        );
    }

    #[tokio::test]
    async fn test_get_batch_empty() {
        let (storage, ctx) = create_in_memory_storage().await;

        let keys: Vec<&str> = vec![];
        let results = storage.get_batch(&ctx, &keys).await.unwrap();

        assert_eq!(results.len(), 0);
    }

    #[tokio::test]
    async fn test_get_batch_with_missing_keys() {
        let (storage, ctx) = create_in_memory_storage().await;

        storage
            .put(&ctx, "key1", JsonValue::String("value1".to_string()))
            .await
            .unwrap();
        storage
            .put(&ctx, "key3", JsonValue::String("value3".to_string()))
            .await
            .unwrap();

        let keys = vec!["key1", "key2", "key3", "key4"];
        let results = storage.get_batch(&ctx, &keys).await.unwrap();

        assert_eq!(results.len(), 4);
        assert_eq!(
            results[0],
            (
                "key1".to_string(),
                Some(JsonValue::String("value1".to_string()))
            )
        );
        assert_eq!(results[1], ("key2".to_string(), None));
        assert_eq!(
            results[2],
            (
                "key3".to_string(),
                Some(JsonValue::String("value3".to_string()))
            )
        );
        assert_eq!(results[3], ("key4".to_string(), None));
    }

    #[tokio::test]
    async fn test_get_batch_all_missing() {
        let (storage, ctx) = create_in_memory_storage().await;

        let keys = vec!["nonexistent1", "nonexistent2", "nonexistent3"];
        let results = storage.get_batch(&ctx, &keys).await.unwrap();

        assert_eq!(results.len(), 3);
        assert_eq!(results[0], ("nonexistent1".to_string(), None));
        assert_eq!(results[1], ("nonexistent2".to_string(), None));
        assert_eq!(results[2], ("nonexistent3".to_string(), None));
    }

    #[tokio::test]
    async fn test_get_batch_preserves_order() {
        let (storage, ctx) = create_in_memory_storage().await;

        storage
            .put(&ctx, "key3", JsonValue::String("value3".to_string()))
            .await
            .unwrap();
        storage
            .put(&ctx, "key1", JsonValue::String("value1".to_string()))
            .await
            .unwrap();
        storage
            .put(&ctx, "key2", JsonValue::String("value2".to_string()))
            .await
            .unwrap();

        let keys = vec!["key1", "key2", "key3"];
        let results = storage.get_batch(&ctx, &keys).await.unwrap();

        assert_eq!(results.len(), 3);
        assert_eq!(results[0].0, "key1");
        assert_eq!(results[1].0, "key2");
        assert_eq!(results[2].0, "key3");
    }

    #[tokio::test]
    async fn test_get_batch_large_batch() {
        let (storage, ctx) = create_in_memory_storage().await;

        for i in 0..1000 {
            storage
                .put(&ctx, &format!("key_{}", i), JsonValue::Number(i.into()))
                .await
                .unwrap();
        }

        let key_strings: Vec<String> = (0..1000).map(|i| format!("key_{}", i)).collect();
        let keys: Vec<&str> = key_strings.iter().map(|s| s.as_str()).collect();
        let results = storage.get_batch(&ctx, &keys).await.unwrap();

        assert_eq!(results.len(), 1000);
        for (i, (key, value)) in results.iter().enumerate() {
            assert_eq!(key, &key_strings[i]);
            assert_eq!(value, &Some(JsonValue::Number(i.into())));
        }
    }

    #[tokio::test]
    async fn test_get_batch_complex_values() {
        let (storage, ctx) = create_in_memory_storage().await;

        let complex1 = serde_json::json!({
            "nested": {
                "array": [1, 2, 3],
                "object": {"key": "value"}
            }
        });
        let complex2 = JsonValue::Array(vec![
            JsonValue::String("a".to_string()),
            JsonValue::Number(1.into()),
        ]);

        storage
            .put(&ctx, "complex1", complex1.clone())
            .await
            .unwrap();
        storage
            .put(&ctx, "complex2", complex2.clone())
            .await
            .unwrap();

        let keys = vec!["complex1", "complex2"];
        let results = storage.get_batch(&ctx, &keys).await.unwrap();

        assert_eq!(results.len(), 2);
        assert_eq!(results[0], ("complex1".to_string(), Some(complex1)));
        assert_eq!(results[1], ("complex2".to_string(), Some(complex2)));
    }

    #[tokio::test]
    async fn test_get_batch_duplicate_keys() {
        let (storage, ctx) = create_in_memory_storage().await;

        storage
            .put(
                &ctx,
                "duplicate_key",
                JsonValue::String("value".to_string()),
            )
            .await
            .unwrap();

        let keys = vec!["duplicate_key", "duplicate_key", "duplicate_key"];
        let results = storage.get_batch(&ctx, &keys).await.unwrap();

        assert_eq!(results.len(), 3);
        for result in &results {
            assert_eq!(
                result,
                &(
                    "duplicate_key".to_string(),
                    Some(JsonValue::String("value".to_string()))
                )
            );
        }
    }

    #[tokio::test]
    async fn test_get_batch_cache_consistency() {
        let (storage, ctx) = create_in_memory_storage().await;

        storage
            .put(&ctx, "key1", JsonValue::String("value1".to_string()))
            .await
            .unwrap();
        storage
            .put(&ctx, "key2", JsonValue::String("value2".to_string()))
            .await
            .unwrap();

        let keys = vec!["key1", "key2"];
        let first_results = storage.get_batch(&ctx, &keys).await.unwrap();
        let second_results = storage.get_batch(&ctx, &keys).await.unwrap();

        assert_eq!(first_results, second_results);
    }

    #[tokio::test]
    async fn test_get_batch_mixed_cache_hit_and_miss() {
        let (storage, ctx) = create_in_memory_storage().await;

        storage
            .put(&ctx, "key1", JsonValue::String("value1".to_string()))
            .await
            .unwrap();
        storage
            .put(&ctx, "key2", JsonValue::String("value2".to_string()))
            .await
            .unwrap();
        storage
            .put(&ctx, "key3", JsonValue::String("value3".to_string()))
            .await
            .unwrap();

        let _ = storage.get(&ctx, "key1").await.unwrap();

        let keys = vec!["key1", "key2", "key3"];
        let results = storage.get_batch(&ctx, &keys).await.unwrap();

        assert_eq!(results.len(), 3);
        assert_eq!(
            results[0],
            (
                "key1".to_string(),
                Some(JsonValue::String("value1".to_string()))
            )
        );
        assert_eq!(
            results[1],
            (
                "key2".to_string(),
                Some(JsonValue::String("value2".to_string()))
            )
        );
        assert_eq!(
            results[2],
            (
                "key3".to_string(),
                Some(JsonValue::String("value3".to_string()))
            )
        );
    }

    #[tokio::test]
    async fn test_get_batch_special_characters_in_keys() {
        let (storage, ctx) = create_in_memory_storage().await;

        let special_keys = vec![
            ("key.with.dots", JsonValue::String("dots".to_string())),
            ("key-with-dashes", JsonValue::String("dashes".to_string())),
            (
                "key_with_underscores",
                JsonValue::String("underscores".to_string()),
            ),
        ];

        for (key, value) in &special_keys {
            storage.put(&ctx, key, value.clone()).await.unwrap();
        }

        let keys: Vec<&str> = special_keys.iter().map(|(k, _)| *k).collect();
        let results = storage.get_batch(&ctx, &keys).await.unwrap();

        assert_eq!(results.len(), 3);
        for (i, (_, value)) in results.iter().enumerate() {
            assert_eq!(value, &Some(special_keys[i].1.clone()));
        }
    }

    // Remove batch tests
    #[tokio::test]
    async fn test_remove_batch_happy_path() {
        let (storage, ctx) = create_in_memory_storage().await;

        storage
            .put(&ctx, "key1", JsonValue::String("value1".to_string()))
            .await
            .unwrap();
        storage
            .put(&ctx, "key2", JsonValue::Number(2.into()))
            .await
            .unwrap();
        storage
            .put(&ctx, "key3", JsonValue::Bool(true))
            .await
            .unwrap();

        let keys = vec!["key1", "key2", "key3"];
        let results = storage.remove_batch(&ctx, &keys).await.unwrap();

        assert_eq!(results.len(), 3);
        assert_eq!(
            results[0],
            (
                "key1".to_string(),
                Some(JsonValue::String("value1".to_string()))
            )
        );
        assert_eq!(
            results[1],
            ("key2".to_string(), Some(JsonValue::Number(2.into())))
        );
        assert_eq!(
            results[2],
            ("key3".to_string(), Some(JsonValue::Bool(true)))
        );

        assert_eq!(storage.get(&ctx, "key1").await.unwrap(), None);
        assert_eq!(storage.get(&ctx, "key2").await.unwrap(), None);
        assert_eq!(storage.get(&ctx, "key3").await.unwrap(), None);
    }

    #[tokio::test]
    async fn test_remove_batch_empty() {
        let (storage, ctx) = create_in_memory_storage().await;

        let keys: Vec<&str> = vec![];
        let results = storage.remove_batch(&ctx, &keys).await.unwrap();

        assert_eq!(results.len(), 0);
    }

    #[tokio::test]
    async fn test_remove_batch_with_missing_keys() {
        let (storage, ctx) = create_in_memory_storage().await;

        storage
            .put(&ctx, "key1", JsonValue::String("value1".to_string()))
            .await
            .unwrap();
        storage
            .put(&ctx, "key3", JsonValue::String("value3".to_string()))
            .await
            .unwrap();

        let keys = vec!["key1", "key2", "key3", "key4"];
        let results = storage.remove_batch(&ctx, &keys).await.unwrap();

        assert_eq!(results.len(), 4);
        assert_eq!(
            results[0],
            (
                "key1".to_string(),
                Some(JsonValue::String("value1".to_string()))
            )
        );
        assert_eq!(results[1], ("key2".to_string(), None));
        assert_eq!(
            results[2],
            (
                "key3".to_string(),
                Some(JsonValue::String("value3".to_string()))
            )
        );
        assert_eq!(results[3], ("key4".to_string(), None));

        assert_eq!(storage.get(&ctx, "key1").await.unwrap(), None);
        assert_eq!(storage.get(&ctx, "key3").await.unwrap(), None);
    }

    #[tokio::test]
    async fn test_remove_batch_all_missing() {
        let (storage, ctx) = create_in_memory_storage().await;

        let keys = vec!["nonexistent1", "nonexistent2", "nonexistent3"];
        let results = storage.remove_batch(&ctx, &keys).await.unwrap();

        assert_eq!(results.len(), 3);
        assert_eq!(results[0], ("nonexistent1".to_string(), None));
        assert_eq!(results[1], ("nonexistent2".to_string(), None));
        assert_eq!(results[2], ("nonexistent3".to_string(), None));
    }

    #[tokio::test]
    async fn test_remove_batch_preserves_order() {
        let (storage, ctx) = create_in_memory_storage().await;

        storage
            .put(&ctx, "key3", JsonValue::String("value3".to_string()))
            .await
            .unwrap();
        storage
            .put(&ctx, "key1", JsonValue::String("value1".to_string()))
            .await
            .unwrap();
        storage
            .put(&ctx, "key2", JsonValue::String("value2".to_string()))
            .await
            .unwrap();

        let keys = vec!["key1", "key2", "key3"];
        let results = storage.remove_batch(&ctx, &keys).await.unwrap();

        assert_eq!(results.len(), 3);
        assert_eq!(results[0].0, "key1");
        assert_eq!(results[1].0, "key2");
        assert_eq!(results[2].0, "key3");
    }

    #[tokio::test]
    async fn test_remove_batch_large_batch() {
        let (storage, ctx) = create_in_memory_storage().await;

        for i in 0..1000 {
            storage
                .put(&ctx, &format!("key_{}", i), JsonValue::Number(i.into()))
                .await
                .unwrap();
        }

        let key_strings: Vec<String> = (0..1000).map(|i| format!("key_{}", i)).collect();
        let keys: Vec<&str> = key_strings.iter().map(|s| s.as_str()).collect();
        let results = storage.remove_batch(&ctx, &keys).await.unwrap();

        assert_eq!(results.len(), 1000);
        for (i, (key, value)) in results.iter().enumerate() {
            assert_eq!(key, &key_strings[i]);
            assert_eq!(value, &Some(JsonValue::Number(i.into())));
        }

        for i in 0..1000 {
            assert_eq!(
                storage.get(&ctx, &format!("key_{}", i)).await.unwrap(),
                None
            );
        }
    }

    #[tokio::test]
    async fn test_remove_batch_partial_removal() {
        let (storage, ctx) = create_in_memory_storage().await;

        storage
            .put(&ctx, "key1", JsonValue::String("value1".to_string()))
            .await
            .unwrap();
        storage
            .put(&ctx, "key2", JsonValue::String("value2".to_string()))
            .await
            .unwrap();
        storage
            .put(&ctx, "key3", JsonValue::String("value3".to_string()))
            .await
            .unwrap();
        storage
            .put(&ctx, "key4", JsonValue::String("value4".to_string()))
            .await
            .unwrap();

        let keys = vec!["key1", "key3"];
        storage.remove_batch(&ctx, &keys).await.unwrap();

        assert_eq!(storage.get(&ctx, "key1").await.unwrap(), None);
        assert_eq!(storage.get(&ctx, "key3").await.unwrap(), None);

        assert_eq!(
            storage.get(&ctx, "key2").await.unwrap(),
            Some(JsonValue::String("value2".to_string()))
        );
        assert_eq!(
            storage.get(&ctx, "key4").await.unwrap(),
            Some(JsonValue::String("value4".to_string()))
        );
    }

    #[tokio::test]
    async fn test_remove_batch_complex_values() {
        let (storage, ctx) = create_in_memory_storage().await;

        let complex1 = serde_json::json!({
            "nested": {
                "array": [1, 2, 3],
                "object": {"key": "value"}
            }
        });
        let complex2 = JsonValue::Array(vec![
            JsonValue::String("a".to_string()),
            JsonValue::Number(1.into()),
        ]);

        storage
            .put(&ctx, "complex1", complex1.clone())
            .await
            .unwrap();
        storage
            .put(&ctx, "complex2", complex2.clone())
            .await
            .unwrap();

        let keys = vec!["complex1", "complex2"];
        let results = storage.remove_batch(&ctx, &keys).await.unwrap();

        assert_eq!(results.len(), 2);
        assert_eq!(results[0], ("complex1".to_string(), Some(complex1)));
        assert_eq!(results[1], ("complex2".to_string(), Some(complex2)));
    }

    #[tokio::test]
    async fn test_remove_batch_duplicate_keys() {
        let (storage, ctx) = create_in_memory_storage().await;

        storage
            .put(
                &ctx,
                "duplicate_key",
                JsonValue::String("value".to_string()),
            )
            .await
            .unwrap();

        let keys = vec!["duplicate_key", "duplicate_key", "duplicate_key"];
        let results = storage.remove_batch(&ctx, &keys).await.unwrap();

        assert_eq!(results.len(), 3);
        assert_eq!(
            results[0],
            (
                "duplicate_key".to_string(),
                Some(JsonValue::String("value".to_string()))
            )
        );
        assert_eq!(results[1], ("duplicate_key".to_string(), None));
        assert_eq!(results[2], ("duplicate_key".to_string(), None));

        assert_eq!(storage.get(&ctx, "duplicate_key").await.unwrap(), None);
    }

    #[tokio::test]
    async fn test_remove_batch_cache_consistency() {
        let (storage, ctx) = create_in_memory_storage().await;

        storage
            .put(&ctx, "key1", JsonValue::String("value1".to_string()))
            .await
            .unwrap();
        storage
            .put(&ctx, "key2", JsonValue::String("value2".to_string()))
            .await
            .unwrap();

        let _ = storage.get(&ctx, "key1").await.unwrap();
        let _ = storage.get(&ctx, "key2").await.unwrap();

        let keys = vec!["key1", "key2"];
        storage.remove_batch(&ctx, &keys).await.unwrap();

        assert_eq!(storage.get(&ctx, "key1").await.unwrap(), None);
        assert_eq!(storage.get(&ctx, "key2").await.unwrap(), None);
    }

    #[tokio::test]
    async fn test_remove_batch_transaction_atomicity() {
        let (storage, ctx) = create_in_memory_storage().await;

        storage
            .put(&ctx, "key1", JsonValue::String("value1".to_string()))
            .await
            .unwrap();
        storage
            .put(&ctx, "key2", JsonValue::String("value2".to_string()))
            .await
            .unwrap();
        storage
            .put(&ctx, "key3", JsonValue::String("value3".to_string()))
            .await
            .unwrap();

        let keys = vec!["key1", "key2", "key3"];
        storage.remove_batch(&ctx, &keys).await.unwrap();

        assert_eq!(storage.get(&ctx, "key1").await.unwrap(), None);
        assert_eq!(storage.get(&ctx, "key2").await.unwrap(), None);
        assert_eq!(storage.get(&ctx, "key3").await.unwrap(), None);
    }

    #[tokio::test]
    async fn test_remove_batch_special_characters_in_keys() {
        let (storage, ctx) = create_in_memory_storage().await;

        let special_keys = vec![
            ("key.with.dots", JsonValue::String("dots".to_string())),
            ("key-with-dashes", JsonValue::String("dashes".to_string())),
            (
                "key_with_underscores",
                JsonValue::String("underscores".to_string()),
            ),
        ];

        for (key, value) in &special_keys {
            storage.put(&ctx, key, value.clone()).await.unwrap();
        }

        let keys: Vec<&str> = special_keys.iter().map(|(k, _)| *k).collect();
        let results = storage.remove_batch(&ctx, &keys).await.unwrap();

        assert_eq!(results.len(), 3);
        for (i, (_, value)) in results.iter().enumerate() {
            assert_eq!(value, &Some(special_keys[i].1.clone()));
        }

        for (key, _) in &special_keys {
            assert_eq!(storage.get(&ctx, key).await.unwrap(), None);
        }
    }

    #[tokio::test]
    async fn test_remove_batch_after_put_batch() {
        let (storage, ctx) = create_in_memory_storage().await;

        let put_items = vec![
            ("key1", JsonValue::String("value1".to_string())),
            ("key2", JsonValue::Number(2.into())),
            ("key3", JsonValue::Bool(true)),
        ];
        storage.put_batch(&ctx, &put_items).await.unwrap();

        let keys = vec!["key1", "key2", "key3"];
        let results = storage.remove_batch(&ctx, &keys).await.unwrap();

        assert_eq!(results.len(), 3);
        assert_eq!(
            results[0],
            (
                "key1".to_string(),
                Some(JsonValue::String("value1".to_string()))
            )
        );
        assert_eq!(
            results[1],
            ("key2".to_string(), Some(JsonValue::Number(2.into())))
        );
        assert_eq!(
            results[2],
            ("key3".to_string(), Some(JsonValue::Bool(true)))
        );
    }

    #[tokio::test]
    async fn test_get_batch_by_prefix_happy_path() {
        let (storage, ctx) = create_in_memory_storage().await;

        storage
            .put(&ctx, "key1", JsonValue::String("value1".to_string()))
            .await
            .unwrap();
        storage
            .put(&ctx, "key2", JsonValue::Number(2.into()))
            .await
            .unwrap();
        storage
            .put(&ctx, "key3", JsonValue::Bool(true))
            .await
            .unwrap();

        let prefix = "key";

        let results = storage.get_batch_by_prefix(&ctx, prefix).await.unwrap();

        let result_map: HashMap<String, JsonValue> = results.into_iter().collect();
        assert_eq!(result_map.len(), 3);
        assert_eq!(result_map["key1"], JsonValue::String("value1".to_string()));
        assert_eq!(result_map["key2"], JsonValue::Number(2.into()));
        assert_eq!(result_map["key3"], JsonValue::Bool(true));
    }

    // This will return all records
    #[tokio::test]
    async fn test_get_batch_by_prefix_empty_prefix() {
        let (storage, ctx) = create_in_memory_storage().await;

        storage
            .put(&ctx, "A", JsonValue::String("A".to_string()))
            .await
            .unwrap();
        storage
            .put(&ctx, "B", JsonValue::String("B".to_string()))
            .await
            .unwrap();

        let results = storage.get_batch_by_prefix(&ctx, "").await.unwrap();

        let result_map: HashMap<String, JsonValue> = results.into_iter().collect();
        assert_eq!(result_map.len(), 2);
        assert_eq!(result_map["A"], JsonValue::String("A".to_string()));
        assert_eq!(result_map["B"], JsonValue::String("B".to_string()));
    }

    #[tokio::test]
    async fn test_get_batch_by_prefix_no_match() {
        let (storage, ctx) = create_in_memory_storage().await;

        storage
            .put(&ctx, "A", JsonValue::String("A".to_string()))
            .await
            .unwrap();
        storage
            .put(&ctx, "B", JsonValue::String("B".to_string()))
            .await
            .unwrap();

        let prefix = "prefix";

        let results = storage.get_batch_by_prefix(&ctx, prefix).await.unwrap();
        assert!(results.is_empty());
    }

    #[tokio::test]
    async fn test_get_batch_by_prefix_large_batch() {
        let (storage, ctx) = create_in_memory_storage().await;

        let prefix = "key_";
        for i in 0..1000 {
            storage
                .put(&ctx, &format!("{prefix}{i}"), JsonValue::Number(i.into()))
                .await
                .unwrap();
        }

        let results = storage.get_batch_by_prefix(&ctx, prefix).await.unwrap();
        let result_map: HashMap<String, JsonValue> = results.into_iter().collect();
        assert_eq!(result_map.len(), 1000);
        for i in 0..1000 {
            assert_eq!(
                result_map[&format!("{prefix}{i}")],
                JsonValue::Number(i.into())
            );
        }
    }

    #[tokio::test]
    async fn test_remove_batch_by_prefix_happy_path() {
        let (storage, ctx) = create_in_memory_storage().await;

        let prefix = "key";
        storage
            .put(&ctx, "key1", JsonValue::String("value1".to_string()))
            .await
            .unwrap();
        storage
            .put(&ctx, "key2", JsonValue::Number(2.into()))
            .await
            .unwrap();
        storage
            .put(&ctx, "key3", JsonValue::Bool(true))
            .await
            .unwrap();

        let results = storage.remove_batch_by_prefix(&ctx, prefix).await.unwrap();
        let result_map: HashMap<String, JsonValue> = results.into_iter().collect();

        assert_eq!(result_map.len(), 3);
        assert_eq!(result_map["key1"], JsonValue::String("value1".to_string()));
        assert_eq!(result_map["key2"], JsonValue::Number(2.into()));
        assert_eq!(result_map["key3"], JsonValue::Bool(true));

        assert_eq!(
            storage
                .get_batch_by_prefix(&ctx, prefix)
                .await
                .unwrap()
                .len(),
            0
        );
    }
    // This will remove all records
    #[tokio::test]
    async fn test_remove_batch_by_empty_prefix() {
        let (storage, ctx) = create_in_memory_storage().await;

        let prefix = "";

        storage
            .put(&ctx, "A", JsonValue::String("A".to_string()))
            .await
            .unwrap();
        storage
            .put(&ctx, "B", JsonValue::String("B".to_string()))
            .await
            .unwrap();
        let results = storage.remove_batch_by_prefix(&ctx, prefix).await.unwrap();
        let result_map: HashMap<String, JsonValue> = results.into_iter().collect();
        assert_eq!(result_map.len(), 2);
        assert_eq!(result_map["A"], JsonValue::String("A".to_string()));
        assert_eq!(result_map["B"], JsonValue::String("B".to_string()));

        assert!(
            storage
                .get_batch_by_prefix(&ctx, prefix)
                .await
                .unwrap()
                .is_empty()
        );
    }

    #[tokio::test]
    async fn test_remove_batch_by_prefix_no_match() {
        let (storage, ctx) = create_in_memory_storage().await;
        let prefix = "nonexistent";

        let results = storage.remove_batch_by_prefix(&ctx, prefix).await.unwrap();
        assert!(results.is_empty());
    }

    #[tokio::test]
    async fn test_remove_batch_by_prefix_large_batch() {
        let (storage, ctx) = create_in_memory_storage().await;

        let prefix = "key_";
        for i in 0..1000 {
            storage
                .put(&ctx, &format!("{prefix}{i}"), JsonValue::Number(i.into()))
                .await
                .unwrap();
        }

        let results = storage.remove_batch_by_prefix(&ctx, prefix).await.unwrap();
        let result_map: HashMap<String, JsonValue> = results.into_iter().collect();
        for i in 0..1000 {
            assert_eq!(
                result_map[&format!("{prefix}{i}")],
                JsonValue::Number(i.into())
            );
        }

        assert!(
            storage
                .get_batch_by_prefix(&ctx, prefix)
                .await
                .unwrap()
                .is_empty()
        );
    }

    #[tokio::test]
    async fn test_remove_batch_by_prefix_complex_values() {
        let (storage, ctx) = create_in_memory_storage().await;

        let prefix = "complex";

        let complex1 = serde_json::json!({
            "nested": {
                "array": [1, 2, 3],
                "object": {"key": "value"}
            }
        });
        let complex2 = JsonValue::Array(vec![
            JsonValue::String("a".to_string()),
            JsonValue::Number(1.into()),
        ]);

        storage
            .put(&ctx, "complex1", complex1.clone())
            .await
            .unwrap();
        storage
            .put(&ctx, "complex2", complex2.clone())
            .await
            .unwrap();

        let results = storage.remove_batch_by_prefix(&ctx, prefix).await.unwrap();
        let result_map: HashMap<String, JsonValue> = results.into_iter().collect();
        assert_eq!(result_map.len(), 2);
        assert_eq!(result_map["complex1"], complex1);
        assert_eq!(result_map["complex2"], complex2);
    }

    #[tokio::test]
    async fn test_remove_batch_by_prefix_special_characters_in_keys() {
        let (storage, ctx) = create_in_memory_storage().await;

        let special_keys = vec![
            ("key.with.dots", JsonValue::String("dots".to_string())),
            ("key-with-dashes", JsonValue::String("dashes".to_string())),
            (
                "key_with_underscores",
                JsonValue::String("underscores".to_string()),
            ),
        ];

        for (key, value) in special_keys.iter() {
            storage.put(&ctx, *key, value.clone()).await.unwrap();
        }
        let prefix = "key";
        let results = storage.remove_batch_by_prefix(&ctx, prefix).await.unwrap();
        let result_map: HashMap<String, JsonValue> = results.into_iter().collect();

        assert_eq!(result_map.len(), 3);
        for (key, value) in special_keys {
            assert_eq!(result_map[key], value);
        }
        assert!(
            storage
                .get_batch_by_prefix(&ctx, prefix)
                .await
                .unwrap()
                .is_empty()
        );
    }
}
