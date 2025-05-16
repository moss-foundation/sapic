use anyhow::Result;
use arc_swap::ArcSwap;
use async_trait::async_trait;
use moss_db::{
    AnyEntity, ClientState, DatabaseClient, ReDbClient, Transaction,
    bincode_table::BincodeTable,
    common::{DatabaseError, DatabaseResult},
};

use std::{
    collections::HashMap,
    future::Future,
    path::{Path, PathBuf},
    pin::Pin,
    sync::Arc,
};
use tokio::sync::Notify;

use crate::storage::{ResettableStorage, Transactional};

use super::{
    CollectionStorage,
    entities::{
        request_store_entities::RequestNodeEntity, state_store_entries::WorktreeEntryEntity,
    },
    request_store::{RequestStoreImpl, TABLE_REQUESTS},
    state_store::StateStoreImpl,
};

const DB_NAME: &str = "state.db";

pub(crate) type RequestStoreTable<'a> = BincodeTable<'a, String, RequestNodeEntity>;
pub trait RequestStore: Send + Sync + 'static {
    fn list_request_nodes(&self) -> DatabaseResult<HashMap<PathBuf, RequestNodeEntity>>;
    fn upsert_request_node(
        &self,
        txn: &mut Transaction,
        path: PathBuf,
        node: RequestNodeEntity,
    ) -> DatabaseResult<()>;
    fn rekey_request_node(
        &self,
        txn: &mut Transaction,
        old_path: PathBuf,
        new_path: PathBuf,
    ) -> DatabaseResult<()>;
    fn delete_request_node(&self, txn: &mut Transaction, path: PathBuf) -> DatabaseResult<()>;
}

pub(crate) type StateStoreTable<'a> = BincodeTable<'a, String, AnyEntity>;

pub trait StateStore: Send + Sync + 'static {
    fn list_worktree_entries(&self) -> DatabaseResult<Vec<WorktreeEntryEntity>>;
    fn upsert_worktree_entry(
        &self,
        txn: &mut Transaction,
        entry: WorktreeEntryEntity,
    ) -> DatabaseResult<()>;
}

struct ResettableStorageCell {
    db_client: ReDbClient,
    request_store: Arc<dyn RequestStore>,
    state_store: Arc<dyn StateStore>,
}

impl ResettableStorageCell {
    pub fn new(path: &Path) -> Result<Self> {
        let db_client = ReDbClient::new(path.join(DB_NAME))?.with_table(&TABLE_REQUESTS)?;

        let request_store = Arc::new(RequestStoreImpl::new(db_client.clone()));
        let state_store = Arc::new(StateStoreImpl::new(db_client.clone()));
        Ok(Self {
            db_client,
            request_store,
            state_store,
        })
    }
}

pub struct CollectionStorageImpl {
    state: ArcSwap<ClientState<ResettableStorageCell>>,
}

impl CollectionStorageImpl {
    pub fn new(path: &Path) -> Result<Self> {
        let cell = ResettableStorageCell::new(path)?;

        Ok(Self {
            state: ArcSwap::new(Arc::new(ClientState::Loaded(cell))),
        })
    }
}

#[async_trait]
impl Transactional for CollectionStorageImpl {
    async fn begin_write(&self) -> Result<Transaction, DatabaseError> {
        loop {
            match self.state.load().as_ref() {
                ClientState::Loaded(cell) => return cell.db_client.begin_write(),
                ClientState::Reloading { notify } => notify.notified().await,
            }
        }
    }

    async fn begin_read(&self) -> Result<Transaction, DatabaseError> {
        loop {
            match self.state.load().as_ref() {
                ClientState::Loaded(cell) => return cell.db_client.begin_read(),
                ClientState::Reloading { notify } => notify.notified().await,
            }
        }
    }
}

#[async_trait]
impl CollectionStorage for CollectionStorageImpl {
    async fn request_store(&self) -> Arc<dyn RequestStore> {
        loop {
            match self.state.load().as_ref() {
                ClientState::Loaded(cell) => return cell.request_store.clone(),
                ClientState::Reloading { notify } => notify.notified().await,
            }
        }
    }

    async fn state_store(&self) -> Arc<dyn StateStore> {
        loop {
            match self.state.load().as_ref() {
                ClientState::Loaded(cell) => return cell.state_store.clone(),
                ClientState::Reloading { notify } => notify.notified().await,
            }
        }
    }
}

#[async_trait]
impl ResettableStorage for CollectionStorageImpl {
    async fn reset(
        &self,
        path: &Path,
        after_drop: Pin<Box<dyn Future<Output = Result<()>> + Send>>,
    ) -> Result<()> {
        let local_notify = Arc::new(Notify::new());
        let reloading_state = Arc::new(ClientState::Reloading {
            notify: local_notify.clone(),
        });
        let old_state = self.state.swap(reloading_state);

        // Wait for current operations to complete
        tokio::task::yield_now().await;
        drop(old_state);

        after_drop.await?;

        let new_cell = ResettableStorageCell::new(path)?;
        let new_state = Arc::new(ClientState::Loaded(new_cell));
        self.state.store(new_state);

        // Notify waiting operations
        local_notify.notify_waiters();
        Ok(())
    }
}
