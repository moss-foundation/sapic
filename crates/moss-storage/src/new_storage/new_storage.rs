// use async_trait::async_trait;
// use moss_db::{DatabaseClient, DatabaseResult, ReDbClient, Transaction};
// use serde_json::Value as JsonValue;
// use std::any::TypeId;
// use std::collections::HashMap;
// use std::path::Path;
// use std::sync::Arc;

// use crate::new_storage::{Dump, NewStorage, NewTransactional};
// use crate::primitives::segkey::SegKeyBuf;
// use crate::storage::table::Table;

// pub struct NewStorageHandle {
//     client: ReDbClient,
//     pub(crate) tables: HashMap<TypeId, Arc<dyn Table>>,
// }

// impl NewStorageHandle {
//     pub fn new(path: impl AsRef<Path>, tables: Vec<Arc<dyn Table>>) -> DatabaseResult<Self> {
//         let mut client = ReDbClient::new(path.as_ref())?;
//         let mut known_tables = HashMap::new();
//         for table in tables {
//             client = client.with_table(table.definition())?;
//             known_tables.insert(table.type_id(), table);
//         }

//         Ok(Self {
//             client,
//             tables: known_tables,
//         })
//     }
// }

// impl NewTransactional for NewStorageHandle {
//     fn begin_write(&self) -> DatabaseResult<Transaction> {
//         self.client.begin_write()
//     }

//     fn begin_read(&self) -> DatabaseResult<Transaction> {
//         self.client.begin_read()
//     }
// }

// impl Dump for NewStorageHandle {
//     fn dump(&self) -> DatabaseResult<HashMap<String, JsonValue>> {
//         let read_txn = self.begin_read()?;
//         let mut result = HashMap::new();
//         for table in self.tables.values() {
//             for (k, v) in table.definition().scan(&read_txn)? {
//                 result.insert(k.to_string(), serde_json::from_slice(v.as_bytes())?);
//             }
//         }

//         Ok(result)
//     }
// }

// #[async_trait]
// impl NewStorage for NewStorageHandle {
//     fn table(&self, id: &TypeId) -> DatabaseResult<Arc<dyn Table>> {
//         Ok(self.tables.get(id).unwrap().clone())
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::primitives::segkey::SegKey;
//     use moss_db::bincode_table::BincodeTable;
//     use moss_db::primitives::AnyValue;
//     use moss_testutils::random_name::random_string;
//     use serde::{Deserialize, Serialize};

//     struct Table1 {
//         table: BincodeTable<'static, SegKeyBuf, AnyValue>,
//         name: &'static str,
//     }
//     impl Table1 {
//         pub const fn new() -> Self {
//             Self {
//                 table: BincodeTable::new("table1"),
//                 name: "table1",
//             }
//         }
//     }
//     impl Table for Table1 {
//         fn definition(&self) -> &BincodeTable<SegKeyBuf, AnyValue> {
//             &self.table
//         }

//         fn name(&self) -> &'static str {
//             &self.name
//         }
//     }

//     struct Table2 {
//         table: BincodeTable<'static, SegKeyBuf, AnyValue>,
//         name: &'static str,
//     }
//     impl Table2 {
//         pub const fn new() -> Self {
//             Self {
//                 table: BincodeTable::new("table1"),
//                 name: "table2",
//             }
//         }
//     }
//     impl Table for Table2 {
//         fn definition(&self) -> &BincodeTable<SegKeyBuf, AnyValue> {
//             &self.table
//         }

//         fn name(&self) -> &'static str {
//             &self.name
//         }
//     }

//     const TABLE1: Table1 = Table1::new();
//     const TABLE2: Table2 = Table2::new();

//     struct TestStore {
//         handle: Arc<dyn NewStorage>,
//     }

//     impl TestStore {
//         pub fn new(path: &Path) -> Self {
//             Self {
//                 handle: Arc::new(
//                     NewStorageHandle::new(path, vec![Arc::new(TABLE1), Arc::new(TABLE2)]).unwrap(),
//                 ),
//             }
//         }
//         pub fn table1(&self) -> DatabaseResult<Arc<dyn Table>> {
//             self.handle.table(&TypeId::of::<Table1>())
//         }
//         pub fn table2(&self) -> DatabaseResult<Arc<dyn Table>> {
//             self.handle.table(&TypeId::of::<Table2>())
//         }
//     }

//     impl NewTransactional for TestStore {
//         fn begin_write(&self) -> DatabaseResult<Transaction> {
//             self.handle.begin_write()
//         }

//         fn begin_read(&self) -> DatabaseResult<Transaction> {
//             self.handle.begin_read()
//         }
//     }

//     impl Dump for TestStore {
//         fn dump(&self) -> DatabaseResult<HashMap<String, JsonValue>> {
//             self.handle.dump()
//         }
//     }

//     #[derive(Serialize, Deserialize, PartialEq)]
//     struct TestData {
//         string: String,
//         number: i32,
//         boolean: bool,
//     }

//     #[tokio::test]
//     async fn test_storage_basic() {
//         let path = Path::new("tests").join(format!("{}.db", random_string(10)));

//         let store = TestStore::new(&path);
//         let table1 = store.table1().unwrap();
//         let table2 = store.table2().unwrap();

//         let mut write_txn = store.begin_write().unwrap();

//         table1
//             .definition()
//             .insert(
//                 &mut write_txn,
//                 SegKey::new("table1").join("string"),
//                 &AnyValue::new(serde_json::to_vec("string").unwrap()),
//             )
//             .unwrap();

//         table1
//             .definition()
//             .insert(
//                 &mut write_txn,
//                 SegKey::new("table1").join("number"),
//                 &AnyValue::new(serde_json::to_vec(&42).unwrap()),
//             )
//             .unwrap();

//         let data = TestData {
//             string: "String".to_string(),
//             number: 42,
//             boolean: true,
//         };

//         table2
//             .definition()
//             .insert(
//                 &mut write_txn,
//                 SegKey::new("table2").join("testdata"),
//                 &AnyValue::new(serde_json::to_vec(&data).unwrap()),
//             )
//             .unwrap();

//         write_txn.commit().unwrap();

//         // Test reading the values from the storage
//         let read_txn = store.begin_read().unwrap();

//         let value1: JsonValue = table1
//             .definition()
//             .read(&read_txn, SegKey::new("table1").join("string"))
//             .unwrap()
//             .deserialize()
//             .unwrap();
//         assert_eq!(value1, JsonValue::String("string".to_string()));

//         let value2: JsonValue = table1
//             .definition()
//             .read(&read_txn, SegKey::new("table1").join("number"))
//             .unwrap()
//             .deserialize()
//             .unwrap();
//         assert_eq!(value2, JsonValue::Number(42.into()));

//         let value3: JsonValue = table2
//             .definition()
//             .read(&read_txn, SegKey::new("table2").join("testdata"))
//             .unwrap()
//             .deserialize()
//             .unwrap();
//         assert_eq!(value3, serde_json::to_value(&data).unwrap());

//         // Test dumping the entries from the storage
//         let dumped = store.dump().unwrap();

//         assert_eq!(dumped.len(), 3);
//         assert_eq!(
//             dumped["table1:string"],
//             JsonValue::String("string".to_string())
//         );
//         assert_eq!(dumped["table1:number"], JsonValue::Number(42.into()));
//         assert_eq!(
//             dumped["table2:testdata"],
//             serde_json::to_value(&data).unwrap()
//         );

//         tokio::fs::remove_file(&path).await.unwrap();
//     }
// }
