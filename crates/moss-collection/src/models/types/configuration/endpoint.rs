use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::models::types::configuration::HttpRequestParts;

// #########################################################
// ###                      Item                         ###
// #########################################################

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct HttpEndpointItemConfiguration {
    pub request_parts: HttpRequestParts,
}

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

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
// #[serde(rename_all = "snake_case")]
// #[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum EndpointDirConfigurationModel {}
