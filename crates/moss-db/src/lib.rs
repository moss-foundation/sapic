pub mod bincode_table;

pub mod encrypted_bincode_store;
pub mod encrypted_bincode_table;

use anyhow::Result;
use bincode_table::BincodeTable;
use encrypted_bincode_table::EncryptedBincodeTable;
use redb::{
    Database, Key, ReadTransaction as InnerReadTransaction,
    WriteTransaction as InnerWriteTransaction,
};
use serde::{de::DeserializeOwned, Serialize};
use std::borrow::Borrow;
use std::hash::Hash;

use std::{path::Path, sync::Arc};

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
    fn begin_write(&self) -> Result<Transaction>;
    fn begin_read(&self) -> Result<Transaction>;
}

pub struct ReDbClient(Arc<Database>);

impl Clone for ReDbClient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl ReDbClient {
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        Ok(Self(Arc::new(Database::create(path)?)))
    }

    /// Initializes and registers a Bincode-based table within the database.
    ///
    /// # Why is this needed?
    /// ReDB lazily creates tables upon the first write transaction that accesses them.
    /// If the first operation on a table is a read, it may result in an error because
    /// the table has not yet been initialized. This method ensures that the table is
    /// properly initialized beforehand to prevent such issues.
    pub fn with_bincode_table<'a, K, V>(self, table: &BincodeTable<'a, K, V>) -> Result<Self>
    where
        K: Key + 'static + Borrow<K::SelfType<'a>> + Clone + Eq + Hash,
        for<'b> K::SelfType<'b>: ToOwned<Owned = K>,
        V: Serialize + DeserializeOwned,
    {
        let init_txn = self.0.begin_write()?;
        init_txn.open_table(table.into())?;
        init_txn.commit()?;

        Ok(self)
    }

    /// Initializes and registers an encrypted Bincode-based table within the database.
    ///
    /// # Why is this needed?
    /// Similar to `with_bincode_table`, this method ensures that an encrypted Bincode table
    /// is initialized before any read operation is performed. Without this step, an initial
    /// read operation could fail because the table has not yet been created.
    pub fn with_encrypted_bincode_table<'a, K, V>(
        self,
        table: &EncryptedBincodeTable<'a, K, V>,
    ) -> Result<Self>
    where
        K: Key + 'static + Borrow<K::SelfType<'a>> + Clone + Eq + Hash,
        for<'b> K::SelfType<'b>: ToOwned<Owned = K>,
        V: Serialize + DeserializeOwned,
    {
        let init_txn = self.0.begin_write()?;
        init_txn.open_table(table.into())?;
        init_txn.commit()?;

        Ok(self)
    }
}

impl DatabaseClient for ReDbClient {
    fn begin_write(&self) -> Result<Transaction> {
        Ok(Transaction::Write(self.0.begin_write()?))
    }

    fn begin_read(&self) -> Result<Transaction> {
        Ok(Transaction::Read(self.0.begin_read()?))
    }
}
