use anyhow::Result;
use redb::{Database, Table, TableDefinition, WriteTransaction};

const TABLE_VAULT_DEFINITION: TableDefinition<&str, u64> = TableDefinition::new("vault");

// pub struct ReDbVaultStore<'a: 'static> {
//     db: Database,
//     table: TableDefinition<'a, &'a str, u64>,
// }

// impl ReDbVaultStore<'_> {
//     pub fn write<'txn, F, T>(&self, f: F) -> Result<T>
//     where
//         F: FnOnce(WriteTransaction, Table<'txn, &str, u64>) -> Result<T>,
//     {
//         let write_txn = self.db.begin_write()?;
//         {
//             let mut table = write_txn.open_table(self.table)?;
//             f(write_txn, table)
//         }
//     }
// }
