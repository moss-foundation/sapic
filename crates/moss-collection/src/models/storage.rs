use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct RequestVariantEntity {
    pub order: usize,
}
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct RequestMetadataEntity {
    pub order: Option<usize>,
    pub variants: HashMap<String, RequestVariantEntity>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CollectionEntity {
    pub order: Option<usize>,
    pub requests: HashMap<Vec<u8>, RequestMetadataEntity>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RequestEntity {
    pub order: Option<usize>,
}
