use async_trait::async_trait;
use joinerror::ResultExt;
use sapic_core::context::{self, AnyAsyncContext, ContextResultExt};
use sapic_system::ports::server_api::{
    auth_github_account_api::*,
    types::{GitHubPkceTokenExchangeResponse, GitHubRevokeRequest, TokenExchangeRequest},
};

use super::HttpServerApiClient;

#[async_trait]
impl GitHubPkceTokenExchangeApiReq for HttpServerApiClient {
    async fn github_pkce_token_exchange(
        &self,
        ctx: &dyn AnyAsyncContext,
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
impl GitHubRevokeApiReq for HttpServerApiClient {
    async fn github_revoke(
        &self,
        ctx: &dyn AnyAsyncContext,
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
impl AuthGitHubAccountApiOperations for HttpServerApiClient {}
