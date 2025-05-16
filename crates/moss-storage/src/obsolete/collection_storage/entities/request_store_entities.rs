use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RequestEntity {
    pub order: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GroupEntity {
    pub order: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum RequestNodeEntity {
    Request(RequestEntity),
    Group(GroupEntity),
}

impl RequestNodeEntity {
    pub fn as_request(&self) -> Option<&RequestEntity> {
        match self {
            RequestNodeEntity::Request(request) => Some(request),
            _ => None,
        }
    }

    pub fn as_group(&self) -> Option<&GroupEntity> {
        match self {
            RequestNodeEntity::Group(group) => Some(group),
            _ => None,
        }
    }
}
