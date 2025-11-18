pub mod bincode_table;
pub mod common;
pub mod encrypted_bincode_table;
pub mod error;
pub mod primitives;

pub use common::*;
pub use error::*;

use anyhow::Result;
use async_trait::async_trait;
use redb::{Builder, Database, Key, TableDefinition};
use sapic_core::context::AnyAsyncContext;
use serde::{Serialize, de::DeserializeOwned};
use std::{borrow::Borrow, path::Path, sync::Arc};
use tokio::sync::Notify;

#[async_trait]
pub trait DatabaseClientWithContext<Context: AnyAsyncContext>: Sized {
    async fn begin_write_with_context(&self, ctx: &Context) -> Result<Transaction, DatabaseError>;
    async fn begin_read_with_context(&self, ctx: &Context) -> Result<Transaction, DatabaseError>;
}

pub trait DatabaseClient {
    fn begin_write(&self) -> Result<Transaction, DatabaseError>;
    fn begin_read(&self) -> Result<Transaction, DatabaseError>;
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
        let mut database = Builder::new()
            .create_with_file_format_v3(true)
            .create(path)?;
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
    fn begin_write(&self) -> Result<Transaction, DatabaseError> {
        Ok(Transaction::Write(self.db.begin_write()?))
    }

    fn begin_read(&self) -> Result<Transaction, DatabaseError> {
        Ok(Transaction::Read(self.db.begin_read()?))
    }
}

#[async_trait]
impl<Context> DatabaseClientWithContext<Context> for ReDbClient
where
    Context: AnyAsyncContext,
{
    async fn begin_write_with_context(&self, ctx: &Context) -> Result<Transaction, DatabaseError> {
        if let Some(reason) = ctx.done() {
            return Err(DatabaseError::Canceled(reason));
        }

        tokio::time::timeout(ctx.deadline(), async move {
            Ok(Transaction::Write(self.db.begin_write()?))
        })
        .await
        .map_err(|_| DatabaseError::Timeout("begin_write".to_string()))?
    }

    async fn begin_read_with_context(&self, ctx: &Context) -> Result<Transaction, DatabaseError> {
        if let Some(reason) = ctx.done() {
            return Err(DatabaseError::Canceled(reason));
        }

        tokio::time::timeout(ctx.deadline(), async move {
            Ok(Transaction::Read(self.db.begin_read()?))
        })
        .await
        .map_err(|_| DatabaseError::Timeout("begin_read".to_string()))?
    }
}
