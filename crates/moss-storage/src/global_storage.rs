pub mod entities;
pub mod workspace_store;

use std::{path::PathBuf, sync::Arc};

use async_trait::async_trait;
use entities::WorkspaceInfoEntity;
use moss_db::{common::DatabaseError, DatabaseClient, ReDbClient, Transaction};
use std::collections::HashMap;
use workspace_store::{WorkspacesStoreImpl, TABLE_WORKSPACES};

use crate::{common::Transactional, GlobalStorage};

const GLOBAL_STATE_DB_NAME: &str = "state.db";

pub trait WorkspacesStore: Send + Sync {
    fn set_workspace(
        &self,
        workspace_name: String,
        entity: WorkspaceInfoEntity,
    ) -> Result<(), DatabaseError>;
    fn list_workspaces(&self) -> Result<HashMap<String, WorkspaceInfoEntity>, DatabaseError>;
    fn delete_workspace(&self, workspace_name: String) -> Result<(), DatabaseError>;
}

pub struct GlobalStorageImpl {
    db_client: ReDbClient,
    workspaces_store: Arc<WorkspacesStoreImpl>,
}

impl GlobalStorageImpl {
    pub fn new(path: &PathBuf) -> Result<Self, DatabaseError> {
        let db_client =
            ReDbClient::new(path.join(GLOBAL_STATE_DB_NAME))?.with_table(&TABLE_WORKSPACES)?;

        let workspaces_store = Arc::new(WorkspacesStoreImpl::new(db_client.clone()));

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
    fn workspaces_store(&self) -> Arc<dyn WorkspacesStore> {
        self.workspaces_store.clone()
    }
}
