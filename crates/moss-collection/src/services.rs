pub mod set_icon_service;
pub mod storage_service;
pub mod worktree_service;

pub use set_icon_service::SetIconService;
pub use storage_service::StorageService;
pub use worktree_service::WorktreeService;

use anyhow::Result;
use async_trait::async_trait;
use derive_more::Deref;
use moss_applib::{AppRuntime, ServiceMarker};
use moss_db::{Transaction, primitives::AnyValue};
use moss_storage::primitives::segkey::SegKeyBuf;
use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::mpsc;

use crate::{
    models::{
        primitives::EntryId,
        types::configuration::docschema::{RawDirConfiguration, RawItemConfiguration},
    },
    services::worktree_service::{EntryDescription, EntryMetadata, ModifyParams, WorktreeResult},
};

// ########################################################
// ###               Set Icon Service                   ###
// ########################################################
#[async_trait]
pub trait AnySetIconService: Send + Sync + ServiceMarker + 'static {
    fn set_icon(&self, img_path: &Path) -> Result<()>;
    async fn remove_icon(&self) -> Result<()>;
    fn icon_path(&self) -> Option<PathBuf>;
}

#[derive(Deref)]
pub struct DynSetIconService(Arc<dyn AnySetIconService>);

impl DynSetIconService {
    pub fn new(service: Arc<dyn AnySetIconService>) -> Arc<Self> {
        Arc::new(Self(service))
    }
}

impl ServiceMarker for DynSetIconService {}

// ########################################################
// ###                Storage Service                   ###
// ########################################################

#[async_trait]
pub trait AnyStorageService<R: AppRuntime>: Send + Sync + ServiceMarker + 'static {
    async fn begin_write(&self, ctx: &R::AsyncContext) -> anyhow::Result<Transaction>;
    async fn begin_read(&self, ctx: &R::AsyncContext) -> anyhow::Result<Transaction>;
    async fn put_entry_order_txn(
        &self,
        ctx: &R::AsyncContext,
        txn: &mut Transaction,
        id: &EntryId,
        order: isize,
    ) -> anyhow::Result<()>;
    async fn get_all_entry_keys(
        &self,
        ctx: &R::AsyncContext,
    ) -> anyhow::Result<HashMap<SegKeyBuf, AnyValue>>;

    async fn put_expanded_entries(
        &self,
        ctx: &R::AsyncContext,
        expanded_entries: Vec<EntryId>,
    ) -> anyhow::Result<()>;

    async fn put_expanded_entries_txn(
        &self,
        ctx: &R::AsyncContext,
        txn: &mut Transaction,
        expanded_entries: Vec<EntryId>,
    ) -> anyhow::Result<()>;

    async fn get_expanded_entries(&self, ctx: &R::AsyncContext) -> anyhow::Result<Vec<EntryId>>;
}

#[derive(Deref)]
pub struct DynStorageService<R: AppRuntime>(Arc<dyn AnyStorageService<R>>);

impl<R: AppRuntime> DynStorageService<R> {
    pub fn new(service: Arc<dyn AnyStorageService<R>>) -> Arc<Self> {
        Arc::new(Self(service))
    }
}

impl<R: AppRuntime + 'static> ServiceMarker for DynStorageService<R> {}

// ########################################################
// ###               Worktree Service                   ###
// ########################################################

#[async_trait]
pub trait AnyWorktreeService<R: AppRuntime>: Send + Sync + ServiceMarker + 'static {
    fn absolutize(&self, path: &Path) -> WorktreeResult<PathBuf>;
    async fn remove_entry(&self, ctx: &R::AsyncContext, id: &EntryId) -> WorktreeResult<()>;
    async fn scan(
        &self,
        ctx: &R::AsyncContext,
        path: &Path,
        expanded_entries: Arc<HashSet<EntryId>>,
        all_entry_keys: Arc<HashMap<SegKeyBuf, AnyValue>>,
        sender: mpsc::UnboundedSender<EntryDescription>,
    ) -> WorktreeResult<()>;
    async fn create_item_entry(
        &self,
        ctx: &R::AsyncContext,
        id: &EntryId,
        name: &str,
        path: &Path,
        configuration: RawItemConfiguration,
        metadata: EntryMetadata,
    ) -> WorktreeResult<()>;
    async fn create_dir_entry(
        &self,
        ctx: &R::AsyncContext,
        id: &EntryId,
        name: &str,
        path: &Path,
        configuration: RawDirConfiguration,
        metadata: EntryMetadata,
    ) -> WorktreeResult<()>;
    async fn update_dir_entry(
        &self,
        ctx: &R::AsyncContext,
        id: &EntryId,
        params: ModifyParams,
    ) -> WorktreeResult<(PathBuf, RawDirConfiguration)>;
    async fn update_item_entry(
        &self,
        ctx: &R::AsyncContext,
        id: &EntryId,
        params: ModifyParams,
    ) -> WorktreeResult<(PathBuf, RawItemConfiguration)>;
}

#[derive(Deref)]
pub struct DynWorktreeService<R: AppRuntime>(Arc<dyn AnyWorktreeService<R>>);
impl<R: AppRuntime> DynWorktreeService<R> {
    pub fn new(service: Arc<dyn AnyWorktreeService<R>>) -> Arc<Self> {
        Arc::new(Self(service))
    }
}

impl<R: AppRuntime> Clone for DynWorktreeService<R> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<R: AppRuntime + 'static> ServiceMarker for DynWorktreeService<R> {}
