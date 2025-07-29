use moss_applib::context::AnyAsyncContext;
use moss_db::primitives::AnyValue;

use crate::{primitives::segkey::SegKeyBuf, storage::operations::*};

pub mod resource_store;

pub trait CollectionResourceStore<Context: AnyAsyncContext>:
    ListByPrefix<Context, Key = SegKeyBuf, Entity = AnyValue>
    + TransactionalListByPrefix<Context, Key = SegKeyBuf, Entity = AnyValue>
    + PutItem<Context, Key = SegKeyBuf, Entity = AnyValue>
    + TransactionalPutItem<Context, Key = SegKeyBuf, Entity = AnyValue>
    + GetItem<Context, Key = SegKeyBuf, Entity = AnyValue>
    + TransactionalGetItem<Context, Key = SegKeyBuf, Entity = AnyValue>
    + RemoveItem<Context, Key = SegKeyBuf, Entity = AnyValue>
    + TransactionalRemoveItem<Context, Key = SegKeyBuf, Entity = AnyValue>
    + Send
    + Sync
{
}
