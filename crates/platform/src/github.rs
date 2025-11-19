pub mod auth;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use joinerror::Error;
use moss_git::url::GitUrl;
use oauth2::http::header::{ACCEPT, AUTHORIZATION, USER_AGENT};
use reqwest::{Client as HttpClient, RequestBuilder};
use sapic_core::context::{self, AnyAsyncContext, ContextResultExt};
use sapic_system::{
    ports::github_api::{
        GetContributorsResponse, GetRepositoryResponse, GetUserResponse, GitHubApiClient,
    },
    user::account::session::AccountSession,
};

const GITHUB_API_URL: &'static str = "https://api.github.com";

trait GitHubHttpRequestBuilderExt {
    fn with_default_github_headers(self, access_token: String) -> Self;
}

impl GitHubHttpRequestBuilderExt for RequestBuilder {
    fn with_default_github_headers(self, access_token: String) -> Self {
        self.header(ACCEPT, "application/vnd.github+json")
            .header(USER_AGENT, "SAPIC/1.0")
            .header(AUTHORIZATION, format!("token {}", access_token))
    }
}

#[derive(Clone)]
pub struct AppGitHubApiClient {
    client: HttpClient,
}

impl AppGitHubApiClient {
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }
}

#[async_trait]
impl GitHubApiClient for AppGitHubApiClient {
    async fn get_user(
        &self,
        ctx: &dyn AnyAsyncContext,
        account_handle: &AccountSession,
    ) -> joinerror::Result<GetUserResponse> {
        context::abortable(ctx, async {
            let token = account_handle.token(ctx).await?;
            let resp = self
                .client
                .get(format!("{GITHUB_API_URL}/user"))
                .with_default_github_headers(token)
                .send()
                .await?;

            let status = resp.status();
            if status.is_success() {
                Ok(resp.json().await?)
            } else {
                let error_text = resp.text().await?;
                eprintln!("GitHub API Error: Status {}, Body: {}", status, error_text);
                Err(joinerror::Error::new::<()>(error_text))
            }
        })
        .await
        .join_err_bare()
    }

    async fn get_contributors(
        &self,
        ctx: &dyn AnyAsyncContext,
        account_handle: &AccountSession,
        url: &GitUrl,
    ) -> joinerror::Result<GetContributorsResponse> {
        context::abortable(ctx, async {
            let token = account_handle.token(ctx).await?;
            let repo_url = format!("{}/{}", &url.owner, &url.name);
            let resp = self
                .client
                .get(format!("{GITHUB_API_URL}/repos/{repo_url}/contributors"))
                .with_default_github_headers(token)
                .send()
                .await?;

            let status = resp.status();
            if status.is_success() {
                Ok(resp.json().await?)
            } else {
                let error_text = resp.text().await?;
                eprintln!("GitHub API Error: Status {}, Body: {}", status, error_text);
                Err(joinerror::Error::new::<()>(error_text))
            }
        })
        .await
        .join_err_bare()
    }

    async fn get_repository(
        &self,
        ctx: &dyn AnyAsyncContext,
        account_handle: &AccountSession,
        url: &GitUrl,
    ) -> joinerror::Result<GetRepositoryResponse> {
        context::abortable(ctx, async {
            let token = account_handle.token(ctx).await?;
            let repo_url = format!("{}/{}", &url.owner, &url.name);
            let resp = self
                .client
                .get(format!("{GITHUB_API_URL}/repos/{repo_url}"))
                .with_default_github_headers(token)
                .send()
                .await?;

            let status = resp.status();
            if status.is_success() {
                Ok(resp.json().await?)
            } else {
                let error_text = resp.text().await?;
                eprintln!("GitHub API Error: Status {}, Body: {}", status, error_text);
                Err(joinerror::Error::new::<()>(error_text))
            }
        })
        .await
        .join_err_bare()
    }

    /// GitHub does not seem to offer a dedicated endpoint for checking the expiry date of a PAT
    /// However, the API responses contain the header `github-authentication-token-expiration`
    /// To make things consistent across providers, we still create a separate method for it
    async fn get_pat_expires_at(
        &self,
        ctx: &dyn AnyAsyncContext,
        account_handle: &AccountSession,
    ) -> joinerror::Result<Option<DateTime<Utc>>> {
        context::abortable(ctx, async {
            let token = account_handle.token(ctx).await?;
            let resp = self
                .client
                .get(format!("{GITHUB_API_URL}/user"))
                .with_default_github_headers(token)
                .send()
                .await?;

            let status = resp.status();
            if status.is_success() {
                let expires_header =
                    resp.headers().get("github-authentication-token-expiration");
                if expires_header.is_none() {
                    return Ok(None);
                }
                // Format:
                // 2025-11-19 15:50:16 UTC
                // chrono does not handle 'UTC' suffix properly
                // We need to convert it into +00:00 for proper parsing
                let expires_at_str =
                    expires_header
                        .unwrap()
                        .to_str()
                        .map_err(|err|
                            Error::new::<()>(format!("failed to convert 'github-authentication-token-expiration' header to string: {}", err))
                        )?
                        .replace("UTC", "+00:00");

                let expires_at_utc =
                    DateTime::parse_from_str(&expires_at_str, "%Y-%m-%d %H:%M:%S %:z")
                        .map_err(|err| joinerror::Error::new::<()>(err.to_string()))?
                        .with_timezone(&Utc);
                Ok(Some(expires_at_utc))

            } else {
                let error_text = resp.text().await?;
                eprintln!("GitHub API Error: Status {}, Body: {}", status, error_text);
                Err(joinerror::Error::new::<()>(error_text))
            }
        })        .await
            .join_err_bare()
    }
}
