use anyhow::{anyhow, Result};
use futures::{Stream, StreamExt};
use std::{
    io,
    path::{Path, PathBuf},
    pin::Pin,
};
use tokio_stream::wrappers::ReadDirStream;

use crate::{CreateOptions, FileSystem, RemoveOptions, RenameOptions};

pub struct DickFileSystem;

impl DickFileSystem {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl FileSystem for DickFileSystem {
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

    async fn read_dir(
        &self,
        path: &Path,
    ) -> Result<Pin<Box<dyn Send + Stream<Item = Result<PathBuf>>>>> {
        let read_dir = tokio::fs::read_dir(path).await?;

        let stream = ReadDirStream::new(read_dir).map(|entry| match entry {
            Ok(entry) => Ok(entry.path()),
            Err(error) => Err(anyhow!("failed to read dir entry: {:?}", error)),
        });

        Ok(Box::pin(stream))
    }
}
