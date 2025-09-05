use joinerror::Error;
use moss_keyring::KeyringClient;
use std::{
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::RwLock;

use crate::{
    account::{
        auth_gateway_api::{GitLabTokenRefreshApiReq, GitLabTokenRefreshRequest},
        common::{calc_expires_at, make_secret_key},
    },
    models::primitives::AccountId,
};

const GITLAB_PREFIX: &str = "gl";

pub struct GitLabInitialToken {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
}

pub(crate) struct LastAccessToken {
    // Access token
    token: String,

    // GitLab access tokens are valid for 2 hours. But we refresh them half
    // an hour before the expiry to avoid any timing issue.
    expires_at: Instant,
}

pub(crate) struct GitLabSessionHandle {
    pub id: AccountId,
    pub host: String,

    auth_api_client: Arc<dyn GitLabTokenRefreshApiReq>,
    token: RwLock<Option<LastAccessToken>>,
}

impl GitLabSessionHandle {
    pub(crate) async fn new(
        id: AccountId,
        host: String,
        auth_api_client: Arc<dyn GitLabTokenRefreshApiReq>,
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

    pub(crate) async fn access_token(
        &self,
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
            .gitlab_token_refresh(GitLabTokenRefreshRequest {
                refresh_token: old_refresh_token,
            })
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
