pub mod set_icon_service;
pub mod storage_service;
pub mod worktree_service;

pub use storage_service::StorageService;
pub use worktree_service::WorktreeService;

use async_trait::async_trait;
use derive_more::Deref;
use moss_applib::ServiceMarker;
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
// ###                Storage Service                   ###
// ########################################################
pub trait AnyStorageService: Send + Sync + ServiceMarker + 'static {
    fn begin_write(&self) -> anyhow::Result<Transaction>;
    fn begin_read(&self) -> anyhow::Result<Transaction>;
    fn put_entry_order_txn(
        &self,
        txn: &mut Transaction,
        id: &EntryId,
        order: isize,
    ) -> anyhow::Result<()>;
    fn get_all_entry_keys(&self) -> anyhow::Result<HashMap<SegKeyBuf, AnyValue>>;

    fn put_expanded_entries(&self, expanded_entries: Vec<EntryId>) -> anyhow::Result<()>;

    fn put_expanded_entries_txn(
        &self,
        txn: &mut Transaction,
        expanded_entries: Vec<EntryId>,
    ) -> anyhow::Result<()>;

    fn get_expanded_entries(&self) -> anyhow::Result<Vec<EntryId>>;
}

#[derive(Deref)]
pub struct DynStorageService(Arc<dyn AnyStorageService>);

impl DynStorageService {
    pub fn new(service: Arc<dyn AnyStorageService>) -> Arc<Self> {
        Arc::new(Self(service))
    }
}

impl ServiceMarker for DynStorageService {}

// ########################################################
// ###               Worktree Service                   ###
// ########################################################

#[async_trait]
pub trait AnyWorktreeService: Send + Sync + ServiceMarker + 'static {
    fn absolutize(&self, path: &Path) -> WorktreeResult<PathBuf>;
    async fn remove_entry(&self, id: &EntryId) -> WorktreeResult<()>;
    async fn scan(
        &self,
        path: &Path,
        expanded_entries: Arc<HashSet<EntryId>>,
        all_entry_keys: Arc<HashMap<SegKeyBuf, AnyValue>>,
        sender: mpsc::UnboundedSender<EntryDescription>,
    ) -> WorktreeResult<()>;
    async fn create_item_entry(
        &self,
        id: &EntryId,
        name: &str,
        path: &Path,
        configuration: RawItemConfiguration,
        metadata: EntryMetadata,
    ) -> WorktreeResult<()>;
    async fn create_dir_entry(
        &self,
        id: &EntryId,
        name: &str,
        path: &Path,
        configuration: RawDirConfiguration,
        metadata: EntryMetadata,
    ) -> WorktreeResult<()>;
    async fn update_dir_entry(
        &self,
        id: &EntryId,
        params: ModifyParams,
    ) -> WorktreeResult<(PathBuf, RawDirConfiguration)>;
    async fn update_item_entry(
        &self,
        id: &EntryId,
        params: ModifyParams,
    ) -> WorktreeResult<(PathBuf, RawItemConfiguration)>;
}

#[derive(Deref)]
pub struct DynWorktreeService(Arc<dyn AnyWorktreeService>);
impl DynWorktreeService {
    pub fn new(service: Arc<dyn AnyWorktreeService>) -> Arc<Self> {
        Arc::new(Self(service))
    }
}

impl ServiceMarker for DynWorktreeService {}
