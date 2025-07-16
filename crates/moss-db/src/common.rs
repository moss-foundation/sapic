use redb::{ReadTransaction as InnerReadTransaction, WriteTransaction as InnerWriteTransaction};

use crate::DatabaseError;

pub type AnyEntity = Vec<u8>;

pub enum Transaction {
    Read(InnerReadTransaction),
    Write(InnerWriteTransaction),
}

impl Transaction {
    pub fn commit(self) -> anyhow::Result<(), DatabaseError> {
        match self {
            Transaction::Read(_) => Ok(()),
            Transaction::Write(txn) => Ok(txn.commit()?),
        }
    }
}
