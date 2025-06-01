use anyhow::{Context as _, Result};
use moss_fs::FileSystem;
use std::{path::Path, sync::Arc};
use tokio::sync::RwLock;

pub struct FileHandle<T>
where
    T: Clone,
{
    fs: Arc<dyn FileSystem>,
    abs_path: Arc<Path>,
    model: RwLock<T>,
}

impl<T> FileHandle<T>
where
    T: Clone,
{
    pub async fn create(
        fs: Arc<dyn FileSystem>,
        abs_path: &Path,
        model: T,
        to_string: impl FnOnce(&T) -> Result<String>,
    ) -> Result<Self> {
        let abs_path: Arc<Path> = abs_path.into();
        debug_assert!(abs_path.is_absolute());

        let s = to_string(&model)?;
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
            model: RwLock::new(model),
        })
    }

    pub async fn load(
        fs: Arc<dyn FileSystem>,
        abs_path: &Path,
        from_string: impl FnOnce(&str) -> Result<T>,
    ) -> Result<Self> {
        let abs_path: Arc<Path> = abs_path.into();
        debug_assert!(abs_path.is_absolute());

        let mut reader = fs
            .open_file(&abs_path)
            .await
            .context("Failed to open file")?;

        let mut buf = String::new();
        reader
            .read_to_string(&mut buf)
            .context("Failed to read file")?;

        let model: T = from_string(&buf)?;

        Ok(Self {
            fs,
            abs_path,
            model: RwLock::new(model),
        })
    }

    pub async fn model(&self) -> T {
        self.model.read().await.clone()
    }

    pub fn path(&self) -> &Arc<Path> {
        &self.abs_path
    }

    pub async fn edit(
        &self,
        f: impl FnOnce(&mut T) -> Result<()>,
        to_string: impl FnOnce(&T) -> Result<String>,
    ) -> Result<()> {
        let mut model_lock = self.model.write().await;

        // We need to clone the model here because we don't want change the original model
        // in place, because we can fail to write the file and we don't want to leave the
        // model in an inconsistent state.
        let mut model_clone = model_lock.clone();
        f(&mut model_clone)?;

        let s = to_string(&model_clone)?;
        self.fs
            .create_file_with(
                &self.abs_path,
                &s.as_bytes(),
                moss_fs::CreateOptions {
                    overwrite: true,
                    ignore_if_exists: true,
                },
            )
            .await?;

        *model_lock = model_clone;

        Ok(())
    }
}
