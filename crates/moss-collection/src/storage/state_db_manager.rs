use anyhow::Result;
use moss_db::ReDbClient;
use std::{path::PathBuf, sync::Arc};

use super::{
    request_store::{self, RequestStoreImpl},
    RequestStore, StateDbManager,
};

const COLLECTION_STATE_DB_NAME: &str = "state.db";

pub struct StateDbManagerImpl {
    request_store: Arc<dyn RequestStore>,
}

impl StateDbManagerImpl {
    pub fn new(path: &PathBuf) -> Result<Self> {
        let db_client = ReDbClient::new(path.join(COLLECTION_STATE_DB_NAME))?
            .with_bincode_table(&request_store::TABLE_REQUESTS)?;

        let request_store = Arc::new(RequestStoreImpl::new(db_client.clone()));

        Ok(Self { request_store })
    }
}

impl StateDbManager for StateDbManagerImpl {
    fn request_store(&self) -> Arc<dyn RequestStore> {
        self.request_store.clone()
    }
}
