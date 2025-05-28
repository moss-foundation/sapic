use anyhow::{Result, anyhow};
use kdl::KdlDocument;
use moss_fs::FileSystem;
use moss_kdl::spec_file::{SpecificationFile, SpecificationFileType};
use std::ops::Deref;
use std::path::Path;
use std::sync::Arc;

use crate::common::FileHandle;
use crate::tokens::{DELETE_SPECFILE, FOLDER_SPECFILE, GET_SPECFILE, POST_SPECFILE, PUT_SPECFILE};

// kdl crate does not follow the serde serialization model
pub struct KdlFileHandle<T>
where
    T: Clone + Into<KdlDocument>,
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
