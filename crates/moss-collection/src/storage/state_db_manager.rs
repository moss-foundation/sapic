use super::{
    request_store::{self, RequestStoreImpl},
    RequestStore, StateDbManager,
};
use anyhow::Result;
use moss_db::ReDbClient;
use std::path::Path;
use std::{path::PathBuf, sync::Arc};

const COLLECTION_STATE_DB_NAME: &str = "state.db";

pub struct StateDbManagerImpl {
    request_store: Arc<dyn RequestStore>,
}

impl StateDbManagerImpl {
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let db_client = ReDbClient::new(path.as_ref().join(COLLECTION_STATE_DB_NAME))?
            .with_bincode_table(&request_store::TABLE_REQUESTS)?;

        let request_store = Arc::new(RequestStoreImpl::new(db_client));

        Ok(Self { request_store })
    }
}

impl StateDbManager for StateDbManagerImpl {
    fn request_store(&self) -> Arc<dyn RequestStore> {
        self.request_store.clone()
    }
}
