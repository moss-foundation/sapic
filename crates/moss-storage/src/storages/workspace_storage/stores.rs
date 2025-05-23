use moss_db::primitives::AnyValue;

use crate::primitives::segkey::SegKeyBuf;
use crate::storage::operations::{
    GetItem, ListByPrefix, PutItem, RemoveItem, TransactionalGetItem, TransactionalListByPrefix,
    TransactionalPutItem, TransactionalRemoveItem,
};

pub mod item_store;
pub mod variable_store;

pub trait WorkspaceVariableStore: Send + Sync {}
pub trait WorkspaceItemStore:
    ListByPrefix<Key = SegKeyBuf, Entity = AnyValue>
    + TransactionalListByPrefix<Key = SegKeyBuf, Entity = AnyValue>
    + PutItem<Key = SegKeyBuf, Entity = AnyValue>
    + TransactionalPutItem<Key = SegKeyBuf, Entity = AnyValue>
    + GetItem<Key = SegKeyBuf, Entity = AnyValue>
    + TransactionalGetItem<Key = SegKeyBuf, Entity = AnyValue>
    + RemoveItem<Key = SegKeyBuf, Entity = AnyValue>
    + TransactionalRemoveItem<Key = SegKeyBuf, Entity = AnyValue>
    + Send
    + Sync
{
}
