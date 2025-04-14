use anyhow::{anyhow, Context as _};
use redb::{Key, ReadableTable, TableDefinition};
use serde::{de::DeserializeOwned, Serialize};
use std::borrow::Borrow;
use std::fmt::{Debug, Display};
use std::hash::Hash;

use crate::{common::DatabaseError, Table, Transaction};

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
                let bytes = bincode::serialize(value)?;
                table.insert(key.borrow(), bytes)?;
                Ok(())
            }
            Transaction::Read(_) => Err(DatabaseError::Transaction(
                "Cannot insert into read transaction".to_string(),
            )),
        }
    }

    pub fn remove(&self, txn: &mut Transaction, key: K) -> Result<V, DatabaseError> {
        match txn {
            Transaction::Write(txn) => {
                let mut table = txn.open_table(self.table)?;
                let value = table
                    .remove(key.borrow())?
                    .ok_or_else(|| DatabaseError::NotFound {
                        key: key.to_string(),
                    })?
                    .value();

                let result = bincode::deserialize(&value)?;
                Ok(result)
            }
            Transaction::Read(_) => Err(DatabaseError::Transaction(
                "Cannot remove from read transaction".to_string(),
            )),
        }
    }

    pub fn read(&self, txn: &Transaction, key: K) -> Result<V, DatabaseError> {
        match txn {
            Transaction::Read(txn) => {
                let table = txn.open_table(self.table)?;
                let entry = table
                    .get(key.borrow())?
                    .ok_or_else(|| DatabaseError::NotFound {
                        key: key.to_string(),
                    })?;

                let value = entry.value().to_vec();
                let result = bincode::deserialize(&value)?;

                Ok(result)
            }
            Transaction::Write(_) => Err(DatabaseError::Transaction(
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
                    let value = bincode::deserialize(&value_guard.value())?;
                    result.push((key_guard.value().to_owned(), value));
                }

                Ok(result.into_iter())
            }
            Transaction::Write(_) => Err(DatabaseError::Transaction(
                "Cannot read from write transaction".to_string(),
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
            Transaction::Read(_) => Err(DatabaseError::Transaction(
                "Cannot truncate table in read transaction".to_string(),
            )),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DatabaseClient, ReDbClient};
    use std::fs;
    use std::path::PathBuf;

    fn random_string(length: usize) -> String {
        use rand::{distr::Alphanumeric, Rng};

        rand::rng()
            .sample_iter(Alphanumeric)
            .take(length)
            .map(char::from)
            .collect()
    }

    fn random_db_name() -> String {
        format!("Test_{}.db", random_string(10))
    }
    #[test]
    fn scan() {
        let tests_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests");
        fs::create_dir_all(&tests_path).unwrap();
        let db_name = random_db_name();
        let client: ReDbClient = ReDbClient::new(tests_path.join(&db_name)).unwrap();
        let bincode_table = BincodeTable::new("test");

        {
            let mut write = client.begin_write().unwrap();
            bincode_table
                .insert(&mut write, "1".to_string(), &1)
                .unwrap();
            bincode_table
                .insert(&mut write, "2".to_string(), &2)
                .unwrap();
            bincode_table
                .insert(&mut write, "3".to_string(), &3)
                .unwrap();
            write.commit().unwrap();
        }

        let expected = vec![
            ("1".to_string(), 1),
            ("2".to_string(), 2),
            ("3".to_string(), 3),
        ];
        {
            let read = client.begin_read().unwrap();

            assert_eq!(
                bincode_table.scan(&read).unwrap().collect::<Vec<_>>(),
                expected
            );
        }
        std::fs::remove_file(tests_path.join(&db_name)).unwrap();
    }
}
