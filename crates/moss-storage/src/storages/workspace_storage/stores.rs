use moss_db::primitives::AnyValue;
use sapic_core::context::AnyAsyncContext;

use crate::{
    primitives::segkey::SegKeyBuf,
    storage::{TransactionalWithContext, operations::*},
};

pub mod item_store;

pub trait WorkspaceItemStore<Context: AnyAsyncContext>:
    TransactionalWithContext<Context>
    + ListByPrefix<Context, Key = SegKeyBuf, Entity = AnyValue>
    + TransactionalListByPrefix<Context, Key = SegKeyBuf, Entity = AnyValue>
    + PutItem<Context, Key = SegKeyBuf, Entity = AnyValue>
    + TransactionalPutItem<Context, Key = SegKeyBuf, Entity = AnyValue>
    + GetItem<Context, Key = SegKeyBuf, Entity = AnyValue>
    + TransactionalGetItem<Context, Key = SegKeyBuf, Entity = AnyValue>
    + RemoveItem<Context, Key = SegKeyBuf, Entity = AnyValue>
    + TransactionalRemoveItem<Context, Key = SegKeyBuf, Entity = AnyValue>
    + RemoveByPrefix<Context, Key = SegKeyBuf, Entity = AnyValue>
    + TransactionalRemoveByPrefix<Context, Key = SegKeyBuf, Entity = AnyValue>
    + TransactionalListByPrefix<Context, Key = SegKeyBuf, Entity = AnyValue>
    + Send
    + Sync
{
}
