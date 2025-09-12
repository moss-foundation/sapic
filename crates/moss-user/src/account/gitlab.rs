use joinerror::Error;
use moss_applib::AppRuntime;
use moss_keyring::KeyringClient;
use moss_server_api::account_auth_gateway::{GitLabTokenRefreshApiReq, GitLabTokenRefreshRequest};
use std::{
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::RwLock;

use crate::{
    account::{
        common::{calc_expires_at, make_secret_key},
        github::GitHubSessionHandle,
    },
    models::primitives::{AccountId, SessionKind},
};

const GITLAB_PREFIX: &str = "gl";

pub struct GitLabInitialToken {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
}

pub struct GitLabPAT {
    pub token: String,
}

pub(crate) struct LastAccessToken {
    // Access token
    token: String,

    // GitLab access tokens are valid for 2 hours. But we refresh them half
    // an hour before the expiry to avoid any timing issue.
    expires_at: Instant,
}

pub(crate) enum GitLabSessionHandle<R: AppRuntime> {
    OAuth(GitLabOAuthSessionHandle<R>),
    PAT(GitLabPATSessionHandle),
}

impl<R: AppRuntime> GitLabSessionHandle<R> {
    pub(crate) async fn oauth(
        id: AccountId,
        host: String,
        auth_api_client: Arc<dyn GitLabTokenRefreshApiReq<R>>,
        initial_token: Option<GitLabInitialToken>,
        keyring: &Arc<dyn KeyringClient>,
    ) -> joinerror::Result<Self> {
        Ok(Self::OAuth(
            GitLabOAuthSessionHandle::new(id, host, auth_api_client, initial_token, keyring)
                .await?,
        ))
    }

    pub(crate) async fn pat(
        id: AccountId,
        host: String,
        pat: Option<GitLabPAT>,
        keyring: &Arc<dyn KeyringClient>,
    ) -> joinerror::Result<Self> {
        Ok(Self::PAT(
            GitLabPATSessionHandle::new(id, host, pat, keyring).await?,
        ))
    }

    pub(crate) async fn token(
        &self,
        ctx: &R::AsyncContext,
        keyring: &Arc<dyn KeyringClient>,
    ) -> joinerror::Result<String> {
        match self {
            GitLabSessionHandle::OAuth(handle) => handle.token(ctx, keyring).await,
            GitLabSessionHandle::PAT(handle) => handle.token(keyring).await,
        }
    }

    pub(crate) fn host(&self) -> String {
        match self {
            GitLabSessionHandle::OAuth(handle) => handle.host.clone(),
            GitLabSessionHandle::PAT(handle) => handle.host.clone(),
        }
    }

    pub(crate) fn session_kind(&self) -> SessionKind {
        match self {
            GitLabSessionHandle::OAuth(_) => SessionKind::OAuth,
            GitLabSessionHandle::PAT(_) => SessionKind::PAT,
        }
    }
}

pub(crate) struct GitLabOAuthSessionHandle<R: AppRuntime> {
    pub id: AccountId,
    pub host: String,

    auth_api_client: Arc<dyn GitLabTokenRefreshApiReq<R>>,
    token: RwLock<Option<LastAccessToken>>,
}

impl<R: AppRuntime> GitLabOAuthSessionHandle<R> {
    pub(crate) async fn new(
        id: AccountId,
        host: String,
        auth_api_client: Arc<dyn GitLabTokenRefreshApiReq<R>>,
        initial_token: Option<GitLabInitialToken>,
        keyring: &Arc<dyn KeyringClient>,
    ) -> joinerror::Result<Self> {
        let mut token: Option<LastAccessToken> = None;
        if let Some(initial_token) = initial_token {
            keyring
                .set_secret(
                    &make_secret_key(GITLAB_PREFIX, &host, &id),
                    &initial_token.refresh_token,
                )
                .await
                .map_err(|e| Error::new::<()>(e.to_string()))?;

            token = Some(LastAccessToken {
                token: initial_token.access_token,
                expires_at: calc_expires_at(Duration::from_secs(initial_token.expires_in)),
            });
        };

        Ok(Self {
            id,
            host,
            auth_api_client,
            token: RwLock::new(token),
        })
    }

    pub(crate) async fn token(
        &self,
        ctx: &R::AsyncContext,
        keyring: &Arc<dyn KeyringClient>,
    ) -> joinerror::Result<String> {
        if let Some(token) = self.token.read().await.as_ref() {
            if token.expires_at > Instant::now() {
                return Ok(token.token.clone()); // Token is still valid
            }
        }

        let key = make_secret_key(GITLAB_PREFIX, &self.host, &self.id);
        let bytes = keyring
            .get_secret(&key)
            .await
            .map_err(|e| Error::new::<()>(e.to_string()))?;

        let old_refresh_token = String::from_utf8(bytes.to_vec())?;

        let resp = self
            .auth_api_client
            .gitlab_token_refresh(
                ctx,
                GitLabTokenRefreshRequest {
                    refresh_token: old_refresh_token,
                },
            )
            .await?;

        self.token.write().await.replace(LastAccessToken {
            token: resp.access_token.clone(),

            // Force refreshing the access token half an hour before the actual expiry
            // To avoid any timing issue
            expires_at: calc_expires_at(Duration::from_secs(resp.expires_in)),
        });

        keyring
            .set_secret(&key, &resp.refresh_token)
            .await
            .map_err(|e| Error::new::<()>(e.to_string()))?;

        return Ok(resp.access_token);
    }
}

pub(crate) struct GitLabPATSessionHandle {
    pub id: AccountId,
    pub host: String,
}

impl GitLabPATSessionHandle {
    pub async fn new(
        id: AccountId,
        host: String,
        pat: Option<GitLabPAT>,
        keyring: &Arc<dyn KeyringClient>,
    ) -> joinerror::Result<Self> {
        if let Some(pat) = pat {
            keyring
                .set_secret(&make_secret_key(GITLAB_PREFIX, &host, &id), &pat.token)
                .await
                .map_err(|e| Error::new::<()>(e.to_string()))?;
        };
        Ok(Self { id, host })
    }

    pub async fn token(&self, keyring: &Arc<dyn KeyringClient>) -> joinerror::Result<String> {
        let key = make_secret_key(GITLAB_PREFIX, &self.host, &self.id);
        let bytes = keyring
            .get_secret(&key)
            .await
            .map_err(|e| Error::new::<()>(e.to_string()))?;

        let token = String::from_utf8(bytes.to_vec())?;
        Ok(token)
    }
}
