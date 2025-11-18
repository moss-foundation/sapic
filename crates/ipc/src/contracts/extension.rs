use serde::Serialize;
use ts_rs::TS;

use moss_extension::models::types::ExtensionInfo;

/// @category Operation
#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "operations.ts")]
pub struct ListExtensionsOutput(#[ts(type = "ExtensionInfo[]")] pub Vec<ExtensionInfo>);
