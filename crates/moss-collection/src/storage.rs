// pub mod request_store;
// pub mod state_db_manager;

// use anyhow::Result;
// use async_trait::async_trait;
// use moss_db::{bincode_table::BincodeTable, common::Transaction};
// use std::{collections::HashMap, future::Future, path::PathBuf, pin::Pin, sync::Arc};

// use crate::models::storage::RequestEntity;

// pub(crate) type RequestStoreTable<'a> = BincodeTable<'a, String, RequestEntity>;

// pub trait RequestStore: Send + Sync + 'static {
//     fn begin_write(&self) -> Result<(Transaction, &RequestStoreTable)>;
//     fn begin_read(&self) -> Result<(Transaction, &RequestStoreTable)>;
//     fn scan(&self) -> Result<HashMap<PathBuf, RequestEntity>>;
// }

// #[async_trait]
// pub trait StateDbManager: Send + Sync + 'static {
//     async fn reload(
//         &self,
//         path: PathBuf,
//         after_drop: Pin<Box<dyn Future<Output = Result<()>> + Send>>,
//     ) -> Result<()>;
//     async fn request_store(&self) -> Arc<dyn RequestStore>;
// }
