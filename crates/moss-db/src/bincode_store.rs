use anyhow::Result;
use redb::Key;
use serde::{de::DeserializeOwned, Serialize};
use std::borrow::Borrow;

use crate::{bincode_table::BincodeTable, DatabaseClient, ReDbClient, Transaction};

pub struct BincodeStore<'a, K, V>
where
    K: Key + 'static + Borrow<K::SelfType<'a>>,
    V: Serialize + DeserializeOwned,
{
    client: ReDbClient,
    table: BincodeTable<'a, K, V>,
}

impl<'a, K, V> BincodeStore<'a, K, V>
where
    K: Key + 'static + Borrow<K::SelfType<'a>>,
    V: Serialize + DeserializeOwned,
{
    pub fn new(client: ReDbClient, table: BincodeTable<'a, K, V>) -> Self {
        Self { client, table }
    }

    pub fn begin_write<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(Transaction, &BincodeTable<'a, K, V>) -> Result<T>,
    {
        let write_txn = self.client.begin_write()?;
        f(Transaction::Write(write_txn), &self.table)
    }

    pub fn begin_read<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(Transaction, &BincodeTable<'a, K, V>) -> Result<T>,
    {
        let read_txn = self.client.begin_read()?;
        f(Transaction::Read(read_txn), &self.table)
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use super::*;

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct MyStruct {
        val: u64,
    }

    const TABLE_VAULT: BincodeTable<&str, MyStruct> = BincodeTable::new("vault");

    #[test]
    fn test_write() {
        let client = ReDbClient::new("sapic.db").unwrap();
        let vault_store = BincodeStore::new(client, TABLE_VAULT);

        vault_store
            .begin_write(|mut txn, table| -> Result<()> {
                table.insert(&mut txn, "my_key", &MyStruct { val: 42 })?;

                Ok(txn.commit()?)
            })
            .unwrap();

        // let client = Arc::new(ReDbClient::new(Database::create("sapic.db").unwrap()));
        // let vault_store = VaultStore::new(client);

        // vault_store
        //     .write(|txn, wrapper| {
        //         let mut table = txn.open_table(wrapper.table_definition())?;
        //         wrapper.insert(&mut table, "my_key", &42u64)?;
        //         Ok(())
        //     })
        //     .unwrap();
    }

    #[test]
    fn test_read() {
        let client = ReDbClient::new("sapic.db").unwrap();
        let vault_store = BincodeStore::new(client, TABLE_VAULT);

        let r = vault_store
            .begin_write(|txn, table| {
                // let t = txn.open_table(table.table)?;

                // let r = t.get("my_key")?.unwrap();
                // let value = r.value();
                // let r = bincode::deserialize(&value)?;
                let r = table.read(&txn, "my_key")?;

                Ok(r)
            })
            .unwrap();

        println!("{:?}", r);

        // let read_txn = db.begin_read().unwrap();
        // let table = read_txn.open_table(TABLE_VAULT).unwrap();

        // assert_eq!(table.get("my_key").unwrap().unwrap().value(), 123);
    }
}
