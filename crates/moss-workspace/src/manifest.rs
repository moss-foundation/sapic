use anyhow::Result;
use moss_file::toml::TomlEditor;
use serde::{Deserialize, Serialize};
use toml_edit::DocumentMut;

pub(crate) const MANIFEST_FILE_NAME: &str = "Sapic.toml";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestModel {
    pub name: String,
}

#[derive(Debug)]
pub struct ManifestModelDiff {
    /// A new name for the workspace, if provided,  the workspace
    /// will be renamed to this name.
    pub name: Option<String>,
}

impl TomlEditor for ManifestModelDiff {
    fn edit(&self, doc: &mut DocumentMut) -> Result<()> {
        if let Some(name) = &self.name {
            doc["name"] = name.into();
        }

        Ok(())
    }
}
