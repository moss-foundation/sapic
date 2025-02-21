pub mod adapters;

use anyhow::Result;
use std::{io, path::Path};
use tokio::fs::ReadDir;

#[derive(Copy, Clone, Default)]
pub struct RemoveOptions {
    pub recursive: bool,
    pub ignore_if_not_exists: bool,
}

#[derive(Copy, Clone, Default)]
pub struct CreateOptions {
    pub overwrite: bool,
    pub ignore_if_exists: bool,
}

#[derive(Copy, Clone, Default)]
pub struct RenameOptions {
    pub overwrite: bool,
    pub ignore_if_exists: bool,
}

#[async_trait::async_trait]
pub trait FileSystem: Send + Sync {
    async fn create_dir(&self, path: &Path) -> Result<()>;
    async fn read_dir(&self, path: &Path) -> Result<ReadDir>;
    async fn remove_dir(&self, path: &Path, options: RemoveOptions) -> Result<()>;

    async fn rename(&self, source: &Path, target: &Path, options: RenameOptions) -> Result<()>;

    async fn create_file(&self, path: &Path, options: CreateOptions) -> Result<()>;
    async fn remove_file(&self, path: &Path, options: RemoveOptions) -> Result<()>;
    async fn open_file(&self, path: &Path) -> Result<Box<dyn io::Read + Send + Sync>>;
}
