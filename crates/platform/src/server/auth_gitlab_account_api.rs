use async_trait::async_trait;
use joinerror::ResultExt;
use sapic_core::context::{self, AnyAsyncContext, ContextResultExt};
use sapic_system::ports::server_api::{
    auth_gitlab_account_api::*,
    types::{
        GitLabPkceTokenExchangeResponse, GitLabRevokeRequest, GitLabTokenRefreshRequest,
        GitLabTokenRefreshResponse, TokenExchangeRequest,
    },
};

use super::HttpServerApiClient;

const BASE_SEGMENT: &str = "account-auth-gateway";

#[async_trait]
impl GitLabTokenRefreshApiReq for HttpServerApiClient {
    async fn gitlab_token_refresh(
        &self,
        ctx: &dyn AnyAsyncContext,
        request: GitLabTokenRefreshRequest,
    ) -> joinerror::Result<GitLabTokenRefreshResponse> {
        context::abortable(ctx, async {
            let resp = self
                .client
                .post(format!(
                    "{}/{BASE_SEGMENT}/auth/gitlab/refresh",
                    self.base_url
                ))
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
impl GitLabRevokeApiReq for HttpServerApiClient {
    async fn gitlab_revoke(
        &self,
        ctx: &dyn AnyAsyncContext,
        request: GitLabRevokeRequest,
    ) -> joinerror::Result<()> {
        context::abortable(ctx, async {
            let resp = self
                .client
                .post(format!(
                    "{}/{BASE_SEGMENT}/auth/gitlab/revoke",
                    self.base_url
                ))
                .json(&request)
                .send()
                .await
                .join_err::<()>("failed to revoke GitLab token")?;

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
impl GitLabPkceTokenExchangeApiReq for HttpServerApiClient {
    async fn gitlab_pkce_token_exchange(
        &self,
        ctx: &dyn AnyAsyncContext,
        request: TokenExchangeRequest,
    ) -> joinerror::Result<GitLabPkceTokenExchangeResponse> {
        context::abortable(ctx, async {
            let resp = self
                .client
                .post(format!(
                    "{}/{BASE_SEGMENT}/auth/gitlab/token",
                    self.base_url
                ))
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
impl AuthGitLabAccountApiOperations for HttpServerApiClient {}
