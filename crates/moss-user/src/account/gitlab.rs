use joinerror::{Error, OptionExt};
use moss_asp::AppSecretsProvider;
use moss_keyring::KeyringClient;
use oauth2::{
    AuthUrl, ClientId, EmptyExtraTokenFields, RefreshToken, StandardTokenResponse, TokenResponse,
    TokenUrl,
    basic::{BasicClient, BasicTokenType},
};
use reqwest::Client as HttpClient;
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
const API_PATH_TOKEN_REFRESH: &str = "/auth/gitlab/refresh";

// fn auth_url(host: &str) -> String {
//     format!("https://{host}/oauth/authorize")
// }
// fn token_url(host: &str) -> String {
//     format!("https://{host}/oauth/token")
// }

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

    account_auth_api_client: Arc<dyn GitLabTokenRefreshApiReq>,
    token: RwLock<Option<LastAccessToken>>,
    // client_id: ClientId,
}

impl GitLabSessionHandle {
    pub(crate) async fn new(
        id: AccountId,
        host: String,
        account_auth_api_client: Arc<dyn GitLabTokenRefreshApiReq>,
        // client_id: ClientId,
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
            account_auth_api_client,
            token: RwLock::new(token),
            // client_id,
        })
    }

    pub(crate) async fn access_token(
        &self,
        keyring: &Arc<dyn KeyringClient>,
        // secrets: &AppSecretsProvider,
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
            .account_auth_api_client
            .gitlab_token_refresh(GitLabTokenRefreshRequest {
                refresh_token: old_refresh_token,
            })
            .await?;

        // let token = self
        //     .refresh_gitlab_token(old_refresh_token, secrets)
        //     .await?;

        // let token_duration = token.expires_in().ok_or_join_err::<()>(
        //     "failed to perform refresh GitLab credentials operation: expires_in value not received",
        // )?;

        // let new_access_token = token.access_token().secret().to_owned();

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

    // async fn refresh_gitlab_token(
    //     &self,
    //     refresh_token: String,
    //     secrets: &AppSecretsProvider,
    // ) -> joinerror::Result<GitLabTokenRefreshResponse> {
    //     let resp = self
    //         .account_auth_api_client
    //         .gitlab_token_refresh(GitLabTokenRefreshRequest { refresh_token })
    //         .await?;
    //     todo!();
    //     // let client_secret = secrets.gitlab_client_secret().await?;
    //     // let client = BasicClient::new(self.client_id.clone())
    //     //     .set_client_secret(client_secret)
    //     //     .set_auth_uri(AuthUrl::new(auth_url(&self.host))?)
    //     //     .set_token_uri(TokenUrl::new(token_url(&self.host))?);

    //     // let token = client
    //     //     .exchange_refresh_token(&RefreshToken::new(refresh_token))
    //     //     .request_async(&reqwest::Client::new()) // TODO: reuse client instead of creating a new one
    //     //     .await
    //     //     .map_err(|e| joinerror::Error::new::<()>(e.to_string()))?;

    //     // Ok(token)
    // }
}
