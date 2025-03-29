use anyhow::Result;
use std::{path::PathBuf, sync::Arc};
use std::path::Path;
use moss_db::ReDbClient;

use super::{
    collection_store::{self, CollectionStoreImpl},
    CollectionStore, StateDbManager,
};

const WORKSPACE_STATE_DB_NAME: &str = "state.db";

pub struct StateDbManagerImpl {
    collection_store: Arc<dyn CollectionStore>,
}

impl StateDbManagerImpl {
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let db_client = ReDbClient::new(path.as_ref().join(WORKSPACE_STATE_DB_NAME))?
            .with_bincode_table(&collection_store::TABLE_COLLECTIONS)?;
        let collection_store = Arc::new(CollectionStoreImpl::new(db_client));

        Ok(Self { collection_store })
    }
}

impl StateDbManager for StateDbManagerImpl {
    fn collection_store(&self) -> Arc<dyn CollectionStore> {
        Arc::clone(&self.collection_store)
    }
}
