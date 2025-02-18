pub mod adapters;

use anyhow::{anyhow, Context as _, Result};
use std::{path::Path, pin::Pin};

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

#[async_trait::async_trait]
pub trait FileSystem: Send + Sync {
    async fn create_dir(&self, path: &Path) -> Result<()>;
    async fn remove_dir(&self, path: &Path, options: RemoveOptions) -> Result<()>;

    async fn create_file(&self, path: &Path, options: CreateOptions) -> Result<()>;
    async fn remove_file(&self, path: &Path, options: RemoveOptions) -> Result<()>;
}
