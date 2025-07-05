use moss_db::primitives::AnyValue;

use crate::{primitives::segkey::SegKeyBuf, storage::operations::*};

pub mod resource_store;
pub mod variable_store;

pub trait CollectionVariableStore: Send + Sync {}

pub trait CollectionResourceStore:
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
