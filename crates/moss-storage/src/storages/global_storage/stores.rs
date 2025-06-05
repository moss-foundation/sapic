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

pub mod applog_cache;
pub mod item_store;
pub mod sessionlog_cache;

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

pub trait AppLogCache:
    PutItem<Key = SegKeyBuf, Entity = AnyValue>
    + TransactionalPutItem<Key = SegKeyBuf, Entity = AnyValue>
    + RemoveItem<Key = SegKeyBuf, Entity = AnyValue>
    + TransactionalRemoveItem<Key = SegKeyBuf, Entity = AnyValue>
    + GetItem<Key = SegKeyBuf, Entity = AnyValue>
    + TransactionalGetItem<Key = SegKeyBuf, Entity = AnyValue>
    + Truncate
    + TransactionalTruncate
    + Scan<Key = SegKeyBuf, Entity = AnyValue>
    + TransactionalScan<Key = SegKeyBuf, Entity = AnyValue>
    + Send
    + Sync
{
}

pub trait SessionLogCache:
    ListByPrefix<Key = SegKeyBuf, Entity = AnyValue>
    + TransactionalListByPrefix<Key = SegKeyBuf, Entity = AnyValue>
    + PutItem<Key = SegKeyBuf, Entity = AnyValue>
    + TransactionalPutItem<Key = SegKeyBuf, Entity = AnyValue>
    + RemoveItem<Key = SegKeyBuf, Entity = AnyValue>
    + TransactionalRemoveItem<Key = SegKeyBuf, Entity = AnyValue>
    + GetItem<Key = SegKeyBuf, Entity = AnyValue>
    + TransactionalGetItem<Key = SegKeyBuf, Entity = AnyValue>
    + Truncate
    + TransactionalTruncate
    + Send
    + Sync
{
}
