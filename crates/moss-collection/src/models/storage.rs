use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestVariantEntity {
    pub order: usize,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct RequestMetadataEntity {
    pub order: Option<usize>,
    pub variants: HashMap<String, RequestVariantEntity>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CollectionMetadataEntity {
    pub order: Option<usize>,
    pub requests: HashMap<Vec<u8>, RequestMetadataEntity>,
}
