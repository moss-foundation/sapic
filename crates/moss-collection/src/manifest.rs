use anyhow::Result;
use moss_bindingutils::primitives::ChangeString;
use moss_file::toml::InPlaceEditor;
use serde::{Deserialize, Serialize};
use toml_edit::DocumentMut;

pub(crate) const MANIFEST_FILE_NAME: &str = "Sapic.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestModel {
    pub name: String,
    /// The canonicalized form of git url, with protocol and ".git" suffix striped
    pub repository: Option<String>,
}

#[derive(Debug)]
pub struct ManifestModelDiff {
    /// A new name for the collection, if provided, the collection
    /// will be renamed to this name.
    pub name: Option<String>,
    /// An update to the repository url, if provided, the collection
    /// will be either updated or removed.
    pub repository: Option<ChangeString>,
}

impl InPlaceEditor for ManifestModelDiff {
    fn edit(&self, doc: &mut DocumentMut) -> Result<()> {
        if let Some(name) = &self.name {
            doc["name"] = name.into();
        }
        match &self.repository {
            None => {}
            Some(ChangeString::Remove) => {
                doc.remove("repository");
            }
            Some(ChangeString::Update(new_repo)) => {
                doc["repository"] = new_repo.clone().into();
            }
        }

        Ok(())
    }
}
