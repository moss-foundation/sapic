use moss_db::primitives::AnyValue;

use crate::{primitives::segkey::SegKeyBuf, storage::operations::*};

pub mod mixed_store;
pub mod unit_store;
pub mod variable_store;

// TODO: name can be changed to `VariableStore`
pub trait CollectionVariableStore: Send + Sync {}

// TODO: should be removed
pub trait CollectionUnitStore: Send + Sync {}

pub trait MixedStore:
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
