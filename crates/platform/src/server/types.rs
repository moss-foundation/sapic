use sapic_base::extension::types::{ExtensionInfo, ExtensionVersionInfo};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionInfoResponse {
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

impl From<ExtensionInfoResponse> for ExtensionInfo {
    fn from(response: ExtensionInfoResponse) -> Self {
        ExtensionInfo {
            id: response.id,
            external_id: response.external_id,
            name: response.name,
            authors: response.authors,
            description: response.description,
            repository: response.repository,
            downloads: response.downloads,
            created_at: response.created_at,
            updated_at: response.updated_at,
            latest_version: response.latest_version,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionVersionInfoResponse {
    pub id: String,
    pub external_id: String,
    pub name: String,
    pub authors: Vec<String>,
    pub description: String,
    pub repository: String,
    pub downloads: u64,
    pub created_at: String,
    pub updated_at: String,
    pub version: String,
    pub min_app_version: String,
    pub published_at: String,
}

impl From<ExtensionVersionInfoResponse> for ExtensionVersionInfo {
    fn from(response: ExtensionVersionInfoResponse) -> Self {
        Self {
            id: response.id,
            external_id: response.external_id,
            name: response.name,
            authors: response.authors,
            description: response.description,
            repository: response.repository,
            downloads: response.downloads,
            created_at: response.created_at,
            updated_at: response.updated_at,
            version: response.version,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ListExtensionsResponse {
    pub extensions: Vec<ExtensionInfoResponse>,
}
