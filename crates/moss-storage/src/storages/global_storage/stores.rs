use moss_db::primitives::AnyValue;

use crate::primitives::segkey::SegKeyBuf;
use crate::storage::operations::{
    ListByPrefix, PutItem, RemoveItem, TransactionalListByPrefix, TransactionalPutItem,
    TransactionalRemoveItem,
};

pub mod item_store;

pub trait GlobalItemStore:
    ListByPrefix<Key = SegKeyBuf, Entity = AnyValue>
    + TransactionalListByPrefix<Key = SegKeyBuf, Entity = AnyValue>
    + PutItem<Key = SegKeyBuf, Entity = AnyValue>
    + TransactionalPutItem<Key = SegKeyBuf, Entity = AnyValue>
    + RemoveItem<Key = SegKeyBuf, Entity = AnyValue>
    + TransactionalRemoveItem<Key = SegKeyBuf, Entity = AnyValue>
    + Send
    + Sync
{
}
