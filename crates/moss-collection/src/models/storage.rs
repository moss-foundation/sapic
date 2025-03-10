use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RequestVariantEntity {
    pub order: usize,
}
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RequestMetadataEntity {
    pub order: Option<usize>,
    pub variants: HashMap<String, RequestVariantEntity>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CollectionMetadataEntity {
    pub order: Option<usize>,
    pub requests: HashMap<Vec<u8>, RequestMetadataEntity>,
}
