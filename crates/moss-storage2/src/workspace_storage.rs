use joinerror::ResultExt;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::OnceCell;

use crate::adapters::{Capabilities, KeyedStorage, Options, sqlite::SqliteStorage};

const DEFAULT_DB_FILENAME: &str = "state.sqlite3";

pub struct WorkspaceStorageBackend {
    db_path: PathBuf,
    storage_options: Option<Options>,
    storage: OnceCell<Arc<SqliteStorage>>,
    capabilities: OnceCell<Capabilities>,
}

impl WorkspaceStorageBackend {
    pub async fn new(path: &Path, options: Option<Options>) -> joinerror::Result<Self> {
        Ok(Self {
            db_path: path.join(DEFAULT_DB_FILENAME),
            storage_options: options,
            storage: OnceCell::new(),
            capabilities: OnceCell::new(),
        })
    }

    pub async fn storage(&self) -> joinerror::Result<Arc<dyn KeyedStorage>> {
        Ok(self.storage_internal().await?)
    }

    pub async fn capabilities(&self) -> joinerror::Result<Capabilities> {
        let capabilities = if let Some(capabilities) = self.capabilities.get() {
            capabilities.clone()
        } else {
            let storage = self.storage_internal().await?;
            let capabilities = Capabilities {
                flushable: Some(storage.clone()),
                optimizable: Some(storage.clone()),
                closable: Some(storage.clone()),
            };

            self.capabilities
                .get_or_init(|| async { capabilities })
                .await
                .clone()
        };

        Ok(capabilities)
    }

    async fn storage_internal(&self) -> joinerror::Result<Arc<SqliteStorage>> {
        let storage = self
            .storage
            .get_or_init(|| async {
                SqliteStorage::new(&self.db_path, self.storage_options.clone().map(Into::into))
                    .await
                    .join_err::<()>("failed to open workspace storage")
                    .unwrap()
            })
            .await;

        Ok(storage.clone())
    }
}
