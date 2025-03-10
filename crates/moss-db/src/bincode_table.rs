use anyhow::{anyhow, Context as _, Result};
use redb::{Key, TableDefinition};
use serde::{de::DeserializeOwned, Serialize};
use std::borrow::Borrow;

use crate::Transaction;

#[derive(Clone)]
pub struct BincodeTable<'a, K, V>
where
    K: Key + 'static + Borrow<K::SelfType<'a>>,
    V: Serialize + DeserializeOwned,
{
    table: TableDefinition<'a, K, Vec<u8>>,
    _marker: std::marker::PhantomData<V>,
}

impl<'a, K, V> BincodeTable<'a, K, V>
where
    K: Key + 'static + Borrow<K::SelfType<'a>>,
    V: Serialize + DeserializeOwned,
{
    pub const fn new(table_name: &'static str) -> Self {
        Self {
            table: TableDefinition::new(table_name),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn clone(&self) -> Self {
        Self {
            table: self.table.clone(),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn insert(&self, txn: &mut Transaction, key: K, value: &V) -> Result<()> {
        match txn {
            Transaction::Write(txn) => {
                let mut table = txn.open_table(self.table)?;
                let bytes = bincode::serialize(value)?;
                table.insert(key.borrow(), bytes)?;
                Ok(())
            }
            Transaction::Read(_) => Err(anyhow!("Cannot insert into read transaction")),
        }
    }

    pub fn read(&self, txn: &Transaction, key: K) -> Result<V> {
        match txn {
            Transaction::Read(txn) => {
                let table = txn.open_table(self.table).context("Failed to open table")?;
                let entry = table
                    .get(key)
                    .context("Failed to retrieve value from table")?
                    .ok_or_else(|| anyhow!("No value found for the specified key"))?;

                let value = entry.value().to_vec();
                let result =
                    bincode::deserialize(&value).context("Failed to deserialize the data")?;

                Ok(result)
            }
            Transaction::Write(_) => Err(anyhow!("Cannot read from write transaction")),
        }
    }
}
