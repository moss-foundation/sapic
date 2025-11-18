use moss_keyring::KeyringClient;
use sapic_base::user::types::primitives::{AccountId, SessionKind};
use sapic_core::context::AnyAsyncContext;
use std::sync::Arc;

use crate::{
    ports::server_api::{RevokeApiReq, auth_gitlab_account_api::GitLabTokenRefreshApiReq},
    user::account::{
        github_session::{GitHubInitialToken, GitHubPAT, GitHubSessionHandle},
        gitlab_session::{GitLabInitialToken, GitLabPAT, GitLabSessionHandle},
    },
};

enum Session {
    GitHub(GitHubSessionHandle),
    GitLab(GitLabSessionHandle),
}

pub struct AccountSession {
    keyring: Arc<dyn KeyringClient>,
    inner: Arc<Session>,
}

impl Clone for AccountSession {
    fn clone(&self) -> Self {
        Self {
            keyring: self.keyring.clone(),
            inner: self.inner.clone(),
        }
    }
}

impl AccountSession {
    pub async fn github_oauth(
        id: AccountId,
        host: String,
        initial_token: Option<GitHubInitialToken>,
        keyring: Arc<dyn KeyringClient>,
    ) -> joinerror::Result<Self> {
        let session = GitHubSessionHandle::oauth(id, host, initial_token, &keyring).await?;

        Ok(Self {
            keyring,
            inner: Arc::new(Session::GitHub(session)),
        })
    }

    pub async fn github_pat(
        id: AccountId,
        host: String,
        pat: Option<GitHubPAT>,
        keyring: Arc<dyn KeyringClient>,
    ) -> joinerror::Result<Self> {
        let session = GitHubSessionHandle::pat(id, host, pat, &keyring).await?;

        Ok(Self {
            keyring,
            inner: Arc::new(Session::GitHub(session)),
        })
    }

    pub async fn gitlab_oauth(
        id: AccountId,
        host: String,
        auth_api_client: Arc<dyn GitLabTokenRefreshApiReq>,
        initial_token: Option<GitLabInitialToken>,
        keyring: Arc<dyn KeyringClient>,
    ) -> joinerror::Result<Self> {
        let session =
            GitLabSessionHandle::oauth(id, host, auth_api_client, initial_token, &keyring).await?;

        Ok(Self {
            // secrets,
            keyring,
            inner: Arc::new(Session::GitLab(session)),
        })
    }

    pub async fn gitlab_pat(
        id: AccountId,
        host: String,
        pat: Option<GitLabPAT>,
        keyring: Arc<dyn KeyringClient>,
    ) -> joinerror::Result<Self> {
        let session = GitLabSessionHandle::pat(id, host, pat, &keyring).await?;

        Ok(Self {
            keyring,
            inner: Arc::new(Session::GitLab(session)),
        })
    }

    pub fn host(&self) -> String {
        match self.inner.as_ref() {
            Session::GitHub(handle) => handle.host(),
            Session::GitLab(handle) => handle.host(),
        }
    }

    pub async fn token(&self, ctx: &dyn AnyAsyncContext) -> joinerror::Result<String> {
        match self.inner.as_ref() {
            Session::GitHub(handle) => handle.token(&self.keyring).await,
            Session::GitLab(handle) => handle.token(ctx, &self.keyring).await,
        }
    }

    pub fn session_kind(&self) -> SessionKind {
        match self.inner.as_ref() {
            Session::GitHub(handle) => handle.session_kind(),
            Session::GitLab(handle) => handle.session_kind(),
        }
    }

    pub async fn revoke(
        &self,
        ctx: &dyn AnyAsyncContext,
        api_client: Arc<dyn RevokeApiReq>,
    ) -> joinerror::Result<()> {
        match self.inner.as_ref() {
            Session::GitHub(handle) => handle.revoke(ctx, &self.keyring, api_client).await,
            Session::GitLab(handle) => handle.revoke(ctx, &self.keyring, api_client).await,
        }
    }

    pub async fn update_pat(&self, pat: &str) -> joinerror::Result<()> {
        match self.inner.as_ref() {
            Session::GitHub(handle) => handle.update_pat(&self.keyring, pat).await,
            Session::GitLab(handle) => handle.update_pat(&self.keyring, pat).await,
        }
    }
}
