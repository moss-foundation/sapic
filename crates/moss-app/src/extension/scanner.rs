use moss_fs::FileSystem;
use moss_logging::session;
use serde::Deserialize;
use serde_json::Value as JsonValue;
use std::{
    collections::HashMap,
    fmt::{self, Display},
    path::PathBuf,
    sync::Arc,
};

#[derive(Debug, Clone, PartialEq)]
pub enum ExtensionKind {
    BuiltIn,
    User,
}

impl ExtensionKind {
    pub fn is_builtin(&self) -> bool {
        self == &ExtensionKind::BuiltIn
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExtensionManifestFile {
    contributes: HashMap<String, JsonValue>,
}

#[derive(Debug)]
pub struct ExtensionDescription {
    pub kind: ExtensionKind,
    pub abs_path: PathBuf,
    pub contributes: HashMap<String, JsonValue>,
}

pub struct ExtensionScanner {
    roots: Vec<(PathBuf, ExtensionKind)>,
    fs: Arc<dyn FileSystem>,
}

impl ExtensionScanner {
    pub fn new(fs: Arc<dyn FileSystem>, roots: Vec<(PathBuf, ExtensionKind)>) -> Self {
        Self { fs, roots }
    }

    pub async fn scan(&self) -> joinerror::Result<Vec<ExtensionDescription>> {
        let mut extensions = Vec::new();

        for (abs_path, kind) in &self.roots {
            let mut read_dir = self.fs.read_dir(abs_path).await?;
            while let Some(entry) = read_dir.next_entry().await? {
                if entry.file_type().await?.is_file() {
                    continue;
                }

                let manifest_path = entry.path().join("Sapic.json");
                if !manifest_path.exists() {
                    session::warn!(format!(
                        "manifest file not found: {}",
                        manifest_path.display()
                    ));
                    continue;
                }

                let rdr = self.fs.open_file(&manifest_path).await?;
                let parsed: ExtensionManifestFile = serde_json::from_reader(rdr)?;

                extensions.push(ExtensionDescription {
                    kind: kind.clone(),
                    abs_path: entry.path(),
                    contributes: parsed.contributes,
                });
            }
        }

        Ok(extensions)
    }
}
