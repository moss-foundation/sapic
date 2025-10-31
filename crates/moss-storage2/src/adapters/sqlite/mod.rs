use async_trait::async_trait;
use joinerror::{Error, ResultExt};
use moss_logging::session;
use serde_json::Value as JsonValue;
use sqlx::{
    Row, SqlitePool,
    sqlite::{SqliteConnectOptions, SqliteJournalMode},
};
use std::{collections::HashMap, path::Path, str::FromStr, sync::Arc, time::Duration};
use tokio::sync::RwLock;

use crate::adapters::{Flushable, KeyedStorage, Optimizable};

const DEFAULT_BUSY_TIMEOUT: Duration = Duration::from_secs(5);

pub struct SqliteStorageOptions {
    busy_timeout: Duration,
}

pub struct SqliteStorage {
    // INFO: This will be improved in the future to support glob search for keys.
    cache: RwLock<HashMap<String, JsonValue>>,
    pool: SqlitePool,
}

impl SqliteStorage {
    pub async fn new(
        path: impl AsRef<Path>,
        options: Option<SqliteStorageOptions>,
    ) -> joinerror::Result<Arc<Self>> {
        // TODO: Backup the database file before creating a new one
        // if path.as_ref().exists() {
        //     let _ = std::fs::copy(&path.as_ref(), path.as_ref().with_extension("bak"));
        // }

        let url = format!("sqlite://{}", path.as_ref().display());
        let options = SqliteConnectOptions::from_str(&url)
            .map_err(|e| Error::new::<()>(e.to_string()))?
            .create_if_missing(true)
            .journal_mode(SqliteJournalMode::Wal)
            .synchronous(sqlx::sqlite::SqliteSynchronous::Normal)
            .busy_timeout(
                options
                    .map(|o| o.busy_timeout)
                    .unwrap_or(DEFAULT_BUSY_TIMEOUT),
            )
            .foreign_keys(true);

        let pool = SqlitePool::connect_with(options)
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
    async fn put(&self, key: &str, value: JsonValue) -> joinerror::Result<()> {
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
    }

    async fn get(&self, key: &str) -> joinerror::Result<Option<JsonValue>> {
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
    }

    async fn remove(&self, key: &str) -> joinerror::Result<()> {
        sqlx::query("DELETE FROM kv WHERE key = ?")
            .bind(key)
            .execute(&self.pool)
            .await
            .join_err::<()>("failed to delete value")?;

        self.cache.write().await.remove(key);

        Ok(())
    }

    async fn put_batch(&self, keys: &[&str], values: &[JsonValue]) -> joinerror::Result<()> {
        let mut tx = self
            .pool
            .begin()
            .await
            .join_err::<()>("failed to begin transaction")?;

        for (key, value) in keys.iter().zip(values) {
            let s = serde_json::to_string(value).join_err::<()>("failed to serialize value")?;

            sqlx::query(
                r#"
                INSERT INTO kv (key, value) VALUES (?, ?)
                ON CONFLICT(key) DO UPDATE SET value=excluded.value
            "#,
            )
            .bind(key)
            .bind(s)
            .execute(&mut *tx)
            .await
            .join_err::<()>("failed to insert value")?;

            self.cache
                .write()
                .await
                .insert((*key).to_string(), value.clone());
        }

        tx.commit()
            .await
            .join_err::<()>("failed to commit transaction")?;

        Ok(())
    }

    async fn get_batch(&self, keys: &[&str]) -> joinerror::Result<Vec<Option<JsonValue>>> {
        let mut values = Vec::with_capacity(keys.len());

        for key in keys {
            values.push(self.get(key).await.join_err::<()>("failed to get value")?);
        }

        Ok(values)
    }

    async fn remove_batch(&self, keys: &[&str]) -> joinerror::Result<()> {
        let mut tx = self
            .pool
            .begin()
            .await
            .join_err::<()>("failed to begin transaction")?;

        for key in keys {
            sqlx::query("DELETE FROM kv WHERE key = ?")
                .bind(key)
                .execute(&mut *tx)
                .await
                .join_err::<()>("failed to delete value")?;

            self.cache.write().await.remove(*key);
        }

        tx.commit()
            .await
            .join_err::<()>("failed to commit transaction")?;

        Ok(())
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
