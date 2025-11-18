pub mod operations;

use async_trait::async_trait;
use moss_db::{DatabaseResult, Transaction, bincode_table::BincodeTable, primitives::AnyValue};
use sapic_core::context::AnyAsyncContext;
use serde_json::Value as JsonValue;
use std::{any::TypeId, collections::HashMap};

use crate::primitives::segkey::SegKeyBuf;

#[async_trait]
pub trait TransactionalWithContext<Context: AnyAsyncContext> {
    async fn begin_write_with_context(&self, ctx: &Context) -> DatabaseResult<Transaction>;
    async fn begin_read_with_context(&self, ctx: &Context) -> DatabaseResult<Transaction>;
}

pub trait Transactional {
    fn begin_write(&self) -> DatabaseResult<Transaction>;
    fn begin_read(&self) -> DatabaseResult<Transaction>;
}

#[async_trait]
pub trait Storage<Context: AnyAsyncContext> {
    async fn dump(&self, ctx: &Context) -> DatabaseResult<HashMap<String, JsonValue>>;
}

pub type StoreTypeId = TypeId;
pub type SegBinTable = BincodeTable<'static, SegKeyBuf, AnyValue>;

#[cfg(test)]
mod tests {
    use super::*;

    use sapic_core::context::MutableContext;
    use serde::{Deserialize, Serialize};
    use std::time::Duration;

    use crate::{
        WorkspaceStorage, primitives::segkey::SegKey, storage::operations::PutItem,
        workspace_storage::WorkspaceStorageImpl,
    };

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct TestData {
        message: String,
        number: i32,
    }

    #[tokio::test]
    pub async fn test_dump() {
        let ctx = MutableContext::background_with_timeout(Duration::from_secs(10)).freeze();
        let storage = WorkspaceStorageImpl::new("tests").unwrap();
        let store = storage.item_store();

        let test_data = TestData {
            message: "Test".to_string(),
            number: 3,
        };
        let key1 = SegKey::new("1").to_segkey_buf();
        let key2 = SegKey::new("2").to_segkey_buf();
        let key3 = SegKey::new("3").to_segkey_buf();

        let value1 = AnyValue::serialize(&"1".to_string()).unwrap();
        let value2 = AnyValue::serialize(&2).unwrap();
        let value3 = AnyValue::serialize(&test_data).unwrap();

        PutItem::put(store.as_ref(), &ctx, key1, value1.clone())
            .await
            .unwrap();
        PutItem::put(store.as_ref(), &ctx, key2, value2.clone())
            .await
            .unwrap();
        PutItem::put(store.as_ref(), &ctx, key3, value3.clone())
            .await
            .unwrap();

        let dumped = storage.dump(&ctx).await.unwrap();

        // Each store has one entry
        assert_eq!(dumped.len(), 2);

        let items_dump = dumped.get("table:items").unwrap();
        assert_eq!(items_dump["1"], JsonValue::String("1".to_string()));
        assert_eq!(items_dump["2"], JsonValue::Number(2.into()));
        assert_eq!(items_dump["3"], serde_json::to_value(&test_data).unwrap());
    }
}
