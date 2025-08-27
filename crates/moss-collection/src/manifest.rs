use moss_git_hosting_provider::models::primitives::GitProviderType;
use serde::{Deserialize, Serialize};

pub(crate) const MANIFEST_FILE_NAME: &str = "Sapic.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct ManifestFile {
    pub name: String,
    pub vcs: Option<ManifestVcs>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(super) enum ManifestVcs {
    GITHUB { repository: String },
    GITLAB { repository: String },
}

impl ManifestVcs {
    pub fn git_provider_type(&self) -> GitProviderType {
        match self {
            ManifestVcs::GITHUB { .. } => GitProviderType::GitHub,
            ManifestVcs::GITLAB { .. } => GitProviderType::GitLab,
        }
    }

    pub fn repository(&self) -> &str {
        match self {
            ManifestVcs::GITHUB { repository, .. } => repository,
            ManifestVcs::GITLAB { repository, .. } => repository,
        }
    }
}
