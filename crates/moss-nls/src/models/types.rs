use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct LocaleInfo {
    pub identifier: String,
    pub display_name: String,
    pub code: String,
    #[ts(optional)]
    pub direction: Option<String>,
    #[ts(optional)]
    pub is_default: Option<bool>,
}
