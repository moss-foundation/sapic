use moss_db::primitives::AnyValue;

use crate::{primitives::segkey::SegKeyBuf, storage::operations::*};

pub mod item_store;
pub mod log_store;

pub trait GlobalItemStore:
    ListByPrefix<Key = SegKeyBuf, Entity = AnyValue>
    + TransactionalListByPrefix<Key = SegKeyBuf, Entity = AnyValue>
    + PutItem<Key = SegKeyBuf, Entity = AnyValue>
    + TransactionalPutItem<Key = SegKeyBuf, Entity = AnyValue>
    + RemoveItem<Key = SegKeyBuf, Entity = AnyValue>
    + TransactionalRemoveItem<Key = SegKeyBuf, Entity = AnyValue>
    + GetItem<Key = SegKeyBuf, Entity = AnyValue>
    + TransactionalGetItem<Key = SegKeyBuf, Entity = AnyValue>
    + RemoveByPrefix<Key = SegKeyBuf, Entity = AnyValue>
    + TransactionalRemoveByPrefix<Key = SegKeyBuf, Entity = AnyValue>
    + Send
    + Sync
{
}

pub trait GlobalLogStore:
    PutItem<Key = SegKeyBuf, Entity = AnyValue>
    + TransactionalPutItem<Key = SegKeyBuf, Entity = AnyValue>
    + RemoveItem<Key = SegKeyBuf, Entity = AnyValue>
    + TransactionalRemoveItem<Key = SegKeyBuf, Entity = AnyValue>
    + GetItem<Key = SegKeyBuf, Entity = AnyValue>
    + TransactionalGetItem<Key = SegKeyBuf, Entity = AnyValue>
    + Send
    + Sync
{
}
