use moss_db::ReDbClient;
use std::sync::Arc;

use crate::{storage::SegBinTable, workspace_storage::stores::WorkspaceVariableStore};

use moss_applib::ctx::AnyAsyncContext;

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

impl<Context> WorkspaceVariableStore<Context> for WorkspaceVariableStoreImpl where
    Context: AnyAsyncContext
{
}
