use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;
use moss_git::url::GitUrl;
use moss_user::AccountSession;
use oauth2::http::header::{ACCEPT, AUTHORIZATION};
use reqwest::{Client as HttpClient, RequestBuilder};
use sapic_core::context::{self, ContextResultExt};

use crate::gitlab::response::{
    GetContributorsResponse, GetRepositoryResponse, GetUserResponse, PATExpiresAtResponse,
};

fn api_url(host: &str) -> String {
    format!("https://{host}/api/v4") // TODO: make version configurable?
}

const CONTENT_TYPE: &'static str = "application/json";

#[async_trait]
pub trait GitLabApiClient<R: AppRuntime>: Send + Sync {
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

struct GlobalGitLabApiClient<R: AppRuntime>(Arc<dyn GitLabApiClient<R>>);

impl<R: AppRuntime> dyn GitLabApiClient<R> {
    pub fn global(delegate: &AppDelegate<R>) -> Arc<dyn GitLabApiClient<R>> {
        delegate.global::<GlobalGitLabApiClient<R>>().0.clone()
    }

    pub fn set_global(delegate: &AppDelegate<R>, v: Arc<dyn GitLabApiClient<R>>) {
        delegate.set_global(GlobalGitLabApiClient(v));
    }
}

trait GitLabHttpRequestBuilderExt {
    fn with_default_gitlab_headers(self, access_token: String) -> Self;
}

impl GitLabHttpRequestBuilderExt for RequestBuilder {
    fn with_default_gitlab_headers(self, access_token: String) -> Self {
        self.header(ACCEPT, CONTENT_TYPE)
            .header(AUTHORIZATION, format!("Bearer {}", access_token))
    }
}

#[derive(Clone)]
pub struct AppGitLabApiClient {
    client: HttpClient,
}

impl AppGitLabApiClient {
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }
}

#[async_trait]
impl<R: AppRuntime> GitLabApiClient<R> for AppGitLabApiClient {
    async fn get_user(
        &self,
        ctx: &R::AsyncContext,
        account_handle: &AccountSession<R>,
    ) -> joinerror::Result<GetUserResponse> {
        context::abortable(ctx, async {
            let token = account_handle.token(ctx).await?;
            let resp = self
                .client
                .get(format!("{}/user", api_url(&account_handle.host())))
                .with_default_gitlab_headers(token)
                .send()
                .await?;

            let status = resp.status();
            if status.is_success() {
                Ok(resp.json().await?)
            } else {
                let error_text = resp.text().await?;
                eprintln!("GitLab API Error: Status {}, Body: {}", status, error_text);
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
            let encoded_url = urlencoding::encode(&repo_url);

            let resp = self
                .client
                .get(format!(
                    "{}/projects/{}/repository/contributors",
                    api_url(&account_handle.host()),
                    encoded_url
                ))
                .with_default_gitlab_headers(token)
                .send()
                .await?;

            let status = resp.status();
            if status.is_success() {
                Ok(resp.json().await?)
            } else {
                let error_text = resp.text().await?;
                eprintln!("GitLab API Error: Status {}, Body: {}", status, error_text);
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
            let encoded_url = urlencoding::encode(&repo_url);

            let resp = self
                .client
                .get(format!(
                    "{}/projects/{}/repository/contributors",
                    api_url(&account_handle.host()),
                    encoded_url
                ))
                .with_default_gitlab_headers(token)
                .send()
                .await?;

            let status = resp.status();
            if status.is_success() {
                Ok(resp.json().await?)
            } else {
                let error_text = resp.text().await?;
                eprintln!("GitLab API Error: Status {}, Body: {}", status, error_text);
                Err(joinerror::Error::new::<()>(error_text))
            }
        })
        .await
        .join_err_bare()
    }

    async fn get_pat_expires_at(
        &self,
        ctx: &R::AsyncContext,
        account_handle: &AccountSession<R>,
    ) -> joinerror::Result<Option<DateTime<Utc>>> {
        context::abortable(ctx, async {
            let token = account_handle.token(ctx).await?;
            let resp = self
                .client
                .get(format!(
                    "{}/personal_access_tokens/self",
                    api_url(&account_handle.host())
                ))
                .with_default_gitlab_headers(token)
                .send()
                .await?;

            let status = resp.status();
            if status.is_success() {
                let pat_resp: PATExpiresAtResponse = resp.json().await?;
                Ok(Some(pat_resp.expires_at))
            } else {
                let error_text = resp.text().await?;
                eprintln!("GitLab API Error: Status {}, Body: {}", status, error_text);
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

    pub struct MockGitLabApiClient {
        pub get_user_response: GetUserResponse,
        pub get_contributors_response: GetContributorsResponse,
        pub get_repository_response: GetRepositoryResponse,
        pub get_pat_expires_at_response: Option<DateTime<Utc>>,
    }

    #[async_trait]
    impl<R: AppRuntime> GitLabApiClient<R> for MockGitLabApiClient {
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
