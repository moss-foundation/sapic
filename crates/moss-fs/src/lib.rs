pub mod error;
pub mod fs_watcher;
pub mod real;
pub mod utils;

pub use error::*;
pub use real::*;
pub use utils::{desanitize_path, normalize_path, sanitize_path};

use futures::stream::BoxStream;
use std::{io, path::Path, time::Duration};
use tokio::fs::ReadDir;

// TODO: Rename to RemoveParams
#[derive(Copy, Clone, Default)]
pub struct RemoveOptions {
    pub recursive: bool,
    pub ignore_if_not_exists: bool,
}

// TODO: Rename to CreateParams
#[derive(Copy, Clone)]
pub struct CreateOptions {
    pub overwrite: bool,
    pub ignore_if_exists: bool,
}

impl Default for CreateOptions {
    fn default() -> Self {
        Self {
            overwrite: true,
            ignore_if_exists: false,
        }
    }
}

// TODO: Rename to RenameParams
#[derive(Copy, Clone)]
pub struct RenameOptions {
    pub overwrite: bool,
    pub ignore_if_exists: bool,
}

impl Default for RenameOptions {
    fn default() -> Self {
        Self {
            overwrite: true,
            ignore_if_exists: false,
        }
    }
}

#[async_trait::async_trait]
pub trait FileSystem: Send + Sync {
    async fn create_dir_all(&self, path: &Path) -> FsResult<()>;
    async fn create_dir(&self, path: &Path) -> FsResult<()>;
    async fn read_dir(&self, path: &Path) -> FsResult<ReadDir>;
    async fn remove_dir(&self, path: &Path, options: RemoveOptions) -> FsResult<()>;

    async fn rename(&self, from: &Path, to: &Path, options: RenameOptions) -> FsResult<()>;

    async fn create_file(&self, path: &Path, options: CreateOptions) -> FsResult<()>;

    async fn create_file_with(
        &self,
        path: &Path,
        content: &[u8],
        options: CreateOptions,
    ) -> FsResult<()>;
    async fn remove_file(&self, path: &Path, options: RemoveOptions) -> FsResult<()>;
    async fn open_file(&self, path: &Path) -> FsResult<Box<dyn io::Read + Send + Sync>>;

    async fn zip_dir(
        &self,
        src_dir: &Path,
        out_file: &Path,
        excluded_entries: Vec<String>,
    ) -> FsResult<()>;

    async fn unzip_dir(&self, src_archive: &Path, out_dir: &Path) -> FsResult<()>;

    fn watch(
        &self,
        path: &Path,
        latency: Duration,
    ) -> FsResult<(
        BoxStream<'static, Vec<notify::Event>>,
        notify::RecommendedWatcher,
    )>;
}
