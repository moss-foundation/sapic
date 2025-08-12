use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::models::types::specification::HttpRequestParts;

// #########################################################
// ###                      Item                         ###
// #########################################################

/// @category Type
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct ItemHttpRequestConfiguration {
    pub request_parts: HttpRequestParts,
}

/// @category Type
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum ItemRequestConfigurationModel {
    Http(ItemHttpRequestConfiguration),
}

// #########################################################
// ###                      Dir                          ###
// #########################################################

/// @category Type
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
// #[serde(rename_all = "snake_case")]
// #[ts(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct DirHttpConfigurationModel {}

/// @category Type
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum RequestDirConfigurationModel {
    Http(DirHttpConfigurationModel),
}
