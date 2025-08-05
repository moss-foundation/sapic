pub mod variable_store;

use crate::{
    primitives::segkey::SegKeyBuf,
    storage::operations::{
        GetItem, PutItem, RemoveItem, TransactionalGetItem, TransactionalPutItem,
        TransactionalRemoveItem,
    },
};
use moss_applib::context::AnyAsyncContext;
use moss_db::primitives::AnyValue;

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
