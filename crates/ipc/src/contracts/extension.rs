use sapic_base::extension::types::ExtensionInfo;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// @category Operation
#[derive(Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct DownloadExtensionInput {
    pub extension_id: String,
    pub version: String,
}

/// @category Operation
#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "operations.ts")]
pub struct ListExtensionsOutput(#[ts(type = "ExtensionInfo[]")] pub Vec<ExtensionInfo>);
