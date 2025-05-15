pub mod store_impl;

use moss_db::{bincode_table::BincodeTable, primitives::AnyValue};

use crate::{primitives::segkey::SegKeyBuf, storage::operations::*};

pub(crate) type ItemStoreTable<'a> = BincodeTable<'a, SegKeyBuf, AnyValue>;

pub trait ItemStore<K, E>:
    TransactionalGetItem<Key = K, Entity = E>
    + GetItem<Key = K, Entity = E>
    + TransactionalListByPrefix<Key = K, Entity = E>
    + ListByPrefix<Key = K, Entity = E>
    + TransactionalPutItem<Key = K, Entity = E>
    + PutItem<Key = K, Entity = E>
    + TransactionalRemoveItem<Key = K>
    + RemoveItem<Key = K>
    + TransactionalRekeyItem<Key = K, Entity = E>
    + RekeyItem<Key = K, Entity = E>
    + Send
    + Sync
{
}
