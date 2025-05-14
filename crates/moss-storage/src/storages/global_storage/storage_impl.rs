use std::{path::PathBuf, sync::Arc};

use async_trait::async_trait;
use moss_db::{DatabaseClient, ReDbClient, Transaction, common::DatabaseError};
use std::collections::HashMap;

use crate::storage::Transactional;

use super::{
    GlobalStorage,
    entities::WorkspaceInfoEntity,
    workbench_store::{TABLE_WORKSPACES, WorkbenchStoreImpl},
};

const GLOBAL_STATE_DB_NAME: &str = "state.db";

pub trait WorkbenchStore: Send + Sync {
    fn upsert_workspace(
        &self,
        txn: &mut Transaction,
        workspace_name: String,
        entity: WorkspaceInfoEntity,
    ) -> Result<(), DatabaseError>;
    fn rekey_workspace(
        &self,
        txn: &mut Transaction,
        old_workspace_name: String,
        new_workspace_name: String,
    ) -> Result<(), DatabaseError>;
    fn list_workspaces(&self) -> Result<HashMap<String, WorkspaceInfoEntity>, DatabaseError>;
    fn delete_workspace(
        &self,
        txn: &mut Transaction,
        workspace_name: String,
    ) -> Result<(), DatabaseError>;
}

pub struct GlobalStorageImpl {
    db_client: ReDbClient,
    workspaces_store: Arc<WorkbenchStoreImpl>,
}

impl GlobalStorageImpl {
    pub fn new(path: &PathBuf) -> Result<Self, DatabaseError> {
        let db_client =
            ReDbClient::new(path.join(GLOBAL_STATE_DB_NAME))?.with_table(&TABLE_WORKSPACES)?;

        let workspaces_store = Arc::new(WorkbenchStoreImpl::new(db_client.clone()));

        Ok(Self {
            db_client,
            workspaces_store,
        })
    }
}

#[async_trait]
impl Transactional for GlobalStorageImpl {
    async fn begin_write(&self) -> Result<Transaction, DatabaseError> {
        self.db_client.begin_write()
    }

    async fn begin_read(&self) -> Result<Transaction, DatabaseError> {
        self.db_client.begin_read()
    }
}

impl GlobalStorage for GlobalStorageImpl {
    fn workspaces_store(&self) -> Arc<dyn WorkbenchStore> {
        self.workspaces_store.clone()
    }
}
