use super::request_store::TABLE_REQUESTS;
use super::{request_store::RequestStoreImpl, RequestStore, StateDbManager};
use anyhow::Result;
use moss_db::ReDbClient;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use async_trait::async_trait;
use moss_fs::{FileSystem, RenameOptions};

const COLLECTION_STATE_DB_NAME: &str = "state.db";

pub struct StateDbManagerImpl {
    path: PathBuf,
    // FIXME: It appears that we need to wrap it in Option for temporary dropping in safe Rust
    request_store: Option<Arc<dyn RequestStore>>,
}

impl StateDbManagerImpl {
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let db_client = ReDbClient::new(path.as_ref().join(COLLECTION_STATE_DB_NAME))?
            .with_table(&TABLE_REQUESTS)?;

        let request_store = Arc::new(RequestStoreImpl::new(db_client));

        Ok(Self { path: path.as_ref().to_path_buf(), request_store: Some(request_store) })
    }
}

#[async_trait::async_trait]
impl StateDbManager for StateDbManagerImpl {
    fn request_store(&self) -> Arc<dyn RequestStore> {
        self.request_store.clone().unwrap()
    }

    // Temporarily drop the db for fs renaming
    async fn reset(&mut self, fs: Arc<dyn FileSystem>, new_path: &Path) -> Result<()> {
        std::mem::take(&mut self.request_store);

        let old_path = std::mem::replace(&mut self.path, new_path.to_path_buf());
        fs.rename(&old_path, &new_path, RenameOptions::default()).await?;

        let db_client = ReDbClient::new(&self.path.join(COLLECTION_STATE_DB_NAME))?
            .with_table(&TABLE_REQUESTS)?;
        self.request_store = Some(Arc::new(RequestStoreImpl::new(db_client)));
        Ok(())

    }

}
