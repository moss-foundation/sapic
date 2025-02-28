use patricia_tree::PatriciaMap;
use std::{collections::HashMap, path::PathBuf};

use super::collection::RequestType;

#[derive(Debug)]
pub struct RequestVariantEntry {
    pub name: String,
}

#[derive(Debug)]
pub struct RequestEntry {
    pub name: String,
    pub typ: Option<RequestType>,
    pub path: Option<PathBuf>,
    pub variants: HashMap<PathBuf, RequestVariantEntry>,
}

#[derive(Debug)]
pub struct IndexedCollection {
    pub requests: PatriciaMap<RequestEntry>,
}
