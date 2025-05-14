use std::collections::HashMap;

use moss_db::{
    DatabaseClient, ReDbClient, Transaction, bincode_table::BincodeTable, common::DatabaseError,
};

use super::{
    WorkbenchStore,
    entities::{EnvironmentInfoEntity, WorkspaceInfoEntity},
};

#[rustfmt::skip]
pub(in crate::global_storage) const TABLE_WORKSPACES: BincodeTable<String, WorkspaceInfoEntity> = BincodeTable::new("workspaces");

pub struct WorkbenchStoreImpl {
    client: ReDbClient,
    table: BincodeTable<'static, String, WorkspaceInfoEntity>,
}

impl WorkbenchStoreImpl {
    pub fn new(client: ReDbClient) -> Self {
        Self {
            client,
            table: TABLE_WORKSPACES,
        }
    }
}

impl WorkbenchStore for WorkbenchStoreImpl {
    fn list_workspaces(&self) -> Result<HashMap<String, WorkspaceInfoEntity>, DatabaseError> {
        let read_txn = self.client.begin_read()?;
        let workspaces = self.table.scan(&read_txn)?;

        Ok(workspaces
            .into_iter()
            .map(|(workspace_name, entity)| (workspace_name, entity))
            .collect())
    }

    // TODO: implement this
    fn upsert_workspace(
        &self,
        txn: &mut Transaction,
        workspace_name: String,
        entity: WorkspaceInfoEntity,
    ) -> Result<(), DatabaseError> {
        self.table.insert(txn, workspace_name, &entity)?;

        Ok(())
    }

    // TODO: implement this
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

    // TODO: implement this
    fn delete_workspace(
        &self,
        txn: &mut Transaction,
        workspace_name: String,
    ) -> Result<(), DatabaseError> {
        self.table.remove(txn, workspace_name)?;

        Ok(())
    }
}
