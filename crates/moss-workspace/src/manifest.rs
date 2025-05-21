use anyhow::{Context as _, Result};
use moss_fs::FileSystem;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::{path::Path, str::FromStr, sync::Arc};
use tokio::sync::RwLock;
use toml_edit::DocumentMut;

pub(crate) const MANIFEST_FILE_NAME: &str = "Sapic.toml";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceManifestModel {
    pub name: String,
}

#[derive(Debug)]
pub struct WorkspaceManifestModelDiff {
    /// A new name for the workspace, if provided,  the workspace
    /// will be renamed to this name.
    pub name: Option<String>,
}

impl Modifier for WorkspaceManifestModelDiff {
    fn modify(&self, doc: &mut DocumentMut) -> Result<()> {
        if let Some(name) = &self.name {
            doc["name"] = name.into();
        }

        Ok(())
    }
}

pub trait Modifier {
    fn modify(&self, doc: &mut DocumentMut) -> Result<()>;
}

pub struct Manifest<T>
where
    T: Clone + Serialize + DeserializeOwned,
{
    fs: Arc<dyn FileSystem>,
    abs_path: Arc<Path>,
    doc: RwLock<DocumentMut>,
    model: RwLock<T>,
}

impl<T> Manifest<T>
where
    T: Clone + Serialize + DeserializeOwned,
{
    pub async fn new(fs: Arc<dyn FileSystem>, dir: Arc<Path>, model: T) -> Result<Self> {
        let abs_path: Arc<Path> = dir.join(MANIFEST_FILE_NAME).into();
        let s = toml::to_string(&model)?;
        let doc = DocumentMut::from_str(&s)?;

        fs.create_file_with(
            &abs_path,
            &s.as_bytes(),
            moss_fs::CreateOptions {
                overwrite: true,
                ignore_if_exists: true,
            },
        )
        .await?;

        Ok(Self {
            fs,
            abs_path,
            doc: RwLock::new(doc),
            model: RwLock::new(model),
        })
    }

    pub async fn load(fs: Arc<dyn FileSystem>, abs_path: Arc<Path>) -> Result<Self> {
        let mut reader = fs
            .open_file(&abs_path.join(MANIFEST_FILE_NAME))
            .await
            .context("Failed to open existing workspace manifest file")?;

        let mut buf = String::new();
        reader
            .read_to_string(&mut buf)
            .context("Failed to read workspace manifest file")?;

        let doc = buf.parse::<DocumentMut>()?;
        let model: T = toml::from_str(&buf)?;

        Ok(Self {
            fs,
            abs_path,
            doc: RwLock::new(doc),
            model: RwLock::new(model),
        })
    }

    async fn sync_model_from_doc(&self) -> Result<()> {
        let doc = self.doc.read().await;
        let model: T = toml::from_str(&doc.to_string())?;
        let mut model_lock = self.model.write().await;
        *model_lock = model;

        Ok(())
    }

    pub async fn modify<M: Modifier>(&self, modifier: M) -> Result<()> {
        let mut doc = self.doc.write().await;
        modifier.modify(&mut *doc)?;
        let content = doc.to_string();
        drop(doc);

        self.fs
            .create_file_with(
                &self.abs_path,
                &content.as_bytes(),
                moss_fs::CreateOptions {
                    overwrite: true,
                    ignore_if_exists: true,
                },
            )
            .await?;

        self.sync_model_from_doc().await?;

        Ok(())
    }

    pub async fn model(&self) -> T {
        self.model.read().await.clone()
    }
}
