use crate::primitives::segkey::SegKeyBuf;
use moss_db::{bincode_table::BincodeTable, primitives::AnyValue};
use std::any::TypeId;
use std::ops::Deref;

pub trait Table: Send + Sync {
    fn definition(&self) -> &BincodeTable<SegKeyBuf, AnyValue>;
    fn type_id(&self) -> TypeId
    where
        Self: 'static,
    {
        TypeId::of::<Self>()
    }
    fn name(&self) -> &'static str;
}

pub struct TableMetadata {}
