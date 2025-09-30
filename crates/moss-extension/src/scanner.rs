use moss_fs::{FileSystem, FsResultExt};
use moss_logging::session;
use serde_json::Value as JsonValue;
use std::{collections::HashMap, path::PathBuf, sync::Arc};

use crate::manifest::AddonManifestFile;

#[derive(Debug, Clone, PartialEq)]
pub enum AddonKind {
    BuiltIn,
    User,
}

#[derive(Debug)]
pub struct AddonDescription {
    #[allow(unused)]
    pub kind: AddonKind,
    pub abs_path: PathBuf,
    pub contributes: HashMap<String, JsonValue>,
}

pub struct AddonScanner {
    roots: Vec<(PathBuf, AddonKind)>,
    fs: Arc<dyn FileSystem>,
}

impl AddonScanner {
    pub fn new(fs: Arc<dyn FileSystem>, roots: Vec<(PathBuf, AddonKind)>) -> Self {
        Self { fs, roots }
    }

    pub async fn scan(&self) -> joinerror::Result<Vec<AddonDescription>> {
        let mut addons = Vec::new();

        for (abs_path, kind) in &self.roots {
            let mut read_dir = self.fs.read_dir(abs_path).await.join_err_with::<()>(|| {
                format!("failed to read directory {}", abs_path.display())
            })?;

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
                let parsed: AddonManifestFile = serde_json::from_reader(rdr)?;

                addons.push(AddonDescription {
                    kind: kind.clone(),
                    abs_path: entry.path(),
                    contributes: parsed.contributes,
                });
            }
        }

        Ok(addons)
    }
}
