use async_trait::async_trait;
use chrono::{DateTime, Utc};
use joinerror::Error;
use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;
use moss_git::url::GitUrl;
use moss_user::AccountSession;
use oauth2::http::header::{ACCEPT, AUTHORIZATION, USER_AGENT};
use reqwest::{Client as HttpClient, RequestBuilder};
use sapic_core::context::{self, ContextResultExt};
use std::sync::Arc;

use crate::github::response::{GetContributorsResponse, GetRepositoryResponse, GetUserResponse};

const GITHUB_API_URL: &'static str = "https://api.github.com";

#[async_trait]
pub trait GitHubApiClient<R: AppRuntime>: Send + Sync {
    async fn get_user(
        &self,
        ctx: &R::AsyncContext,
        account_handle: &AccountSession<R>,
    ) -> joinerror::Result<GetUserResponse>;

    async fn get_contributors(
        &self,
        ctx: &R::AsyncContext,
        account_handle: &AccountSession<R>,
        url: &GitUrl,
    ) -> joinerror::Result<GetContributorsResponse>;

    async fn get_repository(
        &self,
        ctx: &R::AsyncContext,
        account_handle: &AccountSession<R>,
        url: &GitUrl,
    ) -> joinerror::Result<GetRepositoryResponse>;

    async fn get_pat_expires_at(
        &self,
        ctx: &R::AsyncContext,
        account_handle: &AccountSession<R>,
    ) -> joinerror::Result<Option<DateTime<Utc>>>;
}

struct GlobalGitHubApiClient<R: AppRuntime>(Arc<dyn GitHubApiClient<R>>);

impl<R: AppRuntime> dyn GitHubApiClient<R> {
    pub fn global(delegate: &AppDelegate<R>) -> Arc<dyn GitHubApiClient<R>> {
        delegate.global::<GlobalGitHubApiClient<R>>().0.clone()
    }

    pub fn set_global(delegate: &AppDelegate<R>, v: Arc<dyn GitHubApiClient<R>>) {
        delegate.set_global(GlobalGitHubApiClient(v));
    }
}

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
impl<R: AppRuntime> GitHubApiClient<R> for AppGitHubApiClient {
    async fn get_user(
        &self,
        ctx: &R::AsyncContext,
        account_handle: &AccountSession<R>,
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
        ctx: &R::AsyncContext,
        account_handle: &AccountSession<R>,
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
        ctx: &R::AsyncContext,
        account_handle: &AccountSession<R>,
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
        ctx: &R::AsyncContext,
        account_handle: &AccountSession<R>,
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

#[cfg(any(test, feature = "test"))]
pub mod test {
    use super::*;

    pub struct MockGitHubApiClient {
        pub get_user_response: GetUserResponse,
        pub get_contributors_response: GetContributorsResponse,
        pub get_repository_response: GetRepositoryResponse,
        pub get_pat_expires_at_response: Option<DateTime<Utc>>,
    }

    #[async_trait]
    impl<R: AppRuntime> GitHubApiClient<R> for MockGitHubApiClient {
        async fn get_user(
            &self,
            _ctx: &R::AsyncContext,
            _account_handle: &AccountSession<R>,
        ) -> joinerror::Result<GetUserResponse> {
            Ok(self.get_user_response.clone())
        }

        async fn get_contributors(
            &self,
            _ctx: &R::AsyncContext,
            _account_handle: &AccountSession<R>,
            _url: &GitUrl,
        ) -> joinerror::Result<GetContributorsResponse> {
            Ok(self.get_contributors_response.clone())
        }

        async fn get_repository(
            &self,
            _ctx: &R::AsyncContext,
            _account_handle: &AccountSession<R>,
            _url: &GitUrl,
        ) -> joinerror::Result<GetRepositoryResponse> {
            Ok(self.get_repository_response.clone())
        }

        async fn get_pat_expires_at(
            &self,
            _ctx: &R::AsyncContext,
            _account_handle: &AccountSession<R>,
        ) -> joinerror::Result<Option<DateTime<Utc>>> {
            Ok(self.get_pat_expires_at_response.clone())
        }
    }
}
