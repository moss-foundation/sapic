use std::sync::Arc;

use async_trait::async_trait;
use moss_db::{Transaction, common::DatabaseError};

pub trait NamespacedStore<T: Send + Sync + ?Sized> {
    fn namespaces(self: Arc<Self>) -> Arc<T>;
}

#[async_trait]
pub trait Transactional {
    async fn begin_write(&self) -> Result<Transaction, DatabaseError>;
    async fn begin_read(&self) -> Result<Transaction, DatabaseError>;
}
