use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct Options {
    pub timeout: Option<u64>,
}
