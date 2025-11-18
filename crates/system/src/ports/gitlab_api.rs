use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use moss_git::url::GitUrl;
use sapic_core::context::AnyAsyncContext;
use serde::{Deserialize, Deserializer, Serialize};

use crate::{
    ports::{GitAuthAdapter, server_api::types::GitLabPkceTokenExchangeResponse},
    user::account::session::AccountSession,
};

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

// #[derive(Debug, Deserialize)]
// pub struct GitLabPkceTokenExchangeResponse {
//     pub access_token: String,
//     pub refresh_token: String,
//     pub expires_in: u64,
// }

#[derive(Debug, Deserialize, Clone)]
pub struct GitLabPkceTokenCredentials {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
}

impl From<GitLabPkceTokenExchangeResponse> for GitLabPkceTokenCredentials {
    fn from(response: GitLabPkceTokenExchangeResponse) -> Self {
        Self {
            access_token: response.access_token,
            refresh_token: response.refresh_token,
            expires_in: response.expires_in,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetRepositoryResponse {
    pub updated_at: String,
    pub owner: Owner,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Owner {
    pub username: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GetContributorsResponse {
    pub items: Vec<ContributorItem>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContributorItem {
    pub name: String,
    pub email: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetUserResponse {
    pub username: String,
    pub commit_email: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PATExpiresAtResponse {
    /// GitLab's PAT expires_at response is date only, e.g. 2025-11-19
    /// We need to process it into DateTime<Utc>

    #[serde(deserialize_with = "deserialize_expires_at_response")]
    pub expires_at: DateTime<Utc>,
}

fn deserialize_expires_at_response<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    let date = NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(serde::de::Error::custom)?;
    let naive_dt = date.and_hms_opt(0, 0, 0).ok_or(serde::de::Error::custom(
        "failed to convert NaiveDate to NaiveDateTime",
    ))?;
    Ok(Utc.from_utc_datetime(&naive_dt))
}

pub trait GitLabAuthAdapter:
    GitAuthAdapter<PkceToken = GitLabPkceTokenCredentials> + Send + Sync
{
}

#[async_trait]
pub trait GitLabApiClient: Send + Sync {
    async fn get_user(
        &self,
        ctx: &dyn AnyAsyncContext,
        account_handle: &AccountSession,
    ) -> joinerror::Result<GetUserResponse>;

    async fn get_contributors(
        &self,
        ctx: &dyn AnyAsyncContext,
        account_handle: &AccountSession,
        url: &GitUrl,
    ) -> joinerror::Result<GetContributorsResponse>;

    async fn get_repository(
        &self,
        ctx: &dyn AnyAsyncContext,
        account_handle: &AccountSession,
        url: &GitUrl,
    ) -> joinerror::Result<GetRepositoryResponse>;

    async fn get_pat_expires_at(
        &self,
        ctx: &dyn AnyAsyncContext,
        account_handle: &AccountSession,
    ) -> joinerror::Result<Option<DateTime<Utc>>>;
}
