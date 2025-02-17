use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum CollectionKind {
    Local,
    Remote,
}

pub struct LocalCollection {
    pub path: PathBuf,
}
