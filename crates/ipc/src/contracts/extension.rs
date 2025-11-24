use sapic_base::extension::types::ExtensionInfo;
use serde::Serialize;
use ts_rs::TS;

/// @category Operation
#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "operations.ts")]
pub struct ListExtensionsOutput(#[ts(type = "ExtensionInfo[]")] pub Vec<ExtensionInfo>);
