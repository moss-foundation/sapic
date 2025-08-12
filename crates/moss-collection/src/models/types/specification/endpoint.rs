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
pub struct HttpEndpointItemConfiguration {
    pub request_parts: HttpRequestParts,
}

/// @category Type
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
// #[serde(rename_all = "snake_case")]
// #[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum EndpointItemConfigurationModel {
    Http(HttpEndpointItemConfiguration),
}

// #########################################################
// ###                      Dir                          ###
// #########################################################

/// @category Type
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
// #[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct HttpEndpointDirConfiguration {}

/// @category Type
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
// #[serde(rename_all = "snake_case")]
// #[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum EndpointDirConfigurationModel {
    Http(HttpEndpointDirConfiguration),
}
