use moss_git_hosting_provider::common::GitProviderType;
use serde::{Deserialize, Serialize};

pub(crate) const MANIFEST_FILE_NAME: &str = "Sapic.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestFile {
    pub name: String,
    pub repository: Option<String>,
    pub git_provider_type: Option<GitProviderType>,
}
