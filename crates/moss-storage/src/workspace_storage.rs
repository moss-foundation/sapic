use std::path::PathBuf;

use moss_db::{common::DatabaseError, ReDbClient};

const WORKSPACE_STATE_DB_NAME: &str = "state.db";

pub struct WorkspaceStorageImpl {}

// impl WorkspaceStorageImpl {
//     pub fn new(workspace_path: &PathBuf) -> Result<Self, DatabaseError> {
//         let db_client = ReDbClient::new(workspace_path.join(WORKSPACE_STATE_DB_NAME))?
//             .with_table(&TABLE_COLLECTIONS)?
//             .with_table(&TABLE_PARTS_STATE)?;

//         Ok(Self {})
//     }
// }
