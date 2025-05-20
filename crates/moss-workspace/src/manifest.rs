use serde::{Deserialize, Serialize};

pub(crate) const MANIFEST_FILE_NAME: &str = "workspace.toml";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Manifest {
    pub name: String,
}

impl Manifest {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
