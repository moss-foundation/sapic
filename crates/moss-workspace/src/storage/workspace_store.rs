use std::path::PathBuf;
use moss_db::bincode_table::BincodeTable;
use moss_db::{DatabaseClient, ReDbClient, Transaction};
use crate::storage::{WorkspaceEntity, WorkspaceStore, WorkspaceStoreTable};

use anyhow::Result;
#[rustfmt::skip]
pub(super) const TABLE_WORKSPACES: BincodeTable<String, WorkspaceEntity> = BincodeTable::new("workspaces");

pub struct WorkspaceStoreImpl {
    client: ReDbClient,
    table: WorkspaceStoreTable<'static>,
}

impl WorkspaceStoreImpl {
    pub fn new(client: ReDbClient) -> Self {
        Self {
            client,
            table: TABLE_WORKSPACES,
        }
    }
}

impl WorkspaceStore for WorkspaceStoreImpl {
    fn begin_write(&self) -> Result<(Transaction, &WorkspaceStoreTable)> {
        let write_txn = self.client.begin_write()?;
        Ok((write_txn, &self.table))
    }

    fn begin_read(&self) -> Result<(Transaction, &WorkspaceStoreTable)> {
        let read_txn = self.client.begin_read()?;
        Ok((read_txn, &self.table))
    }

    fn scan(&self) -> Result<Vec<(PathBuf, WorkspaceEntity)>> {
        let read_txn = self.client.begin_read()?;
        Ok(self
            .table
            .scan(&read_txn)?
            .map(|(path, metadata)| (PathBuf::from(path), metadata))
            .collect())
    }
}