use std::collections::HashMap;

use moss_db::{bincode_table::BincodeTable, common::DatabaseError, DatabaseClient, ReDbClient};

use super::{entities::WorkspaceInfoEntity, WorkspacesStore};

#[rustfmt::skip]
pub(super) const TABLE_WORKSPACES: BincodeTable<String, WorkspaceInfoEntity> = BincodeTable::new("workspaces");

pub struct WorkspacesStoreImpl {
    client: ReDbClient,
    table: BincodeTable<'static, String, WorkspaceInfoEntity>,
}

impl WorkspacesStoreImpl {
    pub fn new(client: ReDbClient) -> Self {
        Self {
            client,
            table: TABLE_WORKSPACES,
        }
    }
}

impl WorkspacesStore for WorkspacesStoreImpl {
    fn list_workspaces(&self) -> Result<HashMap<String, WorkspaceInfoEntity>, DatabaseError> {
        let read_txn = self.client.begin_read()?;
        let workspaces = self.table.scan(&read_txn)?;

        Ok(workspaces
            .into_iter()
            .map(|(workspace_name, entity)| (workspace_name, entity))
            .collect())
    }

    fn set_workspace(
        &self,
        workspace_name: String,
        entity: WorkspaceInfoEntity,
    ) -> Result<(), DatabaseError> {
        let mut write_txn = self.client.begin_write()?;
        self.table.insert(&mut write_txn, workspace_name, &entity)?;

        Ok(write_txn.commit()?)
    }

    fn delete_workspace(&self, workspace_name: String) -> Result<(), DatabaseError> {
        let mut write_txn = self.client.begin_write()?;
        self.table.remove(&mut write_txn, workspace_name)?;

        Ok(write_txn.commit()?)
    }
}
