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
    request_store: Option<Arc<dyn RequestStore>>,
}

fn generate_request_store(path: &Path) -> Result<Arc<dyn RequestStore>> {
    let db_client =
        ReDbClient::new(path.join(COLLECTION_STATE_DB_NAME))?
            .with_table(&TABLE_REQUESTS)?;

    Ok(Arc::new(RequestStoreImpl::new(db_client)))
}

impl StateDbManagerImpl {
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let request_store = generate_request_store(path.as_ref())?;

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

        self.request_store = Some(generate_request_store(&self.path)?);

        Ok(())

    }

}
