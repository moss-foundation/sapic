use async_trait::async_trait;
use moss_db::{common::DatabaseError, Transaction};

#[async_trait]
pub trait Transactional {
    async fn begin_write(&self) -> Result<Transaction, DatabaseError>;
    async fn begin_read(&self) -> Result<Transaction, DatabaseError>;
}
