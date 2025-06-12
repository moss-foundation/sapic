use moss_db::{
    DatabaseClient, DatabaseResult, ReDbClient, Table, Transaction, anyvalue_enum::AnyValueEnum,
    bincode_table::BincodeTable,
};
use redb::TableHandle;
use serde_json::{Value as JsonValue, json};
use std::{any::TypeId, collections::HashMap, path::Path, sync::Arc};

use crate::{
    GlobalStorage,
    global_storage::stores::{GlobalItemStore, item_store::GlobalItemStoreImpl},
    primitives::segkey::SegKeyBuf,
    storage::{SegBinTable, Storage, StoreTypeId, Transactional},
};

pub mod entities;
pub mod stores;

pub const TABLE_ITEMS: BincodeTable<SegKeyBuf, AnyValueEnum> = BincodeTable::new("items");
pub struct GlobalStorageImpl {
    client: ReDbClient,
    tables: HashMap<StoreTypeId, Arc<SegBinTable>>,
}

const DB_NAME: &str = "state.db";

impl GlobalStorageImpl {
    pub fn new(path: impl AsRef<Path>) -> DatabaseResult<Self> {
        let mut client = ReDbClient::new(path.as_ref().join(DB_NAME))?;

        let mut tables = HashMap::new();
        for (type_id, table) in [(TypeId::of::<GlobalItemStoreImpl>(), TABLE_ITEMS)] {
            client = client.with_table(&table)?;
            tables.insert(type_id, Arc::new(table));
        }

        Ok(Self { client, tables })
    }
}

impl Storage for GlobalStorageImpl {
    fn dump(&self) -> DatabaseResult<HashMap<String, JsonValue>> {
        let read_txn = self.client.begin_read()?;
        let mut result = HashMap::new();
        for table in self.tables.values() {
            let name = table.table_definition().name().to_string();
            let mut table_entries = HashMap::new();
            for (k, v) in table.scan(&read_txn)? {
                table_entries.insert(k.to_string(), serde_json::to_value(&v)?);
            }
            result.insert(format!("table:{}", name), json!(table_entries));
        }
        Ok(result)
    }
}

impl Transactional for GlobalStorageImpl {
    fn begin_write(&self) -> DatabaseResult<Transaction> {
        self.client.begin_write()
    }

    fn begin_read(&self) -> DatabaseResult<Transaction> {
        self.client.begin_read()
    }
}

impl GlobalStorage for GlobalStorageImpl {
    fn item_store(&self) -> Arc<dyn GlobalItemStore> {
        let client = self.client.clone();
        let table = self
            .tables
            .get(&TypeId::of::<GlobalItemStoreImpl>())
            .unwrap()
            .clone();
        Arc::new(GlobalItemStoreImpl::new(client, table))
    }
}
