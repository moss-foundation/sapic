pub mod collection_store;
pub mod entities;
pub mod environment_store;
pub mod state_store;

use anyhow::Result;
use async_trait::async_trait;
use collection_store::CollectionStoreImpl;
use entities::{
    collection_store_entities::CollectionEntity,
    environment_store_entities::EnvironmentEntity,
    state_store_entities::{EditorPartStateEntity, PanelPartStateEntity, SidebarPartStateEntity},
};
use environment_store::EnvironmentStoreImpl;
use moss_db::{
    bincode_table::BincodeTable,
    common::{AnyEntity, DatabaseError, Transaction},
    DatabaseClient, ReDbClient,
};
use state_store::StateStoreImpl;
use std::{collections::HashMap, path::PathBuf, sync::Arc};

use crate::{common::Transactional, WorkspaceStorage};

const WORKSPACE_STATE_DB_NAME: &str = "state.db";

pub(crate) type CollectionStoreTable<'a> = BincodeTable<'a, String, CollectionEntity>;
pub trait CollectionStore: Send + Sync {
    fn upsert_collection(
        &self,
        txn: &mut Transaction,
        path: PathBuf,
        entity: CollectionEntity,
    ) -> Result<(), DatabaseError>;
    fn rekey_collection(
        &self,
        txn: &mut Transaction,
        old_path: PathBuf,
        new_path: PathBuf,
    ) -> Result<(), DatabaseError>;

    fn delete_collection(&self, txn: &mut Transaction, path: PathBuf) -> Result<(), DatabaseError>;

    fn list_collection(&self) -> Result<Vec<(PathBuf, CollectionEntity)>>;
}

type EnvironmentName = String;
pub(crate) type EnvironmentStoreTable<'a> = BincodeTable<'a, EnvironmentName, EnvironmentEntity>;

pub trait EnvironmentStore: Send + Sync {
    fn scan(&self) -> Result<HashMap<EnvironmentName, EnvironmentEntity>>;
}

pub(crate) type StateStoreTable<'a> = BincodeTable<'a, String, AnyEntity>;
pub trait StateStore: Send + Sync + 'static {
    fn get_sidebar_part_state(&self) -> Result<SidebarPartStateEntity, DatabaseError>;
    fn get_panel_part_state(&self) -> Result<PanelPartStateEntity, DatabaseError>;
    fn get_editor_part_state(&self) -> Result<EditorPartStateEntity, DatabaseError>;

    fn put_sidebar_part_state(&self, state: SidebarPartStateEntity) -> Result<(), DatabaseError>;
    fn put_panel_part_state(&self, state: PanelPartStateEntity) -> Result<(), DatabaseError>;
    fn put_editor_part_state(&self, state: EditorPartStateEntity) -> Result<(), DatabaseError>;

    fn delete_sidebar_part_state(&self) -> Result<(), DatabaseError>;
    fn delete_panel_part_state(&self) -> Result<(), DatabaseError>;
    fn delete_editor_part_state(&self) -> Result<(), DatabaseError>;
}

pub struct WorkspaceStorageImpl {
    db_client: ReDbClient,
    collection_store: Arc<dyn CollectionStore>,
    environment_store: Arc<dyn EnvironmentStore>,
    state_store: Arc<dyn StateStore>,
}

impl WorkspaceStorageImpl {
    pub fn new(path: &PathBuf) -> Result<Self> {
        let db_client = ReDbClient::new(path.join(WORKSPACE_STATE_DB_NAME))?
            .with_table(&collection_store::TABLE_COLLECTIONS)?
            .with_table(&environment_store::TABLE_ENVIRONMENTS)?
            .with_table(&state_store::PARTS_STATE)?;

        let collection_store = Arc::new(CollectionStoreImpl::new(db_client.clone()));
        let environment_store = Arc::new(EnvironmentStoreImpl::new(db_client.clone()));
        let state_store = Arc::new(StateStoreImpl::new(db_client.clone()));

        Ok(Self {
            db_client,
            collection_store,
            environment_store,
            state_store,
        })
    }
}

#[async_trait]
impl Transactional for WorkspaceStorageImpl {
    async fn begin_write(&self) -> Result<Transaction, DatabaseError> {
        self.db_client.begin_write()
    }

    async fn begin_read(&self) -> Result<Transaction, DatabaseError> {
        self.db_client.begin_read()
    }
}

impl WorkspaceStorage for WorkspaceStorageImpl {
    fn collection_store(&self) -> Arc<dyn CollectionStore> {
        self.collection_store.clone()
    }

    fn environment_store(&self) -> Arc<dyn EnvironmentStore> {
        self.environment_store.clone()
    }

    fn state_store(&self) -> Arc<dyn StateStore> {
        self.state_store.clone()
    }
}
