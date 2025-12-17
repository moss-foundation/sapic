// Parse Url

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::models::types::ParsedUrl;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct ParseUrlInput {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct ParseUrlOutput(pub ParsedUrl);
