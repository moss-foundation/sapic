use anyhow::Result;
use std::{path::PathBuf, sync::Arc};

use moss_db::ReDbClient;

use super::{collection_store::CollectionStoreImpl, CollectionStore, StateDbManager};

const WORKSPACE_STATE_DB_NAME: &str = "state.db";

pub struct StateDbManagerImpl {
    collection_store: Arc<dyn CollectionStore>,
}

impl StateDbManagerImpl {
    pub fn new(path: &PathBuf) -> Result<Self> {
        let db_client = ReDbClient::new(path.join(WORKSPACE_STATE_DB_NAME))?;
        let collection_store = Arc::new(CollectionStoreImpl::new(db_client.clone()));

        Ok(Self { collection_store })
    }
}

impl StateDbManager for StateDbManagerImpl {
    fn collection_store(&self) -> Arc<dyn CollectionStore> {
        Arc::clone(&self.collection_store)
    }
}
