use moss_db::{bincode_table::BincodeTable, primitives::AnyValue};

use crate::primitives::segkey::SegKeyBuf;

#[rustfmt::skip]
pub(super) type VariableStoreTable<'a> = BincodeTable<'a, SegKeyBuf, AnyValue>;

#[rustfmt::skip] // TODO: standardize names
pub(super) const TABLE_VARIABLES: BincodeTable<SegKeyBuf, AnyValue> = BincodeTable::new("variables");
pub(super) const ITEM_STORE: BincodeTable<SegKeyBuf, AnyValue> = BincodeTable::new("item");
