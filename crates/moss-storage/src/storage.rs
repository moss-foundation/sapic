pub mod operations;
pub mod table;

use async_trait::async_trait;
use moss_db::primitives::AnyValue;
use moss_db::{DatabaseClient, DatabaseResult, ReDbClient, Transaction};
use serde_json::Value as JsonValue;
use std::{any::TypeId, collections::HashMap, future::Future, path::Path, pin::Pin, sync::Arc};
use table::Store;

use crate::primitives::segkey::SegKeyBuf;

#[async_trait]
pub trait Transactional {
    async fn begin_write(&self) -> DatabaseResult<Transaction>;
    async fn begin_read(&self) -> DatabaseResult<Transaction>;
}

#[async_trait]
pub trait Storage: Transactional + Send + Sync {
    async fn table(&self, id: &TypeId) -> DatabaseResult<Arc<dyn Store>>;
    async fn dump(&self) -> DatabaseResult<HashMap<String, JsonValue>>;
}

// TODO: remove this
#[async_trait]
pub trait ResettableStorage {
    async fn reset(
        &self,
        path: &Path,
        after_drop: Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>>, // TODO: change to DatabaseResult
    ) -> anyhow::Result<()>; // TODO: change to DatabaseResult
}

pub struct StorageHandle {
    client: ReDbClient,
    tables: HashMap<TypeId, Arc<dyn Store>>,
}

impl StorageHandle {
    pub fn new(path: impl AsRef<Path>, tables: Vec<impl Store + 'static>) -> DatabaseResult<Self> {
        let mut client = ReDbClient::new(path.as_ref())?;
        let mut known_tables = HashMap::new();
        for table in tables {
            client = client.with_table(table.table())?;
            known_tables.insert(table.type_id(), Arc::from(table) as Arc<dyn Store>);
        }

        Ok(Self {
            client,
            tables: known_tables,
        })
    }
}

#[async_trait]
impl Transactional for StorageHandle {
    async fn begin_write(&self) -> DatabaseResult<Transaction> {
        self.client.begin_write()
    }

    async fn begin_read(&self) -> DatabaseResult<Transaction> {
        self.client.begin_read()
    }
}

#[async_trait]
impl Storage for StorageHandle {
    async fn table(&self, id: &TypeId) -> DatabaseResult<Arc<dyn Store>> {
        Ok(self.tables.get(id).unwrap().clone())
    }
    async fn dump(&self) -> DatabaseResult<HashMap<String, JsonValue>> {
        // FIXME: Error propagation seems to make pipelining very tricky

        let read_txn = self.begin_read().await?;
        let mut result = HashMap::new();
        for table in self.tables.values() {
            for (k, v) in table.table().scan(&read_txn)? {
                result.insert(k.to_string(), serde_json::from_slice(v.as_bytes())?);
            }
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use moss_db::bincode_table::BincodeTable;

    use super::*;

    pub(super) const TABLE_VARIABLES: BincodeTable<SegKeyBuf, AnyValue> =
        BincodeTable::new("variables");

    pub struct VariableStore {
        client: ReDbClient,
        table: Arc<BincodeTable<'static, SegKeyBuf, AnyValue>>,
    }

    impl VariableStore {
        pub fn new(
            client: ReDbClient,
            table: Arc<BincodeTable<'static, SegKeyBuf, AnyValue>>,
        ) -> Self {
            Self { client, table }
        }
    }

    type StoreTypeId = TypeId;
    type Table = Arc<BincodeTable<'static, SegKeyBuf, AnyValue>>;

    pub struct WorkspaceStorage {
        client: ReDbClient,
        tables: HashMap<StoreTypeId, Table>,
    }

    impl WorkspaceStorage {
        pub fn new(path: impl AsRef<Path>) -> DatabaseResult<Self> {
            let mut client = ReDbClient::new(path.as_ref())?;

            let mut tables = HashMap::new();
            for (type_id, table) in [(TypeId::of::<VariableStore>(), TABLE_VARIABLES)] {
                client = client.with_table(&table)?;
                tables.insert(type_id, Arc::new(table));
            }

            Ok(Self { client, tables })
        }

        pub fn variable_store(&self) -> VariableStore {
            VariableStore {
                client: self.client.clone(),
                table: self
                    .tables
                    .get(&TypeId::of::<VariableStore>())
                    .unwrap()
                    .clone(),
            }
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::primitives::segkey::SegKey;
//     use moss_db::bincode_table::BincodeTable;
//     use moss_db::primitives::AnyValue;
//     use moss_testutils::random_name::random_string;
//     use std::ops::Deref;
//
//     struct Table1 {
//         table: BincodeTable<'static, SegKeyBuf, AnyValue>,
//     }
//     impl Table1 {
//         pub const fn new() -> Self {
//             Self {
//                 table: BincodeTable::new("table1"),
//             }
//         }
//     }
//     impl Table for Table1 {
//         fn definition(&self) -> &BincodeTable<SegKeyBuf, AnyValue> {
//             &self.table
//         }
//     }
//
//     const TABLE1: Table1 = Table1::new();
//
//     struct TestStore {
//         handle: Arc<dyn Storage>,
//     }
//
//     impl TestStore {
//         pub fn new(path: &Path) -> Self {
//             Self {
//                 handle: Arc::new(StorageHandle::new(path, vec![Table1::new()]).unwrap()),
//             }
//         }
//         pub async fn table1(&self) -> DatabaseResult<Arc<dyn Table>> {
//             self.handle.table(&TypeId::of::<Table1>()).await
//         }
//     }
//
//     impl Deref for TestStore {
//         type Target = Arc<dyn Storage>;
//
//         fn deref(&self) -> &Self::Target {
//             &self.handle
//         }
//     }
//
//     #[tokio::test]
//     async fn test_storage() {
//         let store = TestStore::new(&Path::new("tests").join(format!("{}.db", random_string(10))));
//         let table1 = store.table1().await.unwrap();
//
//         let mut write_txn = store.begin_write().await.unwrap();
//
//         table1
//             .definition()
//             .insert(
//                 &mut write_txn,
//                 SegKey::new("table1").join("key1"),
//                 &AnyValue::new(serde_json::to_vec("value1").unwrap()),
//             )
//             .unwrap();
//
//         write_txn.commit().unwrap();
//
//         dbg!(store.dump().await.unwrap());
//     }
// }
