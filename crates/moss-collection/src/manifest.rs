use anyhow::Result;
use moss_common::api::Change;
use moss_file::toml::InPlaceEditor;
use serde::{Deserialize, Serialize};
use toml_edit::DocumentMut;
use url::Url;

pub(crate) const MANIFEST_FILE_NAME: &str = "Sapic.toml";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestModel {
    pub name: String,
    pub repository: Option<Url>,
}

#[derive(Debug)]
pub struct ManifestModelDiff {
    /// A new name for the collection, if provided, the collection
    /// will be renamed to this name.
    pub name: Option<String>,
    /// An update to the repository url, if provided, the collection
    /// will be either updated or removed.
    pub repository: Option<Change<Url>>,
}

impl InPlaceEditor for ManifestModelDiff {
    fn edit(&self, doc: &mut DocumentMut) -> Result<()> {
        if let Some(name) = &self.name {
            doc["name"] = name.into();
        }
        match &self.repository {
            None => {}
            Some(Change::Remove) => {
                doc.remove("repository");
            }
            Some(Change::Update(new_repo)) => {
                doc["repository"] = new_repo.to_string().into();
            }
        }

        Ok(())
    }
}
