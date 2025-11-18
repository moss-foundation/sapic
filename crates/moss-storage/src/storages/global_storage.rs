use async_trait::async_trait;
use moss_db::{
    DatabaseClient, DatabaseClientWithContext, DatabaseResult, ReDbClient, Table, Transaction,
    bincode_table::BincodeTable, primitives::AnyValue,
};
use redb::TableHandle;
use sapic_core::context::AnyAsyncContext;
use serde_json::{Value as JsonValue, json};
use std::{any::TypeId, collections::HashMap, path::Path, sync::Arc};

use crate::{
    GlobalStorage,
    global_storage::stores::{
        GlobalItemStore, GlobalLogStore, item_store::GlobalItemStoreImpl,
        log_store::GlobalLogStoreImpl,
    },
    primitives::segkey::SegKeyBuf,
    storage::{SegBinTable, Storage, StoreTypeId, Transactional, TransactionalWithContext},
};

pub mod stores;

pub const TABLE_ITEMS: BincodeTable<SegKeyBuf, AnyValue> = BincodeTable::new("items");
pub const TABLE_LOGS: BincodeTable<SegKeyBuf, AnyValue> = BincodeTable::new("logs");
pub struct GlobalStorageImpl {
    client: ReDbClient,
    tables: HashMap<StoreTypeId, Arc<SegBinTable>>,
}

const DB_NAME: &str = "state.db";

impl GlobalStorageImpl {
    pub fn new(path: impl AsRef<Path>) -> DatabaseResult<Self> {
        let mut client = ReDbClient::new(path.as_ref().join(DB_NAME))?;

        let mut tables = HashMap::new();
        for (type_id, table) in [
            (TypeId::of::<GlobalItemStoreImpl>(), TABLE_ITEMS),
            (TypeId::of::<GlobalLogStoreImpl>(), TABLE_LOGS),
        ] {
            client = client.with_table(&table)?;
            tables.insert(type_id, Arc::new(table));
        }

        Ok(Self { client, tables })
    }
}

#[async_trait]
impl<Context> Storage<Context> for GlobalStorageImpl
where
    Context: AnyAsyncContext,
{
    async fn dump(&self, ctx: &Context) -> DatabaseResult<HashMap<String, JsonValue>> {
        let read_txn = self.client.begin_read_with_context(ctx).await?;
        let mut result = HashMap::new();
        for table in self.tables.values() {
            let name = table.table_definition().name().to_string();
            let mut table_entries = HashMap::new();
            for (k, v) in table.scan_with_context(ctx, &read_txn).await? {
                table_entries.insert(
                    k.to_string(),
                    serde_json::from_slice::<JsonValue>(v.as_bytes())?,
                );
            }
            result.insert(format!("table:{}", name), json!(table_entries));
        }
        Ok(result)
    }
}

#[async_trait]
impl<Context> TransactionalWithContext<Context> for GlobalStorageImpl
where
    Context: AnyAsyncContext,
{
    async fn begin_write_with_context(&self, ctx: &Context) -> DatabaseResult<Transaction> {
        self.client.begin_write_with_context(ctx).await
    }

    async fn begin_read_with_context(&self, ctx: &Context) -> DatabaseResult<Transaction> {
        self.client.begin_read_with_context(ctx).await
    }
}

impl Transactional for GlobalStorageImpl {
    fn begin_write(&self) -> DatabaseResult<Transaction> {
        self.client.begin_write()
    }

    fn begin_read(&self) -> DatabaseResult<Transaction> {
        self.client.begin_read()
    }
}

#[async_trait]
impl<Context> GlobalStorage<Context> for GlobalStorageImpl
where
    Context: AnyAsyncContext,
{
    fn item_store(&self) -> Arc<dyn GlobalItemStore<Context>> {
        let client = self.client.clone();
        let table = self
            .tables
            .get(&TypeId::of::<GlobalItemStoreImpl>())
            .unwrap()
            .clone();
        Arc::new(GlobalItemStoreImpl::new(client, table))
    }

    fn log_store(&self) -> Arc<dyn GlobalLogStore<Context>> {
        let client = self.client.clone();
        let table = self
            .tables
            .get(&TypeId::of::<GlobalLogStoreImpl>())
            .unwrap()
            .clone();
        Arc::new(GlobalLogStoreImpl::new(client, table))
    }
}
