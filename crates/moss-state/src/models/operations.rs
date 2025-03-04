use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::types::{Defaults, Preferences};

#[derive(Debug, Deserialize, Serialize, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operation.ts")]
pub struct DescribeAppStateOutput {
    pub preferences: Preferences,
    pub defaults: Defaults,
}
