use moss_git_hosting_provider::models::primitives::GitProviderType;
use serde::{Deserialize, Serialize};

pub(crate) const MANIFEST_FILE_NAME: &str = "Sapic.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct ManifestFile {
    pub name: String,
    pub repository: Option<ManifestRepository>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct ManifestRepository {
    pub url: String,
    pub git_provider_type: GitProviderType,
}
