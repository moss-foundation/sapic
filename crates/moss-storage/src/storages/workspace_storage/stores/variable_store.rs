use moss_db::ReDbClient;
use std::sync::Arc;

use crate::storage::SegBinTable;
use crate::workspace_storage::stores::WorkspaceVariableStore;


pub struct WorkspaceVariableStoreImpl {
    #[allow(unused)]
    client: ReDbClient,
    #[allow(dead_code)]
    table: Arc<SegBinTable>,
}

impl WorkspaceVariableStoreImpl {
    pub fn new(client: ReDbClient, table: Arc<SegBinTable>) -> Self {
        Self { client, table }
    }
}

impl WorkspaceVariableStore for WorkspaceVariableStoreImpl {}
