use crate::storage::workspace_store::WorkspaceStoreImpl;
use crate::storage::{workspace_store, GlobalDbManager, WorkspaceStore};
use anyhow::Result;
use moss_db::ReDbClient;
use std::path::Path;
use std::sync::Arc;

const GLOBAL_STATE_DB_NAME: &str = "global.db";

// pub struct GlobalDbManagerImpl {
//     workspace_store: Arc<dyn WorkspaceStore>,
// }

// impl GlobalDbManagerImpl {
//     pub fn new(path: impl AsRef<Path>) -> Result<Self> {
//         let db_client = ReDbClient::new(path.as_ref().join(GLOBAL_STATE_DB_NAME))?
//             .with_bincode_table(&workspace_store::TABLE_WORKSPACES)?;
//         let workspace_store = Arc::new(WorkspaceStoreImpl::new(db_client));

//         Ok(Self { workspace_store })
//     }
// }

// impl GlobalDbManager for GlobalDbManagerImpl {
//     fn workspace_store(&self) -> Arc<dyn WorkspaceStore> {
//         Arc::clone(&self.workspace_store)
//     }
// }
