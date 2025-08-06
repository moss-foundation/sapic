pub mod variable_store;

use moss_applib::context::AnyAsyncContext;
use moss_db::primitives::AnyValue;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::{
    primitives::segkey::SegKeyBuf,
    storage::operations::{
        GetItem, PutItem, RemoveItem, TransactionalGetItem, TransactionalPutItem,
        TransactionalRemoveItem,
    },
};

pub trait VariableStore<Context: AnyAsyncContext>:
    PutItem<Context, Key = SegKeyBuf, Entity = AnyValue>
    + TransactionalPutItem<Context, Key = SegKeyBuf, Entity = AnyValue>
    + GetItem<Context, Key = SegKeyBuf, Entity = AnyValue>
    + TransactionalGetItem<Context, Key = SegKeyBuf, Entity = AnyValue>
    + RemoveItem<Context, Key = SegKeyBuf, Entity = AnyValue>
    + TransactionalRemoveItem<Context, Key = SegKeyBuf, Entity = AnyValue>
    + Send
    + Sync
{
}

// FIXME: Where to put this?
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VariableEntity {
    // Using HclExpression here will fail for some reason
    pub local_value: Option<JsonValue>,
    pub order: isize,
}
