use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// @category Operation
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "operations.ts")]
pub struct CancelRequestInput {
    pub request_id: String,
}
