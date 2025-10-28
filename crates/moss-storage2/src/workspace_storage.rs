use joinerror::ResultExt;
use std::{path::Path, sync::Arc};

use crate::adapters::sqlite::SqliteStorage;

const DEFAULT_DB_FILENAME: &str = "state.sqlite3";

pub struct WorkspaceStorageBackend {
    storage: Arc<SqliteStorage>,
}

impl WorkspaceStorageBackend {
    pub async fn new(path: impl AsRef<Path>) -> joinerror::Result<Self> {
        let storage = SqliteStorage::new(path.as_ref().join(DEFAULT_DB_FILENAME), None)
            .await
            .join_err::<()>("failed to create workspace storage")?;

        Ok(Self { storage })
    }
}
