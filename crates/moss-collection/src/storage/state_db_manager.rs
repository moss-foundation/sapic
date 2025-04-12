use anyhow::Result;
use arc_swap::ArcSwap;
use async_trait::async_trait;
use moss_db::{ClientState, ReDbClient};
use std::{
    future::Future,
    path::{Path, PathBuf},
    pin::Pin,
    sync::Arc,
};
use tokio::sync::Notify;

use super::request_store::TABLE_REQUESTS;
use super::{request_store::RequestStoreImpl, RequestStore, StateDbManager};

const COLLECTION_STATE_DB_NAME: &str = "state.db";

struct DbManagerCell {
    request_store: Arc<dyn RequestStore>,
}

impl DbManagerCell {
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let db_client = ReDbClient::new(path.as_ref().join(COLLECTION_STATE_DB_NAME))?
            .with_table(&TABLE_REQUESTS)?;

        let request_store = Arc::new(RequestStoreImpl::new(db_client));
        Ok(Self { request_store })
    }
}

pub struct StateDbManagerImpl {
    state: ArcSwap<ClientState<DbManagerCell>>,
}

impl StateDbManagerImpl {
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let cell = DbManagerCell::new(path)?;

        Ok(Self {
            state: ArcSwap::new(Arc::new(ClientState::Loaded(cell))),
        })
    }
}

#[async_trait]
impl StateDbManager for StateDbManagerImpl {
    async fn reload(
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

    async fn request_store(&self) -> Arc<dyn RequestStore> {
        loop {
            match self.state.load().as_ref() {
                ClientState::Loaded(cell) => return cell.request_store.clone(),
                ClientState::Reloading { notify } => notify.notified().await,
            }
        }
    }
}
