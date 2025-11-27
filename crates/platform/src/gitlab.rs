pub mod auth;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use moss_git::url::GitUrl;
use oauth2::http::header::{ACCEPT, AUTHORIZATION};
use reqwest::{Client as HttpClient, RequestBuilder};
use sapic_core::context::{self, AnyAsyncContext, ContextResultExt};
use sapic_system::{ports::gitlab_api::*, user::account::session::AccountSession};

fn api_url(host: &str) -> String {
    format!("https://{host}/api/v4") // TODO: make version configurable?
}

const CONTENT_TYPE: &'static str = "application/json";

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
impl GitLabApiClient for AppGitLabApiClient {
    async fn get_user(
        &self,
        ctx: &dyn AnyAsyncContext,
        account_handle: &AccountSession,
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
        ctx: &dyn AnyAsyncContext,
        account_handle: &AccountSession,
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
        ctx: &dyn AnyAsyncContext,
        account_handle: &AccountSession,
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
        ctx: &dyn AnyAsyncContext,
        account_handle: &AccountSession,
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
