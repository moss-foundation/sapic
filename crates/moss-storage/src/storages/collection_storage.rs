pub mod entities;
pub mod stores;
mod tables;

use anyhow::Result;
use arc_swap::ArcSwap;
use async_trait::async_trait;
use entities::variable_store_entities::VariableEntity;
use moss_db::primitives::AnyValue;
use moss_db::{ClientState, DatabaseClient, DatabaseResult, ReDbClient, Transaction};
use std::collections::HashMap;
use std::path::Path;
use std::pin::Pin;
use std::sync::Arc;
use stores::variable_store::VariableStoreImpl;
use tables::{TABLE_VARIABLES, UNIT_STORE};
use tokio::sync::Notify;

use crate::storage::ResettableStorage;
use crate::{
    CollectionStorage,
    common::item_store::{ItemStore, ItemStoreImpl},
    primitives::segkey::SegKeyBuf,
    storage::Transactional,
};

const DB_NAME: &str = "state.db";

// <environment_name>:<variable_name>
pub trait VariableStore: Send + Sync {
    fn list_variables(&self) -> DatabaseResult<HashMap<SegKeyBuf, VariableEntity>>;
}

pub struct CollectionResettableCell {
    db_client: ReDbClient,
    variable_store: Arc<dyn VariableStore>,
    unit_store: Arc<dyn ItemStore<SegKeyBuf, AnyValue>>,
}

impl CollectionResettableCell {
    pub fn new(path: &Path) -> Result<Self> {
        let db_client = ReDbClient::new(path.join(DB_NAME))?
            .with_table(&UNIT_STORE)?
            .with_table(&TABLE_VARIABLES)?;

        let variable_store = Arc::new(VariableStoreImpl::new(db_client.clone()));
        let unit_store = Arc::new(ItemStoreImpl::new(db_client.clone(), UNIT_STORE));

        Ok(Self {
            db_client,
            variable_store,
            unit_store,
        })
    }
}

pub struct CollectionStorageImpl {
    state: ArcSwap<ClientState<CollectionResettableCell>>,
}

impl CollectionStorageImpl {
    pub fn new(path: &Path) -> Result<Self> {
        let cell = CollectionResettableCell::new(path)?;

        Ok(Self {
            state: ArcSwap::new(Arc::new(ClientState::Loaded(cell))),
        })
    }
}

#[async_trait]
impl Transactional for CollectionStorageImpl {
    async fn begin_write(&self) -> DatabaseResult<Transaction> {
        loop {
            match self.state.load().as_ref() {
                ClientState::Loaded(cell) => return cell.db_client.begin_write(),
                ClientState::Reloading { notify } => notify.notified().await,
            }
        }
    }

    async fn begin_read(&self) -> DatabaseResult<Transaction> {
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
    async fn variable_store(&self) -> Arc<dyn VariableStore> {
        loop {
            match self.state.load().as_ref() {
                ClientState::Loaded(cell) => return cell.variable_store.clone(),
                ClientState::Reloading { notify } => notify.notified().await,
            }
        }
    }

    async fn unit_store(&self) -> Arc<dyn ItemStore<SegKeyBuf, AnyValue>> {
        loop {
            match self.state.load().as_ref() {
                ClientState::Loaded(cell) => return cell.unit_store.clone(),
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

        let new_cell = CollectionResettableCell::new(path)?;
        let new_state = Arc::new(ClientState::Loaded(new_cell));
        self.state.store(new_state);

        // Notify waiting operations
        local_notify.notify_waiters();
        Ok(())
    }
}
