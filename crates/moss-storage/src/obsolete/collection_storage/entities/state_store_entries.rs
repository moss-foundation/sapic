use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WorktreeEntryEntity {
    pub path: PathBuf,
    pub order: usize,
}
