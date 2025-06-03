use anyhow::{Context as _, Result};
use moss_fs::FileSystem;
use serde::{Serialize, de::DeserializeOwned};
use std::{ops::Deref, path::Path, str::FromStr, sync::Arc};
use tokio::sync::RwLock;
use toml_edit::DocumentMut;

use crate::common::FileHandle;

pub struct TomlFileHandle<T>
where
    T: Clone + Serialize + DeserializeOwned,
{
    inner: FileHandle<T>,
}

impl<T> Deref for TomlFileHandle<T>
where
    T: Clone + Serialize + DeserializeOwned,
{
    type Target = FileHandle<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> TomlFileHandle<T>
where
    T: Clone + Serialize + DeserializeOwned,
{
    pub fn new(file: FileHandle<T>) -> Self {
        Self { inner: file }
    }

    pub async fn load(fs: Arc<dyn FileSystem>, abs_path: &Path) -> Result<Self> {
        let file = FileHandle::load(fs, abs_path, |s| {
            toml::from_str(s).map_err(|err| anyhow::anyhow!("Failed to parse TOML file: {}", err))
        })
        .await?;
        Ok(Self { inner: file })
    }

    pub async fn create(fs: Arc<dyn FileSystem>, abs_path: &Path, model: T) -> Result<Self> {
        let file = FileHandle::create(fs, abs_path, model, |s| {
            toml::to_string(s)
                .map_err(|err| anyhow::anyhow!("Failed to serialize TOML file: {}", err))
        })
        .await?;
        Ok(Self { inner: file })
    }
}

/// A trait for types that can modify a `toml_edit::DocumentMut`.
///
/// This trait is used by `EditableInPlaceFileHandle` to allow for fine-grained modifications
/// to a TOML document while preserving comments and formatting.
pub trait InPlaceEditor {
    /// Modifies the given `toml_edit::DocumentMut` in place.
    fn edit(&self, doc: &mut DocumentMut) -> Result<()>;
}

/// A handle to a TOML file that allows for edits preserving formatting and comments.
///
/// `EditableInPlaceFileHandle` extends the functionality of `FileHandle` by maintaining
/// both a deserialized Rust struct (`model` of type `T`) and a `toml_edit::DocumentMut`
/// (`doc`). This allows modifications to be made directly to the `DocumentMut` via
/// the `TomlEditor` trait, preserving the original TOML structure, including comments
/// and whitespace. After an edit, the `doc` is written to disk, and the `model` is
/// updated by re-parsing the `doc`.
pub struct EditableInPlaceFileHandle<T>
where
    T: Clone + Serialize + DeserializeOwned,
{
    fs: Arc<dyn FileSystem>,
    abs_path: Arc<Path>,
    doc: RwLock<DocumentMut>,
    model: RwLock<T>,
}

impl<T> EditableInPlaceFileHandle<T>
where
    T: Clone + Serialize + DeserializeOwned,
{
    pub async fn create(
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
        let doc_lock = self.doc.read().await;
        let model_from_doc: T = toml::from_str(&doc_lock.to_string())?;
        let mut model_write_lock = self.model.write().await;
        *model_write_lock = model_from_doc;

        Ok(())
    }

    /// Edits the TOML file using a `TomlEditor` implementation.
    ///
    /// The `modifier`'s `edit` method is called with a mutable reference to the internal
    /// `DocumentMut`. After the modification, the `DocumentMut` is serialized to a string
    /// (preserving formatting) and written to the file. The in-memory model (`T`) is then
    /// updated by parsing this new string content.
    pub async fn edit<M: InPlaceEditor>(&self, modifier: M) -> Result<()> {
        let mut doc_lock = self.doc.write().await;
        modifier.edit(&mut *doc_lock)?;
        let content = doc_lock.to_string();
        drop(doc_lock); // Release lock before async file operation

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

    pub fn path(&self) -> &Arc<Path> {
        &self.abs_path
    }
}
