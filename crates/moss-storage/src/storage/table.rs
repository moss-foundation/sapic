use moss_db::{bincode_table::BincodeTable, primitives::AnyValue};
use std::any::TypeId;

use crate::primitives::segkey::SegKeyBuf;

pub trait Table {
    fn definition(&self) -> BincodeTable<SegKeyBuf, AnyValue>;
    fn type_id(&self) -> TypeId
    where
        Self: 'static,
    {
        TypeId::of::<Self>()
    }
    // fn name(&self) -> &str;
    // fn metadata(&self) -> TableMetadata;
}

pub struct TableMetadata {}
