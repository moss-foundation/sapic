pub mod entities;
pub mod request_store;

use anyhow::Result;
use arc_swap::ArcSwap;
use async_trait::async_trait;
use entities::request_store_entities::RequestNodeEntity;
use moss_db::{bincode_table::BincodeTable, common::Transaction, ClientState, ReDbClient};
use request_store::RequestStoreImpl;

use std::{
    collections::HashMap,
    future::Future,
    path::{Path, PathBuf},
    pin::Pin,
    sync::Arc,
};
use tokio::sync::Notify;

use crate::{CollectionStorage, ResettableStorage};

const COLLECTION_STATE_DB_NAME: &str = "state.db";

pub(crate) type RequestStoreTable<'a> = BincodeTable<'a, String, RequestNodeEntity>;
pub trait RequestStore: Send + Sync + 'static {
    fn begin_write(&self) -> Result<(Transaction, &RequestStoreTable)>;
    fn begin_read(&self) -> Result<(Transaction, &RequestStoreTable)>;
    fn scan(&self) -> Result<HashMap<PathBuf, RequestNodeEntity>>;
}

struct DbManagerCell {
    request_store: Arc<dyn RequestStore>,
}

impl DbManagerCell {
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let db_client = ReDbClient::new(path.as_ref().join(COLLECTION_STATE_DB_NAME))?
            .with_table(&request_store::TABLE_REQUESTS)?;

        let request_store = Arc::new(RequestStoreImpl::new(db_client));
        Ok(Self { request_store })
    }
}

pub struct CollectionStorageImpl {
    state: ArcSwap<ClientState<DbManagerCell>>,
}

impl CollectionStorageImpl {
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let cell = DbManagerCell::new(path)?;

        Ok(Self {
            state: ArcSwap::new(Arc::new(ClientState::Loaded(cell))),
        })
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
}

#[async_trait]
impl ResettableStorage for CollectionStorageImpl {
    async fn reset(
        &self,
        new_path: PathBuf,
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

        let new_cell = DbManagerCell::new(new_path)?;
        let new_state = Arc::new(ClientState::Loaded(new_cell));
        self.state.store(new_state);

        // Notify waiting operations
        local_notify.notify_waiters();
        Ok(())
    }
}
