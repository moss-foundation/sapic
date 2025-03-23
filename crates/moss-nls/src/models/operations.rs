use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::types::LocaleInfo;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct GetTranslationsInput {
    pub language: String,
    pub namespace: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct ListLocalesOutput {
    pub contents: Vec<LocaleInfo>,
}
