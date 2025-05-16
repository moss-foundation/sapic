use moss_db::{bincode_table::BincodeTable, primitives::AnyValue};

use crate::{
    primitives::segkey::SegKeyBuf,
    workspace_storage::entities::variable_store_entities::VariableEntity,
};

#[rustfmt::skip]
pub(super) type VariableStoreTable<'a> = BincodeTable<'a, SegKeyBuf, VariableEntity>;

#[rustfmt::skip] // TODO: standardize names
pub(super) const TABLE_VARIABLES: BincodeTable<SegKeyBuf, VariableEntity> = BincodeTable::new("variables");
pub(super) const ITEM_STORE: BincodeTable<SegKeyBuf, AnyValue> = BincodeTable::new("item");
