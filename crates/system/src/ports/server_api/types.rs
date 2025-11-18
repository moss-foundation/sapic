use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
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
