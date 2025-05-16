use crate::primitives::segkey::SegKeyBuf;
use crate::workspace_storage::entities::variable_store_entities::VariableEntity;
use moss_db::bincode_table::BincodeTable;
use moss_db::primitives::AnyValue;

#[rustfmt::skip]
pub(super) type VariableStoreTable<'a> = BincodeTable<'a, SegKeyBuf, VariableEntity>;
#[rustfmt::skip] // TODO: standardize names
pub(super) const TABLE_VARIABLES: BincodeTable<SegKeyBuf, VariableEntity> = BincodeTable::new("variables");
pub(super) const UNIT_STORE: BincodeTable<SegKeyBuf, AnyValue> = BincodeTable::new("unit");
