use async_trait::async_trait;
use moss_applib::ctx::AnyAsyncContext;
use moss_db::{
    DatabaseClient, DatabaseResult, ReDbClient, Table, Transaction, bincode_table::BincodeTable,
    primitives::AnyValue,
};
use redb::TableHandle;
use serde_json::{Value as JsonValue, json};
use std::{any::TypeId, collections::HashMap, path::Path, sync::Arc};

use crate::{
    WorkspaceStorage,
    primitives::segkey::SegKeyBuf,
    storage::{SegBinTable, Storage, StoreTypeId, Transactional},
    workspace_storage::stores::{
        WorkspaceItemStore, WorkspaceVariableStore, item_store::WorkspaceItemStoreImpl,
        variable_store::WorkspaceVariableStoreImpl,
    },
};

pub mod stores;

const DB_NAME: &str = "state.db";
pub const TABLE_VARIABLES: BincodeTable<SegKeyBuf, AnyValue> = BincodeTable::new("variables");
pub const TABLE_ITEMS: BincodeTable<SegKeyBuf, AnyValue> = BincodeTable::new("items");

pub struct WorkspaceStorageImpl {
    client: ReDbClient,
    tables: HashMap<StoreTypeId, Arc<SegBinTable>>,
}

impl WorkspaceStorageImpl {
    pub fn new(path: impl AsRef<Path>) -> DatabaseResult<Self> {
        let mut client = ReDbClient::new(path.as_ref().join(DB_NAME))?;

        let mut tables = HashMap::new();
        for (type_id, table) in [
            (TypeId::of::<WorkspaceVariableStoreImpl>(), TABLE_VARIABLES),
            (TypeId::of::<WorkspaceItemStoreImpl>(), TABLE_ITEMS),
        ] {
            client = client.with_table(&table)?;
            tables.insert(type_id, Arc::new(table));
        }

        Ok(Self { client, tables })
    }
}

#[async_trait]
impl<Context> Storage<Context> for WorkspaceStorageImpl
where
    Context: AnyAsyncContext,
{
    async fn dump(&self, ctx: &Context) -> DatabaseResult<HashMap<String, JsonValue>> {
        let read_txn = self.client.begin_read(ctx).await?;
        let mut result = HashMap::new();
        for table in self.tables.values() {
            let name = table.table_definition().name().to_string();
            let mut table_entries = HashMap::new();
            for (k, v) in table.scan(ctx, &read_txn).await? {
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
impl<Context> Transactional<Context> for WorkspaceStorageImpl
where
    Context: AnyAsyncContext,
{
    async fn begin_write(&self, ctx: &Context) -> DatabaseResult<Transaction> {
        self.client.begin_write(ctx).await
    }

    async fn begin_read(&self, ctx: &Context) -> DatabaseResult<Transaction> {
        self.client.begin_read(ctx).await
    }
}

#[async_trait]
impl<Context> WorkspaceStorage<Context> for WorkspaceStorageImpl
where
    Context: AnyAsyncContext,
{
    fn variable_store(&self) -> Arc<dyn WorkspaceVariableStore<Context>> {
        let client = self.client.clone();
        let table = self
            .tables
            .get(&TypeId::of::<WorkspaceVariableStoreImpl>())
            .unwrap()
            .clone();
        Arc::new(WorkspaceVariableStoreImpl::new(client, table))
    }

    fn item_store(&self) -> Arc<dyn WorkspaceItemStore<Context>> {
        let client = self.client.clone();
        let table = self
            .tables
            .get(&TypeId::of::<WorkspaceItemStoreImpl>())
            .unwrap()
            .clone();
        Arc::new(WorkspaceItemStoreImpl::new(client, table))
    }
}
