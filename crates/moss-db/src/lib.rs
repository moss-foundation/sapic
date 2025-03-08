pub mod bincode_store;
pub mod bincode_table;

pub mod encrypted_bincode_store;
pub mod encrypted_bincode_table;

pub mod sled;

use anyhow::Result;
use redb::{
    Database, Key as ReDbKey, ReadTransaction as InnerReadTransaction,
    WriteTransaction as InnerWriteTransaction,
};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::{borrow::Borrow, path::Path};

pub enum Transaction {
    Read(InnerReadTransaction),
    Write(InnerWriteTransaction),
}

impl Transaction {
    pub fn commit(self) -> Result<()> {
        match self {
            Transaction::Read(_) => Ok(()),
            Transaction::Write(txn) => Ok(txn.commit()?),
        }
    }
}

pub trait DatabaseClient: Sized {
    fn begin_write(&self) -> Result<InnerWriteTransaction>;
    fn begin_read(&self) -> Result<InnerReadTransaction>;
}

pub struct ReDbClient(Database);

impl ReDbClient {
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        Ok(Self(Database::create(path)?))
    }
}

impl DatabaseClient for ReDbClient {
    fn begin_write(&self) -> Result<InnerWriteTransaction> {
        Ok(self.0.begin_write()?)
    }

    fn begin_read(&self) -> Result<InnerReadTransaction> {
        Ok(self.0.begin_read()?)
    }
}

pub trait Store<'a, K, V>
where
    K: ReDbKey + 'static + Borrow<K::SelfType<'a>>,
    V: Serialize + DeserializeOwned,
{
    type Table;
    type Options;

    fn write<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(Transaction, &Self::Table, &Self::Options) -> Result<T>;
    fn read<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(Transaction, &Self::Table, &Self::Options) -> Result<T>;
}
