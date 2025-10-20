use crate::{
    account::common::make_secret_key,
    models::primitives::{AccountId, SessionKind},
};
use chrono::{DateTime, Utc};
use joinerror::Error;
use moss_applib::AppRuntime;
use moss_keyring::KeyringClient;
use moss_server_api::account_auth_gateway::{
    GitHubRevokeApiReq, GitHubRevokeRequest, GitLabPkceTokenExchangeApiReq,
};
use std::sync::Arc;

const GITHUB_PREFIX: &str = "gh";

pub struct GitHubInitialToken {
    pub access_token: String,
}

pub struct GitHubPAT {
    pub token: String,
}

pub(crate) enum GitHubSessionHandle {
    OAuth(GitHubOAuthSessionHandle),
    PAT(GitHubPATSessionHandle),
}

impl GitHubSessionHandle {
    pub(crate) async fn oauth(
        id: AccountId,
        host: String,
        initial_token: Option<GitHubInitialToken>,
        keyring: &Arc<dyn KeyringClient>,
    ) -> joinerror::Result<Self> {
        Ok(Self::OAuth(
            GitHubOAuthSessionHandle::new(id, host, initial_token, keyring).await?,
        ))
    }
    pub(crate) async fn pat(
        id: AccountId,
        host: String,
        pat: Option<GitHubPAT>,
        keyring: &Arc<dyn KeyringClient>,
    ) -> joinerror::Result<Self> {
        Ok(Self::PAT(
            GitHubPATSessionHandle::new(id, host, pat, keyring).await?,
        ))
    }

    pub(crate) async fn token(
        &self,
        keyring: &Arc<dyn KeyringClient>,
    ) -> joinerror::Result<String> {
        match self {
            GitHubSessionHandle::OAuth(handle) => handle.token(keyring).await,
            GitHubSessionHandle::PAT(handle) => handle.token(keyring).await,
        }
    }

    pub(crate) fn host(&self) -> String {
        match self {
            GitHubSessionHandle::OAuth(handle) => handle.host.clone(),
            GitHubSessionHandle::PAT(handle) => handle.host.clone(),
        }
    }

    pub(crate) fn session_kind(&self) -> SessionKind {
        match self {
            GitHubSessionHandle::OAuth(_) => SessionKind::OAuth,
            GitHubSessionHandle::PAT(_) => SessionKind::PAT,
        }
    }

    pub(crate) async fn revoke<R: AppRuntime>(
        &self,
        ctx: &R::AsyncContext,
        keyring: &Arc<dyn KeyringClient>,
        api_client: Arc<dyn GitHubRevokeApiReq<R>>,
    ) -> joinerror::Result<()> {
        match self {
            GitHubSessionHandle::OAuth(handle) => handle.revoke(ctx, keyring, api_client).await,
            GitHubSessionHandle::PAT(handle) => {
                unimplemented!()
            }
        }
    }
}

pub(crate) struct GitHubOAuthSessionHandle {
    pub id: AccountId,
    pub host: String,
}

impl GitHubOAuthSessionHandle {
    pub async fn new(
        id: AccountId,
        host: String,

        initial_token: Option<GitHubInitialToken>,

        keyring: &Arc<dyn KeyringClient>,
    ) -> joinerror::Result<Self> {
        // An OAuth App traditionally doesn’t issue a `refresh_token`;
        // instead, it provides a long-lived `access_token`. The token
        // can be manually revoked, automatically revoked if unused for a year,
        // or revoked if it leaks into a public repository.

        if let Some(initial_token) = initial_token {
            keyring
                .set_secret(
                    &make_secret_key(GITHUB_PREFIX, &host, &id),
                    &initial_token.access_token,
                )
                .await
                .map_err(|e| Error::new::<()>(e.to_string()))?;
        };

        Ok(Self { id, host })
    }

    pub async fn token(&self, keyring: &Arc<dyn KeyringClient>) -> joinerror::Result<String> {
        let key = make_secret_key(GITHUB_PREFIX, &self.host, &self.id);
        let bytes = keyring
            .get_secret(&key)
            .await
            .map_err(|e| Error::new::<()>(e.to_string()))?;

        let access_token = String::from_utf8(bytes.to_vec())?;

        // A GitHub OAuth App doesn’t issue a `refresh_token`;
        // instead, it provides a long-lived `access_token`.
        // So we store it in the keyring and return it immediately.
        return Ok(access_token);
    }

    pub async fn revoke<R: AppRuntime>(
        &self,
        ctx: &R::AsyncContext,
        keyring: &Arc<dyn KeyringClient>,
        api_client: Arc<dyn GitHubRevokeApiReq<R>>,
    ) -> joinerror::Result<()> {
        let key = make_secret_key(GITHUB_PREFIX, &self.host, &self.id);
        let bytes = keyring
            .get_secret(&key)
            .await
            .map_err(|e| Error::new::<()>(e.to_string()))?;

        let access_token = String::from_utf8(bytes.to_vec())?;

        keyring
            .delete_secret(&key)
            .await
            .map_err(|e| Error::new::<()>(e.to_string()))?;

        api_client
            .github_revoke(ctx, GitHubRevokeRequest { access_token })
            .await
    }
}

// TODO: A method to update PAT?
pub(crate) struct GitHubPATSessionHandle {
    pub id: AccountId,
    pub host: String,
}

impl GitHubPATSessionHandle {
    pub async fn new(
        id: AccountId,
        host: String,
        pat: Option<GitHubPAT>,
        keyring: &Arc<dyn KeyringClient>,
    ) -> joinerror::Result<Self> {
        if let Some(pat) = pat {
            keyring
                .set_secret(&make_secret_key(GITHUB_PREFIX, &host, &id), &pat.token)
                .await
                .map_err(|e| Error::new::<()>(e.to_string()))?;
        };

        Ok(Self { id, host })
    }

    pub async fn token(&self, keyring: &Arc<dyn KeyringClient>) -> joinerror::Result<String> {
        let key = make_secret_key(GITHUB_PREFIX, &self.host, &self.id);
        let bytes = keyring
            .get_secret(&key)
            .await
            .map_err(|e| Error::new::<()>(e.to_string()))?;

        let token = String::from_utf8(bytes.to_vec())?;

        return Ok(token);
    }
}
