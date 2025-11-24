use async_trait::async_trait;
use joinerror::ResultExt;
use moss_applib::AppRuntime;
use reqwest::Client as HttpClient;
use sapic_core::context::{self, ContextResultExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

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

#[async_trait]
pub trait GitLabPkceTokenExchangeApiReq<R: AppRuntime>: Send + Sync {
    async fn gitlab_pkce_token_exchange(
        &self,
        ctx: &R::AsyncContext,
        request: TokenExchangeRequest,
    ) -> joinerror::Result<GitLabPkceTokenExchangeResponse>;
}

#[async_trait]
pub trait GitLabTokenRefreshApiReq<R: AppRuntime>: Send + Sync {
    async fn gitlab_token_refresh(
        &self,
        ctx: &R::AsyncContext,
        request: GitLabTokenRefreshRequest,
    ) -> joinerror::Result<GitLabTokenRefreshResponse>;
}

#[async_trait]
pub trait GitHubPkceTokenExchangeApiReq<R: AppRuntime>: Send + Sync {
    async fn github_pkce_token_exchange(
        &self,
        ctx: &R::AsyncContext,
        request: TokenExchangeRequest,
    ) -> joinerror::Result<GitHubPkceTokenExchangeResponse>;
}

#[async_trait]
pub trait GitHubRevokeApiReq<R: AppRuntime>: Send + Sync {
    async fn github_revoke(
        &self,
        ctx: &R::AsyncContext,
        request: GitHubRevokeRequest,
    ) -> joinerror::Result<()>;
}

#[async_trait]
pub trait GitLabRevokeApiReq<R: AppRuntime>: Send + Sync {
    async fn gitlab_revoke(
        &self,
        ctx: &R::AsyncContext,
        request: GitLabRevokeRequest,
    ) -> joinerror::Result<()>;
}

#[async_trait]
pub trait RevokeApiReq<R: AppRuntime>:
    Send + Sync + GitHubRevokeApiReq<R> + GitLabRevokeApiReq<R>
{
}

#[derive(Clone)]
pub struct AccountAuthGatewayApiClient {
    base_url: Arc<String>,
    client: HttpClient,
}

impl AccountAuthGatewayApiClient {
    pub fn new(client: HttpClient, base_url: String) -> Self {
        Self {
            base_url: base_url.into(),
            client,
        }
    }

    pub fn base_url(&self) -> Arc<String> {
        self.base_url.clone()
    }
}

#[async_trait]
impl<R: AppRuntime> GitLabPkceTokenExchangeApiReq<R> for AccountAuthGatewayApiClient {
    async fn gitlab_pkce_token_exchange(
        &self,
        ctx: &R::AsyncContext,
        request: TokenExchangeRequest,
    ) -> joinerror::Result<GitLabPkceTokenExchangeResponse> {
        context::abortable(ctx, async {
            let resp = self
                .client
                .post(format!("{}/auth/gitlab/token", self.base_url))
                .json(&request)
                .send()
                .await
                .join_err::<()>("failed to exchange GitLab PKCE token")?;

            if !resp.status().is_success() {
                let error_text = resp.text().await?;
                return Err(joinerror::Error::new::<()>(error_text));
            }

            resp.json()
                .await
                .join_err::<()>("failed to parse GitLab PKCE token exchange response")
        })
        .await
        .join_err_bare()
    }
}

#[async_trait]
impl<R: AppRuntime> GitLabTokenRefreshApiReq<R> for AccountAuthGatewayApiClient {
    async fn gitlab_token_refresh(
        &self,
        ctx: &R::AsyncContext,
        request: GitLabTokenRefreshRequest,
    ) -> joinerror::Result<GitLabTokenRefreshResponse> {
        context::abortable(ctx, async {
            let resp = self
                .client
                .post(format!("{}/auth/gitlab/refresh", self.base_url))
                .json(&request)
                .send()
                .await
                .join_err::<()>("failed to refresh GitLab token")?;

            if !resp.status().is_success() {
                let error_text = resp.text().await?;
                return Err(joinerror::Error::new::<()>(error_text));
            }

            resp.json()
                .await
                .join_err::<()>("failed to parse GitLab token refresh response")
        })
        .await
        .join_err_bare()
    }
}

#[async_trait]
impl<R: AppRuntime> GitHubPkceTokenExchangeApiReq<R> for AccountAuthGatewayApiClient {
    async fn github_pkce_token_exchange(
        &self,
        ctx: &R::AsyncContext,
        request: TokenExchangeRequest,
    ) -> joinerror::Result<GitHubPkceTokenExchangeResponse> {
        context::abortable(ctx, async {
            let resp = self
                .client
                .post(format!("{}/auth/github/token", self.base_url))
                .json(&request)
                .send()
                .await
                .join_err::<()>("failed to exchange GitHub PKCE token")?;

            if !resp.status().is_success() {
                let error_text = resp.text().await?;
                return Err(joinerror::Error::new::<()>(error_text));
            }

            resp.json()
                .await
                .join_err::<()>("failed to parse GitHub PKCE token exchange response")
        })
        .await
        .join_err_bare()
    }
}

#[async_trait]
impl<R: AppRuntime> GitHubRevokeApiReq<R> for AccountAuthGatewayApiClient {
    async fn github_revoke(
        &self,
        ctx: &R::AsyncContext,
        request: GitHubRevokeRequest,
    ) -> joinerror::Result<()> {
        context::abortable(ctx, async {
            let resp = self
                .client
                .post(format!("{}/auth/github/revoke", self.base_url))
                .json(&request)
                .send()
                .await
                .join_err::<()>("failed to revoke GitHub token")?;

            if !resp.status().is_success() {
                let error_text = resp.text().await?;
                return Err(joinerror::Error::new::<()>(error_text));
            }

            Ok(())
        })
        .await
        .join_err_bare()
    }
}

#[async_trait]
impl<R: AppRuntime> GitLabRevokeApiReq<R> for AccountAuthGatewayApiClient {
    async fn gitlab_revoke(
        &self,
        ctx: &R::AsyncContext,
        request: GitLabRevokeRequest,
    ) -> joinerror::Result<()> {
        context::abortable(ctx, async {
            let resp = self
                .client
                .post(format!("{}/auth/gitlab/revoke", self.base_url))
                .json(&request)
                .send()
                .await
                .join_err::<()>("failed to revoke GitHub token")?;

            if !resp.status().is_success() {
                let error_text = resp.text().await?;
                return Err(joinerror::Error::new::<()>(error_text));
            }

            Ok(())
        })
        .await
        .join_err_bare()
    }
}

#[async_trait]
impl<R: AppRuntime> RevokeApiReq<R> for AccountAuthGatewayApiClient {}
