use async_trait::async_trait;
use joinerror::{Error, ResultExt};
use moss_logging::session;
use sqlx::{
    Row, SqlitePool,
    sqlite::{SqliteConnectOptions, SqliteJournalMode},
};
use std::{collections::HashMap, path::Path, str::FromStr, sync::Arc, time::Duration};
use tokio::sync::RwLock;

use crate::adapters::StorageAdapter;

const DEFAULT_BUSY_TIMEOUT: Duration = Duration::from_secs(5);

pub struct SqliteStorageOptions {
    busy_timeout: Duration,
}

pub struct SqliteStorage {
    // INFO: This will be improved in the future to support glob search for keys.
    cache: RwLock<HashMap<String, String>>,
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

                if let Ok(value_str) = String::from_utf8(value) {
                    cache.insert(key, value_str);
                }
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
impl StorageAdapter for SqliteStorage {
    async fn put(&self, key: &str, value: &str) -> joinerror::Result<()> {
        Ok(())
    }

    async fn get(&self, key: &str) -> joinerror::Result<String> {
        Ok(String::new())
    }

    async fn remove(&self, key: &str) -> joinerror::Result<()> {
        Ok(())
    }

    async fn when_flushed(&self) -> joinerror::Result<()> {
        Ok(())
    }

    async fn flush(&self) -> joinerror::Result<()> {
        Ok(())
    }

    async fn optimize(&self) -> joinerror::Result<()> {
        Ok(())
    }
}
