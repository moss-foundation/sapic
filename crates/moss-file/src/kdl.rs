use std::{path::Path, sync::Arc};

use crate::common::FileHandle;
use anyhow::Result;
use kdl::KdlDocument;
use moss_fs::FileSystem;

fn from_str<T>(s: &str) -> Result<T>
where
    T: From<KdlDocument>,
{
    let document = KdlDocument::parse(s)?;

    Ok(document.into())
}

fn to_string<T>(model: T) -> Result<String>
where
    T: Into<KdlDocument>,
{
    let document: KdlDocument = model.into();
    Ok(document.to_string())
}

pub struct KdlFileHandle<T>
where
    T: Clone + From<KdlDocument> + Into<KdlDocument>,
{
    inner: FileHandle<T>,
}

impl<T> KdlFileHandle<T>
where
    T: Clone + From<KdlDocument> + Into<KdlDocument>,
{
    pub fn new(file: FileHandle<T>) -> Self {
        Self { inner: file }
    }

    pub async fn load(fs: Arc<dyn FileSystem>, abs_path: &Path) -> Result<Self> {
        let file = FileHandle::load(fs, abs_path, |s| from_str(s)).await?;
        Ok(Self { inner: file })
    }

    pub async fn create(fs: Arc<dyn FileSystem>, abs_path: &Path, model: T) -> Result<Self> {
        let file = FileHandle::create(fs, abs_path, model, |s| to_string(s.clone())).await?;
        Ok(Self { inner: file })
    }
}
