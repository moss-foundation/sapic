use async_trait::async_trait;
use chrono::{DateTime, Utc};
use moss_git::url::GitUrl;
use sapic_core::context::AnyAsyncContext;
use serde::Deserialize;

use crate::{
    ports::{GitAuthAdapter, server_api::types::GitHubPkceTokenExchangeResponse},
    user::account::session::AccountSession,
};

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Owner {
    pub login: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct GetRepositoryResponse {
    pub owner: Owner,
    pub updated_at: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(transparent)]
pub struct GetContributorsResponse {
    pub items: Vec<Contributor>,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Contributor {
    pub login: String,
    pub avatar_url: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct GetUserResponse {
    pub id: u64,
    pub login: String,
    pub email: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GitHubPkceTokenCredentials {
    pub access_token: String,
}

impl From<GitHubPkceTokenExchangeResponse> for GitHubPkceTokenCredentials {
    fn from(response: GitHubPkceTokenExchangeResponse) -> Self {
        Self {
            access_token: response.access_token,
        }
    }
}

pub trait GitHubAuthAdapter:
    GitAuthAdapter<PkceToken = GitHubPkceTokenCredentials> + Send + Sync
{
}

#[async_trait]
pub trait GitHubApiClient: Send + Sync {
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
