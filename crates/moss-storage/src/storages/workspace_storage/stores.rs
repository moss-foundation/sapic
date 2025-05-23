use moss_db::primitives::AnyValue;

use crate::primitives::segkey::SegKeyBuf;
use crate::storage::operations::{ListByPrefix, PutItem, RemoveItem, TransactionalGetItem};

pub mod item_store;
pub mod variable_store;

pub trait WorkspaceVariableStore: Send + Sync {}
pub trait WorkspaceItemStore:
    ListByPrefix<Key = SegKeyBuf, Entity = AnyValue>
    + PutItem<Key = SegKeyBuf, Entity = AnyValue>
    + TransactionalGetItem<Key = SegKeyBuf, Entity = AnyValue>
    + RemoveItem<Key = SegKeyBuf>
    + Send
    + Sync
{
}
