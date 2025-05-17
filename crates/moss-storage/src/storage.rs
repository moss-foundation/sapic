pub mod operations;
pub mod table;

use async_trait::async_trait;
use moss_db::{DatabaseResult, ReDbClient, Transaction, primitives::AnyValue};
use std::{any::TypeId, collections::HashMap, future::Future, path::Path, pin::Pin, sync::Arc};
use table::Table;

use crate::primitives::segkey::SegKeyBuf;

// FIXME: Does this need to be an async trait?
#[async_trait]
pub trait Transactional {
    async fn begin_write(&self) -> DatabaseResult<Transaction>;
    async fn begin_read(&self) -> DatabaseResult<Transaction>;
}

pub trait Storage: Transactional + Send + Sync {
    // fn table(&self) -> DatabaseResult<Arc<dyn Table<SegKeyBuf, AnyValue>>>;
    // fn dump(&self) -> DatabaseResult<HashMap<String, JsonValue>>;
}

#[async_trait]
pub trait ResettableStorage {
    async fn reset(
        &self,
        path: &Path,
        after_drop: Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>>, // TODO: change to DatabaseResult
    ) -> anyhow::Result<()>; // TODO: change to DatabaseResult
}

pub struct StorageHandle {
    client: ReDbClient,
    tables: HashMap<TypeId, Arc<dyn Table>>,
}

impl StorageHandle {
    pub fn new(path: impl AsRef<Path>, tables: Vec<impl Table + 'static>) -> DatabaseResult<Self> {
        let mut client = ReDbClient::new(path.as_ref())?;
        let mut known_tables = HashMap::new();
        for table in tables {
            client = client.with_table(&table.definition())?;
            known_tables.insert(table.type_id(), Arc::from(table) as Arc<dyn Table>);
        }

        Ok(Self {
            client,
            tables: known_tables,
        })
    }
}
