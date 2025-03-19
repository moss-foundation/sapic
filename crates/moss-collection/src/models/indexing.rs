use patricia_tree::PatriciaMap;
use std::path::PathBuf;

use super::collection::RequestType;

#[derive(Debug)]
pub struct RequestEntry {
    pub name: String,
    pub typ: Option<RequestType>,
    pub path: Option<PathBuf>,
}

#[derive(Debug)]
pub struct IndexedCollection {
    pub requests: PatriciaMap<RequestEntry>,
}
