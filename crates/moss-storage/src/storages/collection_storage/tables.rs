use moss_db::bincode_table::BincodeTable;
use moss_db::primitives::AnyValue;

use crate::primitives::segkey::SegKeyBuf;

#[rustfmt::skip]
pub(super) type VariableStoreTable<'a> = BincodeTable<'a, SegKeyBuf, AnyValue>;
#[rustfmt::skip] // TODO: standardize names
pub(super) const TABLE_VARIABLES: BincodeTable<SegKeyBuf, AnyValue> = BincodeTable::new("variables");
pub(super) const UNIT_STORE: BincodeTable<SegKeyBuf, AnyValue> = BincodeTable::new("unit");
