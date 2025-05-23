pub mod operations;

use moss_db::bincode_table::BincodeTable;
use moss_db::primitives::AnyValue;
use moss_db::{DatabaseResult, Transaction};
use serde_json::Value as JsonValue;
use std::{any::TypeId, collections::HashMap};

use crate::primitives::segkey::SegKeyBuf;

pub trait Transactional {
    fn begin_write(&self) -> DatabaseResult<Transaction>;
    fn begin_read(&self) -> DatabaseResult<Transaction>;
}

pub trait Storage {
    // TODO: How to organize the output from different tables?
    fn dump(&self) -> DatabaseResult<HashMap<String, JsonValue>>;
}

pub type StoreTypeId = TypeId;
pub type SegBinTable = BincodeTable<'static, SegKeyBuf, AnyValue>;

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
