use anyhow::Result;
use moss_file::toml::InPlaceEditor;
use serde::{Deserialize, Serialize};
use toml_edit::DocumentMut;
use url::Url;

pub(crate) const MANIFEST_FILE_NAME: &str = "Sapic.toml";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestModel {
    pub name: String,
    // TODO: Validation of repo path?
    // We can have two types of repo paths though: HTTPS and SSH
    pub repo: Option<Url>,
}

#[derive(Debug)]
pub struct ManifestModelDiff {
    /// A new name for the collection, if provided, the collection
    /// will be renamed to this name.
    pub name: Option<String>,
    /// A new repo link for the collection, if provided, the collection
    /// will be relinked to this repo.
    pub repo: Option<Url>,
}

impl InPlaceEditor for ManifestModelDiff {
    fn edit(&self, doc: &mut DocumentMut) -> Result<()> {
        if let Some(name) = &self.name {
            doc["name"] = name.into();
        }
        if let Some(repo) = &self.repo {
            doc["repo"] = repo.to_string().into();
        }

        Ok(())
    }
}
