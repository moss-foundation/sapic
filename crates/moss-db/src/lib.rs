pub mod bincode_table;

pub mod encrypted_bincode_store;
pub mod encrypted_bincode_table;

use anyhow::Result;
use redb::{
    Database, ReadTransaction as InnerReadTransaction, WriteTransaction as InnerWriteTransaction,
};

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
}

impl DatabaseClient for ReDbClient {
    fn begin_write(&self) -> Result<Transaction> {
        Ok(Transaction::Write(self.0.begin_write()?))
    }

    fn begin_read(&self) -> Result<Transaction> {
        Ok(Transaction::Read(self.0.begin_read()?))
    }
}
