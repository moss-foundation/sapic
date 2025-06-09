use moss_db::{
    DatabaseClient, DatabaseResult, ReDbClient, Table, Transaction, bincode_table::BincodeTable,
    primitives::AnyValue,
};
use redb::TableHandle;
use serde_json::{Value as JsonValue, json};
use std::{any::TypeId, collections::HashMap, path::Path, sync::Arc};

use crate::{
    CollectionStorage,
    collection_storage::stores::{
        CollectionUnitStore, CollectionVariableStore, MixedStore, mixed_store::MixedStoreImpl,
        unit_store::CollectionUnitStoreImpl, variable_store::CollectionVariableStoreImpl,
    },
    primitives::segkey::SegKeyBuf,
    storage::{SegBinTable, Storage, StoreTypeId, Transactional},
};

pub mod entities;
pub mod stores;

const DB_NAME: &str = "state.db";

pub const TABLE_VARIABLES: BincodeTable<SegKeyBuf, AnyValue> = BincodeTable::new("variables");
pub const TABLE_UNITS: BincodeTable<SegKeyBuf, AnyValue> = BincodeTable::new("units");
pub const TABLE_MIXED: BincodeTable<SegKeyBuf, AnyValue> = BincodeTable::new("mixed");

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
            (TypeId::of::<MixedStoreImpl>(), TABLE_MIXED),
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

    fn mixed_store(&self) -> Arc<dyn MixedStore> {
        let client = self.client.clone();
        let table = self
            .tables
            .get(&TypeId::of::<MixedStoreImpl>())
            .unwrap()
            .clone();

        Arc::new(MixedStoreImpl::new(client, table))
    }
}
