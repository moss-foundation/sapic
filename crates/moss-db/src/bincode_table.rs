use redb::{Key, ReadableTable, TableDefinition};
use serde::{Serialize, de::DeserializeOwned};
use std::{
    borrow::Borrow,
    fmt::{Debug, Display},
    hash::Hash,
};

use crate::{
    Table,
    common::{DatabaseError, Transaction},
};

#[derive(Clone)]
pub struct BincodeTable<'a, K, V>
where
    K: Key + 'static + Borrow<K::SelfType<'a>> + Clone + Eq + Hash + Debug + Display,
    for<'b> K::SelfType<'b>: ToOwned<Owned = K>,
    V: Serialize + DeserializeOwned,
{
    table: TableDefinition<'a, K, Vec<u8>>,
    _marker: std::marker::PhantomData<V>,
}

impl<'a, K, V> From<&BincodeTable<'a, K, V>> for TableDefinition<'a, K, Vec<u8>>
where
    K: Key + 'static + Borrow<K::SelfType<'a>> + Clone + Eq + Hash + Debug + Display,
    for<'b> K::SelfType<'b>: ToOwned<Owned = K>,
    V: Serialize + DeserializeOwned,
{
    fn from(value: &BincodeTable<'a, K, V>) -> Self {
        value.table
    }
}

impl<'a, K, V> Table<'a, K, V> for BincodeTable<'a, K, V>
where
    K: Key + 'static + Borrow<K::SelfType<'a>> + Clone + Eq + Hash + Debug + Display,
    for<'b> K::SelfType<'b>: ToOwned<Owned = K>,
    V: Serialize + DeserializeOwned,
{
    fn table_definition(&self) -> TableDefinition<'a, K, Vec<u8>> {
        self.table.clone()
    }
}

impl<'a, K, V> BincodeTable<'a, K, V>
where
    K: Key + 'static + Borrow<K::SelfType<'a>> + Clone + Eq + Hash + Debug + Display,
    for<'b> K::SelfType<'b>: ToOwned<Owned = K>,
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

    pub fn insert(&self, txn: &mut Transaction, key: K, value: &V) -> Result<(), DatabaseError> {
        match txn {
            Transaction::Write(txn) => {
                let mut table = txn.open_table(self.table)?;

                let bytes = serde_json::to_vec(value)?;

                table.insert(key.borrow(), bytes)?;
                Ok(())
            }
            Transaction::Read(_txn) => Err(DatabaseError::Transaction(
                "Cannot insert into read transaction".to_string(),
            )),
        }
    }

    pub fn remove(&self, txn: &mut Transaction, key: K) -> Result<V, DatabaseError> {
        match txn {
            Transaction::Write(txn) => {
                let mut table = txn.open_table(self.table)?;

                let bytes = table
                    .remove(key.borrow())?
                    .ok_or_else(|| DatabaseError::NotFound {
                        key: key.to_string(),
                    })?
                    .value();
                let value: V = serde_json::from_slice(&bytes)?;
                Ok(value)
            }
            Transaction::Read(_txn) => Err(DatabaseError::Transaction(
                "Cannot remove from read transaction".to_string(),
            )),
        }
    }

    pub fn read(&self, txn: &Transaction, key: K) -> Result<V, DatabaseError> {
        match txn {
            Transaction::Read(txn) => {
                let table = txn.open_table(self.table)?;

                let bytes = table
                    .get(key.borrow())?
                    .ok_or_else(|| DatabaseError::NotFound {
                        key: key.to_string(),
                    })?
                    .value();
                let value: V = serde_json::from_slice(&bytes)?;

                Ok(value)
            }
            Transaction::Write(_txn) => Err(DatabaseError::Transaction(
                "Cannot read from write transaction".to_string(),
            )),
        }
    }

    pub fn scan(&self, txn: &Transaction) -> Result<impl Iterator<Item = (K, V)>, DatabaseError> {
        match txn {
            Transaction::Read(txn) => {
                let table = txn.open_table(self.table)?;
                let mut result = Vec::new();

                for entry in table.iter()? {
                    let (key_guard, value_guard) = entry?;

                    let bytes = value_guard.value();
                    let value: V = serde_json::from_slice(&bytes)?;
                    result.push((key_guard.value().to_owned(), value));
                }

                Ok(result.into_iter())
            }
            Transaction::Write(_txn) => Err(DatabaseError::Transaction(
                "Cannot read from write transaction".to_string(),
            )),
        }
    }

    pub fn rekey(
        &self,
        txn: &mut Transaction,
        old_key: K,
        new_key: K,
    ) -> Result<(), DatabaseError> {
        match txn {
            Transaction::Write(txn) => {
                let mut table = txn.open_table(self.table)?;
                let bytes = table
                    .remove(old_key.borrow())?
                    .ok_or_else(|| DatabaseError::NotFound {
                        key: old_key.to_string(),
                    })?
                    .value();

                table.insert(new_key.borrow(), bytes)?;

                Ok(())
            }
            Transaction::Read(_txn) => Err(DatabaseError::Transaction(
                "Cannot rekey in read transaction".to_string(),
            )),
        }
    }

    pub fn truncate(&self, txn: &mut Transaction) -> Result<(), DatabaseError> {
        match txn {
            Transaction::Write(txn) => {
                let mut table = txn.open_table(self.table)?;
                table.retain(|_, _| false)?;

                Ok(())
            }
            Transaction::Read(_txn) => Err(DatabaseError::Transaction(
                "Cannot truncate table in read transaction".to_string(),
            )),
        }
    }

    pub fn scan_by_prefix<P>(
        &self,
        txn: &Transaction,
        prefix: P,
    ) -> Result<Vec<(K, V)>, DatabaseError>
    where
        P: AsRef<str>,
    {
        match txn {
            Transaction::Read(txn) => {
                let table = txn.open_table(self.table)?;
                let mut result = Vec::new();
                let prefix_str = prefix.as_ref();

                for entry in table.iter()? {
                    let (key_guard, value_guard) = entry?;
                    let key = key_guard.value().to_owned();
                    let key_str = key.to_string();

                    if key_str.starts_with(prefix_str) {
                        let bytes = value_guard.value();
                        let value: V = serde_json::from_slice(&bytes)?;
                        result.push((key, value));
                    }
                }

                Ok(result)
            }
            Transaction::Write(_txn) => Err(DatabaseError::Transaction(
                "Cannot scan from write transaction".to_string(),
            )),
        }
    }

    pub fn remove_by_prefix<P>(
        &self,
        txn: &mut Transaction,
        prefix: P,
    ) -> Result<Vec<(K, V)>, DatabaseError>
    where
        P: AsRef<str>,
    {
        match txn {
            Transaction::Write(txn) => {
                let mut table = txn.open_table(self.table)?;
                let prefix_str = prefix.as_ref();
                let mut result = Vec::new();

                for entry in table.iter()? {
                    let (key_guard, value_guard) = entry?;
                    let key = key_guard.value().to_owned();
                    let key_str = key.to_string();

                    if key_str.starts_with(prefix_str) {
                        let bytes = value_guard.value();
                        let value: V = serde_json::from_slice(&bytes)?;
                        result.push((key.clone(), value));
                    }
                }

                for (key, _) in result.iter() {
                    table.remove(key.borrow())?;
                }

                Ok(result)
            }
            Transaction::Read(_txn) => Err(DatabaseError::Transaction(
                "Cannot remove by prefix from read transaction".to_string(),
            )),
        }
    }
}
