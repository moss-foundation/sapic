pub mod fs_watcher;
pub mod real;
pub mod utils;

pub use real::*;
pub use utils::{desanitize_path, normalize_path, sanitize_path};

use atomic_fs::Rollback;
use futures::stream::BoxStream;
use sapic_core::context::AnyAsyncContext;
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

// Swapping FsResult to joinerror::Result to make context easier to work with
#[async_trait::async_trait]
pub trait FileSystem: Send + Sync {
    async fn create_dir_all(&self, ctx: &dyn AnyAsyncContext, path: &Path)
    -> joinerror::Result<()>;
    async fn create_dir(&self, ctx: &dyn AnyAsyncContext, path: &Path) -> joinerror::Result<()>;
    async fn read_dir(&self, ctx: &dyn AnyAsyncContext, path: &Path) -> joinerror::Result<ReadDir>;
    async fn remove_dir(
        &self,
        ctx: &dyn AnyAsyncContext,
        path: &Path,
        options: RemoveOptions,
    ) -> joinerror::Result<()>;
    async fn is_dir_empty(&self, ctx: &dyn AnyAsyncContext, path: &Path)
    -> joinerror::Result<bool>;

    async fn rename(
        &self,
        ctx: &dyn AnyAsyncContext,
        from: &Path,
        to: &Path,
        options: RenameOptions,
    ) -> joinerror::Result<()>;

    async fn create_file(
        &self,
        ctx: &dyn AnyAsyncContext,
        path: &Path,
        options: CreateOptions,
    ) -> joinerror::Result<()>;

    async fn create_file_with(
        &self,
        ctx: &dyn AnyAsyncContext,
        path: &Path,
        content: &[u8],
        options: CreateOptions,
    ) -> joinerror::Result<()>;
    async fn remove_file(
        &self,
        ctx: &dyn AnyAsyncContext,
        path: &Path,
        options: RemoveOptions,
    ) -> joinerror::Result<()>;
    async fn open_file(
        &self,
        ctx: &dyn AnyAsyncContext,
        path: &Path,
    ) -> joinerror::Result<Box<dyn io::Read + Send + Sync>>;

    async fn zip(
        &self,
        ctx: &dyn AnyAsyncContext,
        src_dir: &Path,
        out_file: &Path,
        excluded_entries: &[&str],
    ) -> joinerror::Result<()>;

    async fn unzip(
        &self,
        ctx: &dyn AnyAsyncContext,
        src_archive: &Path,
        out_dir: &Path,
    ) -> joinerror::Result<()>;

    fn watch(
        &self,
        path: &Path,
        latency: Duration,
    ) -> joinerror::Result<(
        BoxStream<'static, Vec<notify::Event>>,
        notify::RecommendedWatcher,
    )>;

    async fn start_rollback(&self, ctx: &dyn AnyAsyncContext) -> joinerror::Result<Rollback>;

    async fn create_dir_with_rollback(
        &self,
        ctx: &dyn AnyAsyncContext,
        rb: &mut Rollback,
        path: &Path,
    ) -> joinerror::Result<()>;

    async fn create_dir_all_with_rollback(
        &self,
        ctx: &dyn AnyAsyncContext,
        rb: &mut Rollback,
        path: &Path,
    ) -> joinerror::Result<()>;

    async fn remove_dir_with_rollback(
        &self,
        ctx: &dyn AnyAsyncContext,
        rb: &mut Rollback,
        path: &Path,
        options: RemoveOptions,
    ) -> joinerror::Result<()>;

    async fn create_file_with_rollback(
        &self,
        ctx: &dyn AnyAsyncContext,
        rb: &mut Rollback,
        path: &Path,
        options: CreateOptions,
    ) -> joinerror::Result<()>;

    async fn create_file_with_content_with_rollback(
        &self,
        ctx: &dyn AnyAsyncContext,
        rb: &mut Rollback,
        path: &Path,
        content: &[u8],
        options: CreateOptions,
    ) -> joinerror::Result<()>;

    async fn remove_file_with_rollback(
        &self,
        ctx: &dyn AnyAsyncContext,
        rb: &mut Rollback,
        path: &Path,
        options: RemoveOptions,
    ) -> joinerror::Result<()>;

    async fn rename_with_rollback(
        &self,
        ctx: &dyn AnyAsyncContext,
        rb: &mut Rollback,
        from: &Path,
        to: &Path,
        options: RenameOptions,
    ) -> joinerror::Result<()>;
}
