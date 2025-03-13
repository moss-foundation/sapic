use anyhow::Result;
use moss_db::ReDbClient;
use std::path::PathBuf;

const WORKSPACE_STATE_DB_NAME: &str = "state.db";

pub struct Workspace {
    // environments,
    state_db_client: ReDbClient,
}

impl Workspace {
    pub fn new(path: PathBuf) -> Result<Self> {
        let db_client = ReDbClient::new(path.join(WORKSPACE_STATE_DB_NAME))?;

        Ok(Self {
            state_db_client: db_client,
        })
    }
}
