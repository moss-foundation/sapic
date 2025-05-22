use anyhow::{Context as _, Result};
use moss_fs::FileSystem;
use serde::{Serialize, de::DeserializeOwned};
use std::{path::Path, str::FromStr, sync::Arc};
use tokio::sync::RwLock;
use toml_edit::DocumentMut;

pub trait TomlEditor {
    fn edit(&self, doc: &mut DocumentMut) -> Result<()>;
}

pub struct EditableToml<T>
where
    T: Clone + Serialize + DeserializeOwned,
{
    fs: Arc<dyn FileSystem>,
    abs_path: Arc<Path>,
    doc: RwLock<DocumentMut>,
    model: RwLock<T>,
}

impl<T> EditableToml<T>
where
    T: Clone + Serialize + DeserializeOwned,
{
    pub async fn new(
        fs: Arc<dyn FileSystem>,
        abs_path: impl AsRef<Path>,
        model: T,
    ) -> Result<Self> {
        let abs_path: Arc<Path> = abs_path.as_ref().into();
        debug_assert!(abs_path.is_absolute());

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

    pub async fn load(fs: Arc<dyn FileSystem>, abs_path: impl AsRef<Path>) -> Result<Self> {
        let abs_path: Arc<Path> = abs_path.as_ref().into();
        debug_assert!(abs_path.is_absolute());

        dbg!(&abs_path);
        let mut reader = fs
            .open_file(&abs_path)
            .await
            .context("Failed to open file")?;

        let mut buf = String::new();
        reader
            .read_to_string(&mut buf)
            .context("Failed to read file")?;

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

    pub async fn edit<M: TomlEditor>(&self, modifier: M) -> Result<()> {
        let mut doc = self.doc.write().await;
        modifier.edit(&mut *doc)?;
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
