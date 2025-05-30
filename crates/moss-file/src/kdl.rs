use anyhow::{Result, anyhow};
use kdl::KdlDocument;
use moss_fs::FileSystem;
use std::{ops::Deref, path::Path, sync::Arc};

use crate::common::FileHandle;

pub struct KdlFileHandle<T>
where
    T: Clone,
{
    inner: FileHandle<T>,
}

impl<T> Deref for KdlFileHandle<T>
where
    T: Clone + Into<KdlDocument>,
{
    type Target = FileHandle<T>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> KdlFileHandle<T>
where
    T: Clone + Into<KdlDocument>,
{
    pub fn new(file: FileHandle<T>) -> Self {
        Self { inner: file }
    }

    // TODO: Pass parsing logic as a callback
    pub async fn load(
        fs: Arc<dyn FileSystem>,
        abs_path: &Path,
        from_string: impl FnOnce(&str) -> Result<T>,
    ) -> Result<Self> {
        let file = FileHandle::load(fs, abs_path, from_string).await?;
        Ok(Self { inner: file })
    }

    pub async fn create(fs: Arc<dyn FileSystem>, abs_path: &Path, model: T) -> Result<Self> {
        let file = FileHandle::create(fs, abs_path, model, |s| {
            // Standardized output formatting
            let document: KdlDocument = s.clone().into();
            Ok(document
                .into_iter()
                .map(|node| node.to_string())
                .collect::<Vec<String>>()
                .join("\n"))
        })
        .await?;
        Ok(Self { inner: file })
    }
}
