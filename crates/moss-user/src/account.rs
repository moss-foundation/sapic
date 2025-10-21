mod common;
pub mod github;
pub mod gitlab;

use chrono::{DateTime, Utc};
use moss_applib::AppRuntime;
use moss_keyring::KeyringClient;
use moss_server_api::account_auth_gateway::{GitLabTokenRefreshApiReq, RevokeApiReq};
use std::sync::Arc;

use crate::{
    account::{
        github::{GitHubInitialToken, GitHubPAT, GitHubSessionHandle},
        gitlab::{GitLabInitialToken, GitLabPAT, GitLabSessionHandle},
    },
    models::{
        primitives::{AccountId, AccountKind, SessionKind},
        types::{AccountInfo, AccountMetadata},
    },
};

#[derive(Clone)]
pub(crate) struct Metadata {
    pub(crate) expires_at: Option<DateTime<Utc>>,
}

pub struct Account<R: AppRuntime> {
    pub(crate) id: AccountId,
    pub(crate) username: String,
    pub(crate) host: String,
    pub(crate) session: AccountSession<R>,
    pub(crate) kind: AccountKind,
    pub(crate) metadata: Metadata,
}

impl<R: AppRuntime> Clone for Account<R> {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            username: self.username.clone(),
            host: self.host.clone(),
            session: self.session.clone(),
            kind: self.kind.clone(),
            metadata: self.metadata.clone(),
        }
    }
}

impl<R: AppRuntime> Account<R> {
    pub fn new(
        id: AccountId,
        username: String,
        host: String,
        session: AccountSession<R>,
        kind: AccountKind,
        expires_at: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            id,
            username,
            host,
            session,
            kind,
            metadata: Metadata { expires_at },
        }
    }

    pub fn id(&self) -> AccountId {
        self.id.clone()
    }

    pub fn session(&self) -> &AccountSession<R> {
        &self.session
    }

    pub fn username(&self) -> String {
        self.username.clone()
    }

    pub fn host(&self) -> String {
        self.host.clone()
    }

    pub fn info(&self) -> AccountInfo {
        AccountInfo {
            id: self.id.clone(),
            username: self.username.clone(),
            host: self.host.clone(),
            kind: self.kind.clone(),
            method: self.session.session_kind().into(),
            metadata: AccountMetadata {
                pat_expires_at: self.metadata.expires_at,
            },
        }
    }

    pub async fn revoke(
        &self,
        ctx: &R::AsyncContext,
        api_client: Arc<dyn RevokeApiReq<R>>,
    ) -> joinerror::Result<()> {
        self.session.revoke(ctx, api_client).await
    }

    // Update PAT and returns the old PAT
    // If the new PAT belongs to a different account or does not exist, revert the change
    pub async fn update_pat(&self, ctx: &R::AsyncContext, pat: &str) -> joinerror::Result<String> {
        let old_pat = self.session.token(ctx).await?;
        self.session.update_pat(pat).await?;
        Ok(old_pat)
    }
}

enum Session<R: AppRuntime> {
    GitHub(GitHubSessionHandle),
    GitLab(GitLabSessionHandle<R>),
}

pub struct AccountSession<R: AppRuntime> {
    keyring: Arc<dyn KeyringClient>,
    inner: Arc<Session<R>>,
}

impl<R: AppRuntime> Clone for AccountSession<R> {
    fn clone(&self) -> Self {
        Self {
            keyring: self.keyring.clone(),
            inner: self.inner.clone(),
        }
    }
}

impl<R: AppRuntime> AccountSession<R> {
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
        auth_api_client: Arc<dyn GitLabTokenRefreshApiReq<R>>,
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

    pub async fn token(&self, ctx: &R::AsyncContext) -> joinerror::Result<String> {
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
        ctx: &R::AsyncContext,
        api_client: Arc<dyn RevokeApiReq<R>>,
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
