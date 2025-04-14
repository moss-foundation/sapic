use anyhow::Result;
use moss_db::ReDbClient;
use std::{path::PathBuf, sync::Arc};

use super::{
    collection_store::{CollectionStoreImpl, TABLE_COLLECTIONS},
    environment_store::EnvironmentStoreImpl,
    layout_parts_state_store::{PartsStateStoreImpl, TABLE_PARTS_STATE},
    CollectionStore, EnvironmentStore, LayoutPartsStateStore, StateDbManager,
};

const WORKSPACE_STATE_DB_NAME: &str = "state.db";

pub struct StateDbManagerImpl {
    collection_store: Arc<dyn CollectionStore>,
    environment_store: Arc<dyn EnvironmentStore>,
    parts_state_store: Arc<dyn LayoutPartsStateStore>,
}

impl StateDbManagerImpl {
    pub fn new(path: &PathBuf) -> Result<Self> {
        let db_client = ReDbClient::new(path.join(WORKSPACE_STATE_DB_NAME))?
            .with_table(&TABLE_COLLECTIONS)?
            .with_table(&TABLE_PARTS_STATE)?;

        let collection_store = Arc::new(CollectionStoreImpl::new(db_client.clone()));
        let environment_store = Arc::new(EnvironmentStoreImpl::new(db_client.clone()));
        let parts_state_store = Arc::new(PartsStateStoreImpl::new(db_client.clone()));

        Ok(Self {
            collection_store,
            environment_store,
            parts_state_store,
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

    fn layout_parts_state_store(&self) -> Arc<dyn LayoutPartsStateStore> {
        Arc::clone(&self.parts_state_store)
    }
}
