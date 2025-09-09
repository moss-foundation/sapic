use std::sync::Arc;

use async_trait::async_trait;
use moss_app_delegate::AppDelegate;
use moss_applib::{
    AppRuntime,
    context::{self, ContextResultExt},
};
use moss_git::url::GitUrl;
use moss_user::AccountSession;
use oauth2::http::header::{ACCEPT, AUTHORIZATION, USER_AGENT};
use reqwest::{Client as HttpClient, RequestBuilder};

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
pub struct RealGitHubApiClient {
    client: HttpClient,
}

impl RealGitHubApiClient {
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }
}

#[async_trait]
impl<R: AppRuntime> GitHubApiClient<R> for RealGitHubApiClient {
    async fn get_user(
        &self,
        ctx: &R::AsyncContext,
        account_handle: &AccountSession<R>,
    ) -> joinerror::Result<GetUserResponse> {
        context::abortable(ctx, async {
            let access_token = account_handle.access_token(ctx).await?;
            let resp = self
                .client
                .get(format!("{GITHUB_API_URL}/user"))
                .with_default_github_headers(access_token)
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
            let access_token = account_handle.access_token(ctx).await?;
            let repo_url = format!("{}/{}", &url.owner, &url.name);
            let resp = self
                .client
                .get(format!("{GITHUB_API_URL}/repos/{repo_url}/contributors"))
                .with_default_github_headers(access_token)
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
            let access_token = account_handle.access_token(ctx).await?;
            let repo_url = format!("{}/{}", &url.owner, &url.name);
            let resp = self
                .client
                .get(format!("{GITHUB_API_URL}/repos/{repo_url}"))
                .with_default_github_headers(access_token)
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
}

#[cfg(any(test, feature = "test"))]
pub mod test {
    use super::*;

    pub struct MockGitHubApiClient {
        pub get_user_response: GetUserResponse,
        pub get_contributors_response: GetContributorsResponse,
        pub get_repository_response: GetRepositoryResponse,
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
    }
}
