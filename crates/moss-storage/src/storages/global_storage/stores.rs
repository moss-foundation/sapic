use moss_db::primitives::AnyValue;
use redb::Key;

use crate::{
    primitives::segkey::SegKeyBuf,
    storage::operations::{
        GetItem, ListByPrefix, PutItem, RemoveItem, Scan, TransactionalGetItem,
        TransactionalListByPrefix, TransactionalPutItem, TransactionalRemoveItem,
        TransactionalScan, TransactionalTruncate, Truncate,
    },
};

pub mod item_store;

pub trait GlobalItemStore:
    ListByPrefix<Key = SegKeyBuf, Entity = AnyValue>
    + TransactionalListByPrefix<Key = SegKeyBuf, Entity = AnyValue>
    + PutItem<Key = SegKeyBuf, Entity = AnyValue>
    + TransactionalPutItem<Key = SegKeyBuf, Entity = AnyValue>
    + RemoveItem<Key = SegKeyBuf, Entity = AnyValue>
    + TransactionalRemoveItem<Key = SegKeyBuf, Entity = AnyValue>
    + GetItem<Key = SegKeyBuf, Entity = AnyValue>
    + TransactionalGetItem<Key = SegKeyBuf, Entity = AnyValue>
    + Send
    + Sync
{
}
