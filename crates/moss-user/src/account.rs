mod common;
pub mod github;
pub mod gitlab;

use moss_applib::AppRuntime;
use moss_keyring::KeyringClient;
use moss_server_api::account_auth_gateway::GitLabTokenRefreshApiReq;
use std::sync::Arc;

use crate::{
    account::{
        github::{GitHubInitialToken, GitHubSessionHandle},
        gitlab::{GitLabInitialToken, GitLabSessionHandle},
    },
    models::{
        primitives::{AccountId, AccountKind},
        types::AccountInfo,
    },
};

pub struct Account<R: AppRuntime> {
    pub(crate) id: AccountId,
    pub(crate) username: String,
    pub(crate) host: String,
    pub(crate) session: AccountSession<R>,
    pub(crate) kind: AccountKind,
}

impl<R: AppRuntime> Clone for Account<R> {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            username: self.username.clone(),
            host: self.host.clone(),
            session: self.session.clone(),
            kind: self.kind.clone(),
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
    ) -> Self {
        Self {
            id,
            username,
            host,
            session,
            kind,
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
        }
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
    pub async fn github(
        id: AccountId,
        host: String,
        keyring: Arc<dyn KeyringClient>,

        initial_token: Option<GitHubInitialToken>,
    ) -> joinerror::Result<Self> {
        let session = GitHubSessionHandle::new(id, host, initial_token, &keyring).await?;

        Ok(Self {
            keyring,
            inner: Arc::new(Session::GitHub(session)),
        })
    }

    pub async fn gitlab(
        id: AccountId,
        host: String,
        keyring: Arc<dyn KeyringClient>,
        auth_api_client: Arc<dyn GitLabTokenRefreshApiReq<R>>,
        initial_token: Option<GitLabInitialToken>,
    ) -> joinerror::Result<Self> {
        let session =
            GitLabSessionHandle::new(id, host, auth_api_client, initial_token, &keyring).await?;

        Ok(Self {
            // secrets,
            keyring,
            inner: Arc::new(Session::GitLab(session)),
        })
    }

    pub fn host(&self) -> String {
        match self.inner.as_ref() {
            Session::GitHub(handle) => handle.host.clone(),
            Session::GitLab(handle) => handle.host.clone(),
        }
    }

    pub async fn access_token(&self, ctx: &R::AsyncContext) -> joinerror::Result<String> {
        match self.inner.as_ref() {
            Session::GitHub(handle) => handle.access_token(&self.keyring).await,
            Session::GitLab(handle) => handle.access_token(ctx, &self.keyring).await,
        }
    }
}
