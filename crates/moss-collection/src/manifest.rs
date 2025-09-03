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
    GitHub { repository: String },
    GitLab { repository: String },
}

impl ManifestVcs {
    pub fn repository(&self) -> &str {
        match self {
            ManifestVcs::GitHub { repository, .. } => repository,
            ManifestVcs::GitLab { repository, .. } => repository,
        }
    }
}
