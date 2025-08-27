use moss_git_hosting_provider::models::primitives::GitProviderType;
use serde::{Deserialize, Serialize};

pub const MANIFEST_FILE_NAME: &str = "Sapic.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestFile {
    pub name: String,
    pub repository: Option<ManifestRepository>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestRepository {
    pub url: String,
    pub git_provider_type: GitProviderType,
}
