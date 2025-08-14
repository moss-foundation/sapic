use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// @category Type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "types.ts")]
pub struct Contributor {
    pub name: String,
    pub avatar_url: String,
}

/// @category Type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct RepositoryInfo {
    /// A timestamp like 2024-10-05T12:19:15Z
    pub created_at: String,
    pub updated_at: String,
    pub owner: String,
}

/// @category Type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct UserInfo {
    pub name: String,
    pub email: String,
}
