use anyhow::{anyhow, Result};
use std::{io, path::Path, pin::Pin};
use tokio::{fs::ReadDir, io::AsyncRead};

use crate::ports::{CreateOptions, FileSystem, RemoveOptions, RenameOptions};

pub struct DiskFileSystem;

impl DiskFileSystem {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl FileSystem for DiskFileSystem {
    async fn create_dir(&self, path: &Path) -> Result<()> {
        Ok(tokio::fs::create_dir_all(path).await?)
    }

    async fn remove_dir(&self, path: &Path, options: RemoveOptions) -> Result<()> {
        (if options.recursive {
            tokio::fs::remove_dir_all(path).await
        } else {
            tokio::fs::remove_dir(path).await
        })
        .or_else(|err| {
            if err.kind() == io::ErrorKind::NotFound && options.ignore_if_not_exists {
                Ok(())
            } else {
                Err(err)?
            }
        })
    }

    async fn create_file(&self, path: &Path, options: CreateOptions) -> Result<()> {
        let mut open_options = tokio::fs::OpenOptions::new();
        open_options.write(true).create(true);
        if options.overwrite {
            open_options.truncate(true);
        } else if !options.ignore_if_exists {
            open_options.create_new(true);
        }

        open_options.open(path).await?;

        Ok(())
    }

    async fn create_file_with(
        &self,
        path: &Path,
        content: Pin<&mut (dyn AsyncRead + Send)>,
    ) -> Result<()> {
        let mut file = tokio::fs::File::create(&path).await?;
        tokio::io::copy(content.get_mut(), &mut file).await?;
        Ok(())
    }

    async fn remove_file(&self, path: &Path, options: RemoveOptions) -> Result<()> {
        tokio::fs::remove_file(path).await.or_else(|err| {
            if err.kind() == io::ErrorKind::NotFound && options.ignore_if_not_exists {
                Ok(())
            } else {
                Err(err)?
            }
        })
    }

    async fn open_file(&self, path: &Path) -> Result<Box<dyn io::Read + Send + Sync>> {
        Ok(Box::new(std::fs::File::open(path)?))
    }

    async fn rename(&self, source: &Path, target: &Path, options: RenameOptions) -> Result<()> {
        if !options.overwrite && tokio::fs::metadata(target).await.is_ok() {
            if options.ignore_if_exists {
                return Ok(());
            } else {
                return Err(anyhow!("{target:?} already exists"));
            }
        }

        Ok(tokio::fs::rename(source, target).await?)
    }

    async fn read_dir(&self, path: &Path) -> Result<ReadDir> {
        Ok(tokio::fs::read_dir(path).await?)
    }
}
