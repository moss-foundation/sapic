use moss_db::bincode_table::BincodeTable;
use moss_db::primitives::AnyValue;
use moss_db::{DatabaseClient, DatabaseResult, ReDbClient, Table, Transaction};
use serde_json::Value as JsonValue;
use std::any::TypeId;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use crate::CollectionStorage;
use crate::collection_storage::stores::unit_store::CollectionUnitStoreImpl;
use crate::collection_storage::stores::variable_store::CollectionVariableStoreImpl;
use crate::collection_storage::stores::{CollectionUnitStore, CollectionVariableStore};
use crate::primitives::segkey::SegKeyBuf;
use crate::storage::{SegBinTable, Storage, StoreTypeId, Transactional};

pub mod stores;

const DB_NAME: &str = "state.db";
pub const TABLE_VARIABLES: BincodeTable<SegKeyBuf, AnyValue> = BincodeTable::new("items");
pub const TABLE_UNITS: BincodeTable<SegKeyBuf, AnyValue> = BincodeTable::new("units");

pub struct CollectionStorageImpl {
    client: ReDbClient,
    tables: HashMap<StoreTypeId, Arc<SegBinTable>>,
}

impl CollectionStorageImpl {
    pub fn new(path: impl AsRef<Path>) -> DatabaseResult<Self> {
        let mut client = ReDbClient::new(path.as_ref().join(DB_NAME))?;

        let mut tables = HashMap::new();
        for (type_id, table) in [
            (TypeId::of::<CollectionVariableStoreImpl>(), TABLE_VARIABLES),
            (TypeId::of::<CollectionUnitStoreImpl>(), TABLE_UNITS),
        ] {
            client = client.with_table(&table)?;
            tables.insert(type_id, Arc::new(table));
        }

        Ok(Self { client, tables })
    }
}

impl Storage for CollectionStorageImpl {
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

impl Transactional for CollectionStorageImpl {
    fn begin_write(&self) -> DatabaseResult<Transaction> {
        self.client.begin_write()
    }

    fn begin_read(&self) -> DatabaseResult<Transaction> {
        self.client.begin_read()
    }
}

impl CollectionStorage for CollectionStorageImpl {
    fn variable_store(&self) -> Arc<dyn CollectionVariableStore> {
        let client = self.client.clone();
        let table = self
            .tables
            .get(&TypeId::of::<CollectionVariableStoreImpl>())
            .unwrap()
            .clone();
        Arc::new(CollectionVariableStoreImpl::new(client, table))
    }

    fn unit_store(&self) -> Arc<dyn CollectionUnitStore> {
        let client = self.client.clone();
        let table = self
            .tables
            .get(&TypeId::of::<CollectionUnitStoreImpl>())
            .unwrap()
            .clone();
        Arc::new(CollectionUnitStoreImpl::new(client, table))
    }
}
