use async_trait::async_trait;
use joinerror::ResultExt;
use reqwest::Client as HttpClient;
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

#[async_trait]
pub trait GitLabPkceTokenExchangeApiReq: Send + Sync {
    async fn gitlab_pkce_token_exchange(
        &self,
        request: TokenExchangeRequest,
    ) -> joinerror::Result<GitLabPkceTokenExchangeResponse>;
}

#[async_trait]
pub trait GitLabTokenRefreshApiReq: Send + Sync {
    async fn gitlab_token_refresh(
        &self,
        request: GitLabTokenRefreshRequest,
    ) -> joinerror::Result<GitLabTokenRefreshResponse>;
}

#[async_trait]
pub trait GitHubPkceTokenExchangeApiReq: Send + Sync {
    async fn github_pkce_token_exchange(
        &self,
        request: TokenExchangeRequest,
    ) -> joinerror::Result<GitHubPkceTokenExchangeResponse>;
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
impl GitLabPkceTokenExchangeApiReq for AccountAuthGatewayApiClient {
    async fn gitlab_pkce_token_exchange(
        &self,
        request: TokenExchangeRequest,
    ) -> joinerror::Result<GitLabPkceTokenExchangeResponse> {
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
    }
}

#[async_trait]
impl GitLabTokenRefreshApiReq for AccountAuthGatewayApiClient {
    async fn gitlab_token_refresh(
        &self,
        request: GitLabTokenRefreshRequest,
    ) -> joinerror::Result<GitLabTokenRefreshResponse> {
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
    }
}

#[async_trait]
impl GitHubPkceTokenExchangeApiReq for AccountAuthGatewayApiClient {
    async fn github_pkce_token_exchange(
        &self,
        request: TokenExchangeRequest,
    ) -> joinerror::Result<GitHubPkceTokenExchangeResponse> {
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
    }
}
