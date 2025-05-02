use std::collections::HashMap;

use moss_db::{
    bincode_table::BincodeTable, common::DatabaseError, DatabaseClient, ReDbClient, Transaction,
};

use super::{entities::WorkspaceInfoEntity, WorkspacesStore};

#[rustfmt::skip]
pub(in crate::global_storage) const TABLE_WORKSPACES: BincodeTable<String, WorkspaceInfoEntity> = BincodeTable::new("workspaces");

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

    fn upsert_workspace(
        &self,
        txn: &mut Transaction,
        workspace_name: String,
        entity: WorkspaceInfoEntity,
    ) -> Result<(), DatabaseError> {
        self.table.insert(txn, workspace_name, &entity)?;

        Ok(())
    }

    fn rekey_workspace(
        &self,
        txn: &mut Transaction,
        old_workspace_name: String,
        new_workspace_name: String,
    ) -> Result<(), DatabaseError> {
        self.table
            .rekey(txn, old_workspace_name, new_workspace_name)?;

        Ok(())
    }

    fn delete_workspace(
        &self,
        txn: &mut Transaction,
        workspace_name: String,
    ) -> Result<(), DatabaseError> {
        self.table.remove(txn, workspace_name)?;

        Ok(())
    }
}
