// Placing these types in this crate isn't entirely correct.
// In theory, this crate should contain system-level types used internally by the application,
// and the current DTO types should be moved to the `platform` crate.
// But since it's not yet clear what exactly will be used at the internal system level
// of the application, I'll leave them here for now.

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct TokenExchangeRequest {
    pub code: String,
    pub state: String,
}

#[derive(Debug, Serialize)]
pub struct GitLabTokenRefreshRequest {
    pub refresh_token: String,
}

#[derive(Debug, Deserialize)]
pub struct GitLabTokenRefreshResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
}

#[derive(Debug, Deserialize)]
pub struct GitLabPkceTokenExchangeResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
}

#[derive(Debug, Deserialize)]
pub struct GitHubPkceTokenExchangeResponse {
    pub access_token: String,
}

#[derive(Debug, Serialize)]
pub struct GitHubRevokeRequest {
    pub access_token: String,
}

#[derive(Debug, Serialize)]
pub struct GitLabRevokeRequest {
    pub access_token: Option<String>,
    pub refresh_token: String,
}
