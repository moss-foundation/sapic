pub mod operations;

use moss_db::{
    DatabaseResult, Transaction, anyvalue_enum::AnyValueEnum, bincode_table::BincodeTable,
};
use serde_json::Value as JsonValue;
use std::{any::TypeId, collections::HashMap};

use crate::primitives::segkey::SegKeyBuf;

pub trait Transactional {
    fn begin_write(&self) -> DatabaseResult<Transaction>;
    fn begin_read(&self) -> DatabaseResult<Transaction>;
}

pub trait Storage {
    fn dump(&self) -> DatabaseResult<HashMap<String, JsonValue>>;
}

pub type StoreTypeId = TypeId;
pub type SegBinTable = BincodeTable<'static, SegKeyBuf, AnyValueEnum>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        WorkspaceStorage, primitives::segkey::SegKey, storage::operations::PutItem,
        workspace_storage::WorkspaceStorageImpl,
    };
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct TestData {
        message: String,
        number: i32,
    }

    #[test]
    pub fn test_dump() {
        let storage = WorkspaceStorageImpl::new("tests").unwrap();
        let store = storage.item_store();

        let test_data = TestData {
            message: "Test".to_string(),
            number: 3,
        };
        let key1 = SegKey::new("1").to_segkey_buf();
        let key2 = SegKey::new("2").to_segkey_buf();
        let key3 = SegKey::new("3").to_segkey_buf();

        let value1 = AnyValueEnum::from("1".to_string());
        let value2 = AnyValueEnum::from(2);
        let value3 = AnyValueEnum::serialize(&test_data).unwrap();

        PutItem::put(store.as_ref(), key1, value1.clone()).unwrap();
        PutItem::put(store.as_ref(), key2, value2.clone()).unwrap();
        PutItem::put(store.as_ref(), key3, value3.clone()).unwrap();

        let dumped = storage.dump().unwrap();

        // Each store has one entry
        assert_eq!(dumped.len(), 2);

        let items_dump = dumped.get("table:items").unwrap();
        assert_eq!(items_dump["1"], serde_json::to_value(&value1).unwrap());
        assert_eq!(items_dump["2"], serde_json::to_value(&value2).unwrap());
        assert_eq!(items_dump["3"], serde_json::to_value(&value3).unwrap());
        dbg!(items_dump);
    }
}
