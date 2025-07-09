use serde::{Deserialize, Serialize};
use ts_rs::TS;

// #########################################################
// ###                      Item                         ###
// #########################################################

/// @category Type
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
// #[serde(rename_all = "snake_case")]
// #[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct SchemaItemConfigurationModel {}

// #########################################################
// ###                      Dir                          ###
// #########################################################

/// @category Type
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
// #[serde(rename_all = "snake_case")]
// #[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct SchemaDirConfigurationModel {}
