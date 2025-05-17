pub mod operations;

use async_trait::async_trait;
use moss_db::{DatabaseResult, Transaction};
use std::{future::Future, path::Path, pin::Pin};

// FIXME: Does this need to be an async trait?
#[async_trait]
pub trait Transactional {
    async fn begin_write(&self) -> DatabaseResult<Transaction>;
    async fn begin_read(&self) -> DatabaseResult<Transaction>;
}

#[async_trait]
pub trait ResettableStorage {
    async fn reset(
        &self,
        path: &Path,
        after_drop: Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>>, // TODO: change to DatabaseResult
    ) -> anyhow::Result<()>; // TODO: change to DatabaseResult
}
