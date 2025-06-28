use serde::{Deserialize, Serialize};

pub(crate) const MANIFEST_FILE_NAME: &str = "Sapic.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestModel {
    pub name: String,
}
