pub mod bincode_table;
pub mod common;
pub mod encrypted_bincode_table;

use anyhow::Result;
use redb::{Database, Key, TableDefinition};
use serde::{de::DeserializeOwned, Serialize};
use std::{borrow::Borrow, path::Path, sync::Arc};
use tokio::sync::Notify;

use crate::common::Transaction;

pub trait DatabaseClient: Sized {
    fn begin_write(&self) -> Result<Transaction>;
    fn begin_read(&self) -> Result<Transaction>;
}

pub enum ClientState<C> {
    Loaded(C),
    Reloading { notify: Arc<Notify> },
}
pub struct ReDbClient {
    db: Arc<Database>,
}

impl Clone for ReDbClient {
    fn clone(&self) -> Self {
        Self {
            db: Arc::clone(&self.db),
        }
    }
}

pub trait Table<'a, K, V>
where
    K: Key + 'static + Borrow<K::SelfType<'a>> + Clone + Eq,
    for<'b> K::SelfType<'b>: ToOwned<Owned = K>,
    V: Serialize + DeserializeOwned,
{
    fn table_definition(&self) -> TableDefinition<'a, K, Vec<u8>>;
}

impl ReDbClient {
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        // Using compact() on an empty ReDb database will shrink its file size by 1 mb
        let mut database = Database::create(path)?;
        database.compact()?;
        Ok(Self {
            db: Arc::new(database),
        })
    }

    /// Initializes and registers a Bincode-based table within the database.
    ///
    /// # Why is this needed?
    /// ReDB lazily creates tables upon the first write transaction that accesses them.
    /// If the first operation on a table is a read, it may result in an error because
    /// the table has not yet been initialized. This method ensures that the table is
    /// properly initialized beforehand to prevent such issues.
    pub fn with_table<'a, K, V>(self, table: &dyn Table<'a, K, V>) -> Result<Self>
    where
        K: Key + 'static + Borrow<K::SelfType<'a>> + Clone + Eq,
        for<'b> K::SelfType<'b>: ToOwned<Owned = K>,
        V: Serialize + DeserializeOwned,
    {
        let table_def = table.table_definition();
        let init_txn = self.db.begin_write()?;
        init_txn.open_table(table_def)?;
        init_txn.commit()?;

        Ok(self)
    }
}

impl DatabaseClient for ReDbClient {
    fn begin_write(&self) -> Result<Transaction> {
        Ok(Transaction::Write(self.db.begin_write()?))
    }

    fn begin_read(&self) -> Result<Transaction> {
        Ok(Transaction::Read(self.db.begin_read()?))
    }
}
