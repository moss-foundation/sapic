use joinerror::ResultExt;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::OnceCell;

use crate::adapters::sqlite::SqliteStorage;

const DEFAULT_DB_FILENAME: &str = "state.sqlite3";

#[derive(Clone)]
pub struct WorkspaceStorageBackend {
    db_path: PathBuf,
    storage: OnceCell<Arc<SqliteStorage>>,
}

impl WorkspaceStorageBackend {
    pub async fn new(path: &Path) -> joinerror::Result<Self> {
        Ok(Self {
            db_path: path.join(DEFAULT_DB_FILENAME),
            storage: OnceCell::new(),
        })
    }

    pub(crate) async fn storage(&self) -> joinerror::Result<Arc<SqliteStorage>> {
        let storage = self
            .storage
            .get_or_init(|| async {
                SqliteStorage::new(&self.db_path, None)
                    .await
                    .join_err::<()>("failed to open workspace storage")
                    .unwrap()
            })
            .await;

        Ok(storage.clone())
    }
}
