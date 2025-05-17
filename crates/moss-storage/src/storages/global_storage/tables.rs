use moss_db::{bincode_table::BincodeTable, primitives::AnyValue};

use crate::primitives::segkey::SegKeyBuf;

#[rustfmt::skip]
pub(in super::super) const ITEM_STORE: BincodeTable<SegKeyBuf, AnyValue> = BincodeTable::new("item");
