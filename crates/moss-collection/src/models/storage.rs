use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum RequestEntity {
    Request { order: Option<usize> },
    Group { order: Option<usize> },
}

impl RequestEntity {
    pub fn order(&self) -> Option<usize> {
        match self {
            RequestEntity::Group { order } => order.clone(),
            RequestEntity::Request { order } => order.clone(),
        }
    }
}
