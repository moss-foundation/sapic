use crate::storage::SegBinTable;
use crate::workspace_storage::stores::WorkspaceVariableStore;

use moss_db::ReDbClient;
use std::sync::Arc;

pub struct WorkspaceVariableStoreImpl {
    client: ReDbClient,
    table: Arc<SegBinTable>,
}

impl WorkspaceVariableStoreImpl {
    pub fn new(client: ReDbClient, table: Arc<SegBinTable>) -> Self {
        Self { client, table }
    }
}

impl WorkspaceVariableStore for WorkspaceVariableStoreImpl {}
