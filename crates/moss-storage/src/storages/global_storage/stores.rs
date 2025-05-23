use crate::primitives::segkey::SegKeyBuf;
use crate::storage::operations::{ListByPrefix, PutItem, RemoveItem};
use moss_db::primitives::AnyValue;

pub mod item_store;

pub trait GlobalItemStore:
    ListByPrefix<Key = SegKeyBuf, Entity = AnyValue>
    + PutItem<Key = SegKeyBuf, Entity = AnyValue>
    + RemoveItem<Key = SegKeyBuf>
    + Send
    + Sync
{
}
