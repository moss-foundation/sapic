pub mod collection_store;
pub mod entities;
pub mod state_store;
pub mod variable_store;

use anyhow::Result;
use async_trait::async_trait;
use collection_store::CollectionStoreImpl;
use entities::{
    collection_store_entities::CollectionEntity,
    state_store_entities::{EditorPartStateEntity, PanelPartStateEntity, SidebarPartStateEntity},
    variable_store_entities::VariableEntity,
};
use moss_db::{
    DatabaseClient, ReDbClient,
    bincode_table::BincodeTable,
    common::{AnyEntity, DatabaseError, Transaction},
};
use state_store::StateStoreImpl;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};
use variable_store::VariableStoreImpl;

use crate::{
    WorkspaceStorage,
    common::{NamespacedStore, Transactional},
};

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

    // TODO: rename to list_collections
    fn list_collection(&self) -> Result<Vec<(PathBuf, CollectionEntity)>>;
}

pub(crate) type VariableStoreTable<'a> = BincodeTable<'a, String, VariableEntity>;

// <environment_name>:<variable_name>
pub type VariableKey = String;
pub trait VariableStore: Send + Sync {
    fn list_variables(&self) -> Result<HashMap<VariableKey, VariableEntity>, DatabaseError>;
}

pub(crate) type StateStoreTable<'a> = BincodeTable<'a, String, AnyEntity>;
pub trait StateStore: NamespacedStore<dyn StateStoreNamespaceExt> + Send + Sync {
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

pub trait PartNamespace: Send + Sync {}
pub trait CollectionNamespace: Send + Sync {}
pub trait EnvironmentNamespace: Send + Sync {}

pub trait StateStoreNamespaceExt: Send + Sync {
    fn part(self: Arc<Self>) -> Arc<dyn PartNamespace>;
    // fn collection(&self) -> Arc<dyn CollectionNamespace>;
    // fn environment(&self) -> Arc<dyn EnvironmentNamespace>;
}

pub struct WorkspaceStorageImpl {
    db_client: ReDbClient,
    collection_store: Arc<dyn CollectionStore>,
    environment_store: Arc<dyn VariableStore>,
    state_store: Arc<dyn StateStore>,
    variable_store: Arc<dyn VariableStore>,
}

impl WorkspaceStorageImpl {
    pub fn new(path: &Path) -> Result<Self> {
        let db_client = ReDbClient::new(path.join(WORKSPACE_STATE_DB_NAME))?
            .with_table(&collection_store::TABLE_COLLECTIONS)?
            .with_table(&variable_store::TABLE_VARIABLES)?
            .with_table(&state_store::PARTS_STATE)?;

        let collection_store = Arc::new(CollectionStoreImpl::new(db_client.clone()));
        let environment_store = Arc::new(VariableStoreImpl::new(db_client.clone()));
        let state_store = Arc::new(StateStoreImpl::new(db_client.clone()));
        let variable_store = Arc::new(VariableStoreImpl::new(db_client.clone()));

        Ok(Self {
            db_client,
            collection_store,
            environment_store,
            state_store,
            variable_store,
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

    fn environment_store(&self) -> Arc<dyn VariableStore> {
        self.environment_store.clone()
    }

    fn state_store(&self) -> Arc<dyn StateStore> {
        self.state_store.clone()
    }

    fn variable_store(&self) -> Arc<dyn VariableStore> {
        self.variable_store.clone()
    }
}
