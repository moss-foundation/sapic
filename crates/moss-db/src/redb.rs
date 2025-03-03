use std::path::Path;

use anyhow::Result;
use redb::{Database, ReadTransaction, ReadableTable, TableDefinition, WriteTransaction};

// const TABLE_VAULT_DEFINITION: TableDefinition<&str, u64> = TableDefinition::new("vault");

pub struct ReDbClient {
    db: Database,
}

impl ReDbClient {
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        Ok(Self {
            db: Database::create(path)?,
        })
    }

    pub fn write<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(WriteTransaction) -> Result<T>,
    {
        let write_txn = self.db.begin_write()?;
        f(write_txn)
    }

    // pub fn write<F, T>(&self, f: F) -> Result<T>
    // where
    //     F: FnOnce(WriteTransaction) -> Result<T>,
    // {
    //     let write_txn = self.db.begin_write()?;
    //     f(write_txn)
    // }

    pub fn read<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(ReadTransaction) -> Result<T>,
    {
        let read_txn = self.db.begin_read()?;
        f(read_txn)
    }
}

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
