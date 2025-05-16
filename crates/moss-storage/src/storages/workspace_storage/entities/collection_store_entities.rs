use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CollectionEntity {
    pub order: Option<usize>,
    pub external_abs_path: Option<PathBuf>,
}
