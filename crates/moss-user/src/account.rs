pub mod auth_gateway_api;
mod common;
pub mod github;
pub mod gitlab;

use moss_asp::AppSecretsProvider;
use moss_keyring::KeyringClient;
use oauth2::{ClientId, EmptyExtraTokenFields, StandardTokenResponse, basic::BasicTokenType};
use std::sync::Arc;

use crate::{
    account::{
        auth_gateway_api::GitLabTokenRefreshApiReq,
        github::{GitHubInitialToken, GitHubSessionHandle},
        gitlab::{GitLabInitialToken, GitLabSessionHandle},
    },
    models::primitives::AccountId,
};

#[derive(Clone)]
pub struct Account {
    id: AccountId,
    username: String,
    host: String,
    session: AccountSession,
}

impl Account {
    pub fn new(id: AccountId, username: String, host: String, session: AccountSession) -> Self {
        Self {
            id,
            username,
            host,
            session,
        }
    }

    pub fn id(&self) -> AccountId {
        self.id.clone()
    }

    pub fn session(&self) -> &AccountSession {
        &self.session
    }

    pub fn username(&self) -> String {
        self.username.clone()
    }

    pub fn host(&self) -> String {
        self.host.clone()
    }
}

enum Session {
    GitHub(GitHubSessionHandle),
    GitLab(GitLabSessionHandle),
}

#[derive(Clone)]

pub struct AccountSession {
    // secrets: AppSecretsProvider,
    keyring: Arc<dyn KeyringClient>,
    inner: Arc<Session>,
}

impl AccountSession {
    pub async fn github(
        id: AccountId,
        host: String,
        // secrets: AppSecretsProvider,
        keyring: Arc<dyn KeyringClient>,

        initial_token: Option<GitHubInitialToken>,
    ) -> joinerror::Result<Self> {
        let session = GitHubSessionHandle::new(id, host, initial_token, &keyring).await?;

        Ok(Self {
            // secrets,
            keyring,
            inner: Arc::new(Session::GitHub(session)),
        })
    }

    pub async fn gitlab(
        id: AccountId,
        // client_id: ClientId,
        host: String,
        keyring: Arc<dyn KeyringClient>,
        account_auth_api_client: Arc<dyn GitLabTokenRefreshApiReq>,
        // secrets: AppSecretsProvider,
        initial_token: Option<GitLabInitialToken>,
    ) -> joinerror::Result<Self> {
        let session =
            GitLabSessionHandle::new(id, host, account_auth_api_client, initial_token, &keyring)
                .await?;

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

    pub async fn access_token(&self) -> joinerror::Result<String> {
        match self.inner.as_ref() {
            Session::GitHub(handle) => handle.access_token(&self.keyring).await,
            Session::GitLab(handle) => handle.access_token(&self.keyring).await,
        }
    }
}
