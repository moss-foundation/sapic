pub mod entities;
pub mod workspace_store;

use std::{path::PathBuf, sync::Arc};

use entities::WorkspaceInfoEntity;
use moss_db::{common::DatabaseError, ReDbClient};
use std::collections::HashMap;
use workspace_store::{WorkspacesStoreImpl, TABLE_WORKSPACES};

use crate::GlobalStorage;

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
    workspaces_store: Arc<WorkspacesStoreImpl>,
}

impl GlobalStorageImpl {
    pub fn new(path: &PathBuf) -> Result<Self, DatabaseError> {
        let db_client =
            ReDbClient::new(path.join(GLOBAL_STATE_DB_NAME))?.with_table(&TABLE_WORKSPACES)?;

        Ok(Self {
            workspaces_store: Arc::new(WorkspacesStoreImpl::new(db_client)),
        })
    }
}

impl GlobalStorage for GlobalStorageImpl {
    fn workspaces_store(&self) -> Arc<dyn WorkspacesStore> {
        self.workspaces_store.clone()
    }
}
