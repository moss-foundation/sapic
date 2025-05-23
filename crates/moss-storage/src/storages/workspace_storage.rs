use moss_db::bincode_table::BincodeTable;
use moss_db::primitives::AnyValue;
use moss_db::{DatabaseClient, DatabaseResult, ReDbClient, Transaction};
use serde_json::Value as JsonValue;
use std::any::TypeId;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use crate::WorkspaceStorage;
use crate::primitives::segkey::SegKeyBuf;
use crate::storage::{SegBinTable, Storage, StoreTypeId, Transactional};
use crate::workspace_storage::stores::item_store::WorkspaceItemStoreImpl;
use crate::workspace_storage::stores::variable_store::WorkspaceVariableStoreImpl;
use crate::workspace_storage::stores::{WorkspaceItemStore, WorkspaceVariableStore};

pub mod entities;
pub mod stores;

const DB_NAME: &str = "state.db";
pub const TABLE_VARIABLES: BincodeTable<SegKeyBuf, AnyValue> = BincodeTable::new("variables");
pub const TABLE_ITEMS: BincodeTable<SegKeyBuf, AnyValue> = BincodeTable::new("items");

pub struct WorkspaceStorageImpl {
    client: ReDbClient,
    tables: HashMap<StoreTypeId, Arc<SegBinTable>>,
}

impl WorkspaceStorageImpl {
    pub fn new(path: impl AsRef<Path>) -> DatabaseResult<Self> {
        let mut client = ReDbClient::new(path.as_ref().join(DB_NAME))?;

        let mut tables = HashMap::new();
        for (type_id, table) in [
            (TypeId::of::<WorkspaceVariableStoreImpl>(), TABLE_VARIABLES),
            (TypeId::of::<WorkspaceItemStoreImpl>(), TABLE_ITEMS),
        ] {
            client = client.with_table(&table)?;
            tables.insert(type_id, Arc::new(table));
        }

        Ok(Self { client, tables })
    }
}

impl Storage for WorkspaceStorageImpl {
    fn dump(&self) -> DatabaseResult<HashMap<String, JsonValue>> {
        let read_txn = self.client.begin_read()?;
        let mut result = HashMap::new();
        for table in self.tables.values() {
            for (k, v) in table.scan(&read_txn)? {
                result.insert(k.to_string(), serde_json::from_slice(v.as_bytes())?);
            }
        }

        Ok(result)
    }
}

impl Transactional for WorkspaceStorageImpl {
    fn begin_write(&self) -> DatabaseResult<Transaction> {
        self.client.begin_write()
    }

    fn begin_read(&self) -> DatabaseResult<Transaction> {
        self.client.begin_read()
    }
}

impl WorkspaceStorage for WorkspaceStorageImpl {
    fn variable_store(&self) -> Arc<dyn WorkspaceVariableStore> {
        let client = self.client.clone();
        let table = self
            .tables
            .get(&TypeId::of::<WorkspaceVariableStoreImpl>())
            .unwrap()
            .clone();
        Arc::new(WorkspaceVariableStoreImpl::new(client, table))
    }

    fn item_store(&self) -> Arc<dyn WorkspaceItemStore> {
        let client = self.client.clone();
        let table = self
            .tables
            .get(&TypeId::of::<WorkspaceItemStoreImpl>())
            .unwrap()
            .clone();
        Arc::new(WorkspaceItemStoreImpl::new(client, table))
    }
}
