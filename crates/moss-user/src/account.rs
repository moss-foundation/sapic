mod common;
mod github;
mod gitlab;

use moss_asp::AppSecretsProvider;
use moss_keyring::KeyringClient;
use oauth2::{ClientId, EmptyExtraTokenFields, StandardTokenResponse, basic::BasicTokenType};
use std::sync::Arc;

use crate::{
    account::{github::GitHubSessionHandle, gitlab::GitLabSessionHandle},
    models::primitives::AccountId,
};

enum Session {
    GitHub(GitHubSessionHandle),
    GitLab(GitLabSessionHandle),
}

// There are two scenarios for creating an account handler.
// The first is when we create a handler from an already added account (like when restoring a profile).
// The second is when we’ve just added an account and received the necessary token as part of that process.

pub struct AccountSession {
    secrets: AppSecretsProvider,
    keyring: Arc<dyn KeyringClient>,
    inner: Session,
}

impl AccountSession {
    pub fn github(
        id: AccountId,
        host: String,
        secrets: AppSecretsProvider,
        keyring: Arc<dyn KeyringClient>,

        initial_token: Option<StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>>,
    ) -> joinerror::Result<Self> {
        let session = GitHubSessionHandle::new(id, host, initial_token, &keyring)?;

        Ok(Self {
            secrets,
            keyring,
            inner: Session::GitHub(session),
        })
    }

    pub fn gitlab(
        id: AccountId,
        client_id: ClientId,
        host: String,
        keyring: Arc<dyn KeyringClient>,
        secrets: AppSecretsProvider,

        initial_token: Option<StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>>,
    ) -> joinerror::Result<Self> {
        let session = GitLabSessionHandle::new(id, host, client_id, initial_token, &keyring)?;

        Ok(Self {
            secrets,
            keyring,
            inner: Session::GitLab(session),
        })
    }

    pub fn host(&self) -> String {
        match &self.inner {
            Session::GitHub(handle) => handle.host.clone(),
            Session::GitLab(handle) => handle.host.clone(),
        }
    }

    pub async fn access_token(&self) -> joinerror::Result<String> {
        match &self.inner {
            Session::GitHub(handle) => handle.access_token(&self.keyring).await,
            Session::GitLab(handle) => handle.access_token(&self.keyring, &self.secrets).await,
        }
    }
}
