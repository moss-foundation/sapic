pub mod error;
pub mod fs_watcher;
pub mod real;
pub mod utils;

pub use error::*;
pub use real::*;
pub use utils::{desanitize_path, normalize_path, sanitize_path};

use futures::stream::BoxStream;
use moss_applib::{GlobalMarker, context_old::Context};
use std::{io, path::Path, sync::Arc, time::Duration};
use tauri::{AppHandle, Manager, Runtime as TauriRuntime};
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
    fn watch(
        &self,
        path: &Path,
        latency: Duration,
    ) -> FsResult<(
        BoxStream<'static, Vec<notify::Event>>,
        notify::RecommendedWatcher,
    )>;
}

pub struct GlobalFileSystem(Arc<dyn FileSystem>);

impl GlobalMarker for GlobalFileSystem {}

impl dyn FileSystem {
    pub fn global<R: TauriRuntime, C: Context<R>>(ctx: &C) -> Arc<Self> {
        ctx.global::<GlobalFileSystem>().0.clone()
    }

    pub fn set_global<R: TauriRuntime>(fs: Arc<Self>, app_handle: &AppHandle<R>) {
        app_handle.manage(GlobalFileSystem(fs));
    }
}
