pub mod operations;

use moss_db::bincode_table::BincodeTable;
use moss_db::primitives::AnyValue;
use moss_db::{DatabaseResult, Transaction};
use serde_json::Value as JsonValue;
use std::{any::TypeId, collections::HashMap};

use crate::primitives::segkey::SegKeyBuf;

pub trait Transactional {
    fn begin_write(&self) -> DatabaseResult<Transaction>;
    fn begin_read(&self) -> DatabaseResult<Transaction>;
}

pub trait Storage {
    // TODO: How to organize the output from different tables?
    fn dump(&self) -> DatabaseResult<HashMap<String, JsonValue>>;
}

pub type StoreTypeId = TypeId;
pub type SegBinTable = BincodeTable<'static, SegKeyBuf, AnyValue>;
