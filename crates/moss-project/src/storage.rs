pub mod entities;
pub mod segments;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WorktreeNodeStateEntity {
    pub expanded: bool,
    pub order: Option<isize>,
}
