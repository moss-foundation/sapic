use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct VaultEntry {
    pub value: String,
    pub description: String,
}
