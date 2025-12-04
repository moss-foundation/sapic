use serde::{Deserialize, Serialize};

pub const MANIFEST_FILE_NAME: &str = "Sapic.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ManifestVcs {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectManifest {
    pub name: String,
    pub vcs: Option<ManifestVcs>,
}
