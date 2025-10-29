use serde::Serialize;
use ts_rs::TS;

// #########################################################
// ###                    Extensions                     ###
// #########################################################
/// @category Type
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct ExtensionInfo {
    pub id: String,
    pub external_id: String,
    pub name: String,
    pub authors: Vec<String>,
    pub description: String,
    pub repository: String,
    pub downloads: u64,
    pub created_at: String,
    pub updated_at: String,
    pub latest_version: String,
}
