use redb::{Database, ReadableTable, TableDefinition};

const TABLE_VAULT: TableDefinition<&str, u64> = TableDefinition::new("vault");

// #[cfg(test)]
// mod tests {
//     use super::*;

//     struct MyStruct {
//         val: u128,
//     }

//     #[test]
//     fn test_write() {
//         let db = Database::create("sapic.db").unwrap();

//         let write_txn = db.begin_write().unwrap();
//         {
//             let v = MyStruct { val: 128 };
//             let mut table = write_txn.open_table(TABLE_VAULT).unwrap();
//             table.insert("my_key", v).unwrap();
//         }
//         write_txn.commit().unwrap();
//     }

//     #[test]
//     fn test_read() {
//         let db = Database::create("sapic.db").unwrap();
//         let read_txn = db.begin_read().unwrap();
//         let table = read_txn.open_table(TABLE_VAULT).unwrap();

//         assert_eq!(table.get("my_key").unwrap().unwrap().value(), 123);
//     }
// }
