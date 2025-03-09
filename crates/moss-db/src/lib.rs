pub mod bincode_store;
pub mod bincode_table;

pub mod encrypted_bincode_store;
pub mod encrypted_bincode_table;

pub mod sled;

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
    fn begin_write(&self) -> Result<InnerWriteTransaction>;
    fn begin_read(&self) -> Result<InnerReadTransaction>;
}

pub struct ReDbClient(Arc<Database>);

impl ReDbClient {
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        Ok(Self(Arc::new(Database::create(path)?)))
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
