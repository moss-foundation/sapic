use anyhow::Result;
use moss_db::ReDbClient;
use std::{path::PathBuf, sync::Arc};

use super::{
    collection_store::{CollectionStoreImpl, TABLE_COLLECTIONS},
    environment_store::EnvironmentStoreImpl,
    CollectionStore, EnvironmentStore, PartsStateStore, StateDbManager,
};

const WORKSPACE_STATE_DB_NAME: &str = "state.db";

pub struct StateDbManagerImpl {
    collection_store: Arc<dyn CollectionStore>,
    environment_store: Arc<dyn EnvironmentStore>,
}

impl StateDbManagerImpl {
    pub fn new(path: &PathBuf) -> Result<Self> {
        let db_client =
            ReDbClient::new(path.join(WORKSPACE_STATE_DB_NAME))?.with_table(&TABLE_COLLECTIONS)?;

        let collection_store = Arc::new(CollectionStoreImpl::new(db_client.clone()));
        let environment_store = Arc::new(EnvironmentStoreImpl::new(db_client.clone()));

        Ok(Self {
            collection_store,
            environment_store,
        })
    }
}

impl StateDbManager for StateDbManagerImpl {
    fn collection_store(&self) -> Arc<dyn CollectionStore> {
        Arc::clone(&self.collection_store)
    }

    fn environment_store(&self) -> Arc<dyn EnvironmentStore> {
        Arc::clone(&self.environment_store)
    }

    fn parts_state_store(&self) -> Arc<dyn PartsStateStore> {
        todo!()
    }
}
