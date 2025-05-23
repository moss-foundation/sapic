// use crate::primitives::segkey::SegKeyBuf;
// use moss_db::{bincode_table::BincodeTable, primitives::AnyValue};
// use std::any::TypeId;
//
// pub trait Store: Send + Sync {
//     fn table(&self) -> &BincodeTable<SegKeyBuf, AnyValue>;
//     fn type_id(&self) -> TypeId
//     where
//         Self: 'static,
//     {
//         TypeId::of::<Self>()
//     }
//     fn name(&self) -> &'static str;
// }
//
// pub struct TableMetadata {}
