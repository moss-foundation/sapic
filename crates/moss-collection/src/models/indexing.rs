use patricia_tree::PatriciaMap;
use std::path::PathBuf;

use super::collection::RequestType;

#[derive(Debug)]
pub struct RequestVariantEntry {
    pub name: String,
    pub path: PathBuf,
}

#[derive(Debug)]
pub struct RequestEntry {
    pub name: String,
    pub ext: Option<RequestType>,
    pub path: Option<PathBuf>,
    pub variants: Vec<RequestVariantEntry>,
}

#[derive(Debug)]
pub struct DirEntry {
    pub name: String,
    pub path: PathBuf,
    pub children: Vec<RequestIndexEntry>,
}

#[derive(Debug)]
pub enum RequestIndexEntry {
    Request(RequestEntry),
    Dir(DirEntry),
}

#[derive(Debug)]
pub struct IndexedCollection {
    pub requests: PatriciaMap<RequestEntry>,
}
