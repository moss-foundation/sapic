use moss_db::anyvalue_enum::AnyValueEnum;

use crate::{
    primitives::segkey::SegKeyBuf,
    storage::operations::{
        GetItem, ListByPrefix, PutItem, RemoveItem, TransactionalGetItem,
        TransactionalListByPrefix, TransactionalPutItem, TransactionalRemoveItem,
    },
};

pub mod item_store;
pub mod variable_store;

pub trait WorkspaceVariableStore: Send + Sync {}
pub trait WorkspaceItemStore:
    ListByPrefix<Key = SegKeyBuf, Entity = AnyValueEnum>
    + TransactionalListByPrefix<Key = SegKeyBuf, Entity = AnyValueEnum>
    + PutItem<Key = SegKeyBuf, Entity = AnyValueEnum>
    + TransactionalPutItem<Key = SegKeyBuf, Entity = AnyValueEnum>
    + GetItem<Key = SegKeyBuf, Entity = AnyValueEnum>
    + TransactionalGetItem<Key = SegKeyBuf, Entity = AnyValueEnum>
    + RemoveItem<Key = SegKeyBuf, Entity = AnyValueEnum>
    + TransactionalRemoveItem<Key = SegKeyBuf, Entity = AnyValueEnum>
    + Send
    + Sync
{
}
