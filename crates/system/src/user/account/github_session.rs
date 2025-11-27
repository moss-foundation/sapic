use joinerror::Error;
use moss_keyring::KeyringClient;
use sapic_base::user::types::primitives::{AccountId, SessionKind};
use sapic_core::context::AnyAsyncContext;
use std::sync::Arc;

use crate::{
    ports::server_api::{auth_github_account_api::GitHubRevokeApiReq, types::GitHubRevokeRequest},
    user::account::make_secret_key,
};

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
        revoke_api_op: Arc<dyn GitHubRevokeApiReq>,
        initial_token: Option<GitHubInitialToken>,
        keyring: &Arc<dyn KeyringClient>,
    ) -> joinerror::Result<Self> {
        Ok(Self::OAuth(
            GitHubOAuthSessionHandle::new(id, host, revoke_api_op, initial_token, keyring).await?,
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

    pub(crate) async fn revoke(
        &self,
        ctx: &dyn AnyAsyncContext,
        keyring: &Arc<dyn KeyringClient>,
    ) -> joinerror::Result<()> {
        match self {
            GitHubSessionHandle::OAuth(handle) => handle.revoke(ctx, keyring).await,
            GitHubSessionHandle::PAT(handle) => handle.revoke(keyring).await,
        }
    }

    pub(crate) async fn update_pat(
        &self,
        keyring: &Arc<dyn KeyringClient>,
        pat: &str,
    ) -> joinerror::Result<()> {
        match self {
            GitHubSessionHandle::OAuth(_) => Err(Error::new::<()>(
                "cannot update PAT when the authentication method is OAuth",
            )),
            GitHubSessionHandle::PAT(handle) => handle.update_pat(keyring, pat).await,
        }
    }
}

pub(crate) struct GitHubOAuthSessionHandle {
    pub id: AccountId,
    pub host: String,

    revoke_api_op: Arc<dyn GitHubRevokeApiReq>,
}

impl GitHubOAuthSessionHandle {
    pub async fn new(
        id: AccountId,
        host: String,
        revoke_api_op: Arc<dyn GitHubRevokeApiReq>,
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

        Ok(Self {
            id,
            host,
            revoke_api_op,
        })
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

    pub async fn revoke(
        &self,
        ctx: &dyn AnyAsyncContext,
        keyring: &Arc<dyn KeyringClient>,
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

        self.revoke_api_op
            .github_revoke(ctx, GitHubRevokeRequest { access_token })
            .await
    }
}

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

    // We only need to remove the record from the keyring
    pub async fn revoke(&self, keyring: &Arc<dyn KeyringClient>) -> joinerror::Result<()> {
        let key = make_secret_key(GITHUB_PREFIX, &self.host, &self.id);
        keyring
            .delete_secret(&key)
            .await
            .map_err(|e| Error::new::<()>(e.to_string()))?;

        Ok(())
    }

    pub async fn update_pat(
        &self,
        keyring: &Arc<dyn KeyringClient>,
        pat: &str,
    ) -> joinerror::Result<()> {
        let key = make_secret_key(GITHUB_PREFIX, &self.host, &self.id);
        keyring
            .set_secret(&key, pat)
            .await
            .map_err(|e| Error::new::<()>(e.to_string()))?;

        Ok(())
    }
}
