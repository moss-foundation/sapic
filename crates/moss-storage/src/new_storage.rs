use async_trait::async_trait;
use moss_db::{DatabaseResult, Transaction};
use serde_json::Value as JsonValue;
use std::any::TypeId;
use std::collections::HashMap;
use std::path::Path;
use std::pin::Pin;
use std::sync::Arc;

use crate::storage::Transactional;
use crate::storage::table::Store;

mod new_storage;

// pub trait NewTransactional {
//     fn begin_write(&self) -> DatabaseResult<Transaction>;
//     fn begin_read(&self) -> DatabaseResult<Transaction>;
// }

// pub trait Dump {
//     fn dump(&self) -> DatabaseResult<HashMap<String, JsonValue>>;
// }

// pub trait NewStorage: NewTransactional + Dump + Send + Sync {
//     fn table(&self, id: &TypeId) -> DatabaseResult<Arc<dyn Table>>;
// }
