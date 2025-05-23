use moss_db::bincode_table::BincodeTable;
use moss_db::primitives::AnyValue;
use moss_db::{DatabaseClient, DatabaseResult, ReDbClient, Table, Transaction};
use redb::TableHandle;
use serde_json::{Value as JsonValue, json};
use std::any::TypeId;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use crate::GlobalStorage;
use crate::global_storage::stores::GlobalItemStore;
use crate::global_storage::stores::item_store::GlobalItemStoreImpl;
use crate::primitives::segkey::SegKeyBuf;
use crate::storage::{SegBinTable, Storage, StoreTypeId, Transactional};

pub mod entities;
pub mod stores;

pub const TABLE_ITEMS: BincodeTable<SegKeyBuf, AnyValue> = BincodeTable::new("items");

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
                table_entries.insert(
                    k.to_string(),
                    serde_json::from_slice::<JsonValue>(v.as_bytes())?,
                );
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
