use joinerror::ResultExt;
use moss_fs::FileSystem;
use sapic_base::extension::manifest::AddonManifestFile;
use sapic_core::context::AnyAsyncContext;
use serde_json::Value as JsonValue;
use std::{collections::HashMap, path::PathBuf, sync::Arc};

#[derive(Debug, Clone, PartialEq)]
pub enum ExtensionsKind {
    BuiltIn,
    User,
}

#[derive(Debug)]
pub struct ExtensionDescription {
    #[allow(unused)]
    pub kind: ExtensionsKind,
    pub abs_path: PathBuf,
    pub contributes: HashMap<String, JsonValue>,
}

pub struct ExtensionScanner {
    roots: Vec<(PathBuf, ExtensionsKind)>,
    fs: Arc<dyn FileSystem>,
}

impl ExtensionScanner {
    pub fn new(fs: Arc<dyn FileSystem>, roots: Vec<(PathBuf, ExtensionsKind)>) -> Self {
        Self { fs, roots }
    }

    pub async fn scan(
        &self,
        ctx: &dyn AnyAsyncContext,
    ) -> joinerror::Result<Vec<ExtensionDescription>> {
        let mut extensions = Vec::new();

        for (abs_path, kind) in &self.roots {
            let mut read_dir = self
                .fs
                .read_dir(ctx, abs_path)
                .await
                .join_err_with::<()>(|| {
                    format!("failed to read directory {}", abs_path.display())
                })?;

            while let Some(entry) = read_dir.next_entry().await? {
                if entry.file_type().await?.is_file() {
                    continue;
                }

                let manifest_path = entry.path().join("Sapic.json");
                if !manifest_path.exists() {
                    tracing::warn!("manifest file not found: {}", manifest_path.display());
                    continue;
                }

                let rdr = self.fs.open_file(ctx, &manifest_path).await?;
                let parsed: AddonManifestFile = serde_json::from_reader(rdr)?;

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
