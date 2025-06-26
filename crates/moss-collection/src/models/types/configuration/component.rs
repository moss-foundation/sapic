use serde::{Deserialize, Serialize};
use ts_rs::TS;

// #########################################################
// ###                      Item                         ###
// #########################################################

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
// #[serde(rename_all = "snake_case")]
// #[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum ComponentItemConfigurationModel {}

// #########################################################
// ###                      Dir                          ###
// #########################################################

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
// #[serde(rename_all = "snake_case")]
// #[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum ComponentDirConfigurationModel {}
