use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WorkspaceInfoEntity {
    pub last_opened_at: i64,
}
