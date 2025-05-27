use anyhow::Result;
use moss_fs::FileSystem;
use serde::{Serialize, de::DeserializeOwned};
use std::{ops::Deref, path::Path, sync::Arc};

use crate::common::FileHandle;

pub struct JsonFileHandle<T>
where
    T: Clone + Serialize + DeserializeOwned,
{
    inner: FileHandle<T>,
}

impl<T> Deref for JsonFileHandle<T>
where
    T: Clone + Serialize + DeserializeOwned,
{
    type Target = FileHandle<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> JsonFileHandle<T>
where
    T: Clone + Serialize + DeserializeOwned,
{
    pub fn new(file: FileHandle<T>) -> Self {
        Self { inner: file }
    }

    pub async fn load(fs: Arc<dyn FileSystem>, abs_path: &Path) -> Result<Self> {
        let file = FileHandle::load(fs, abs_path, |s| {
            serde_json::from_str(s)
                .map_err(|err| anyhow::anyhow!("Failed to parse JSON file: {}", err))
        })
        .await?;
        Ok(Self { inner: file })
    }

    pub async fn create(fs: Arc<dyn FileSystem>, abs_path: &Path, model: T) -> Result<Self> {
        let file = FileHandle::create(fs, abs_path, model, |s| {
            serde_json::to_string(s)
                .map_err(|err| anyhow::anyhow!("Failed to serialize JSON file: {}", err))
        })
        .await?;
        Ok(Self { inner: file })
    }
}
