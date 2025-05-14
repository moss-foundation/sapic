use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CollectionEntity {
    pub order: Option<usize>,
}
