use joinerror::OptionExt;
use moss_asp::AppSecretsProvider;
use moss_keyring::KeyringClient;
use oauth2::{
    AuthUrl, ClientId, EmptyExtraTokenFields, RefreshToken, StandardTokenResponse, TokenResponse,
    TokenUrl,
    basic::{BasicClient, BasicTokenType},
};
use std::{sync::Arc, time::Instant};
use tokio::sync::RwLock;

use crate::{
    account::common::{calc_expires_at, make_secret_key},
    models::primitives::AccountId,
};

const GITLAB_PREFIX: &str = "gl";

fn auth_url(host: &str) -> String {
    format!("https://{host}/oauth/authorize")
}
fn token_url(host: &str) -> String {
    format!("https://{host}/oauth/token")
}

pub(crate) struct LastAccessToken {
    token: String,
    expires_at: Instant,
}

pub(crate) struct GitLabSessionHandle {
    pub id: AccountId,
    pub host: String,

    token: RwLock<Option<LastAccessToken>>,

    client_id: ClientId,
}

impl GitLabSessionHandle {
    pub(crate) fn new(
        id: AccountId,
        host: String,
        client_id: ClientId,

        initial_token: Option<StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>>,

        keyring: &Arc<dyn KeyringClient>,
    ) -> joinerror::Result<Self> {
        let mut token: Option<LastAccessToken> = None;
        if let Some(initial_token) = initial_token {
            let refresh_token = initial_token
                .refresh_token()
                .ok_or_join_err::<()>(" refresh_token value not received")?
                .secret()
                .to_owned();
            let token_duration = initial_token
                .expires_in()
                .ok_or_join_err::<()>(" expires_in value not received")?;

            let access_token = initial_token.access_token().secret().to_owned();

            keyring
                .set_secret(&make_secret_key(GITLAB_PREFIX, &host, &id), &refresh_token)
                .map_err(|e| joinerror::Error::new::<()>(e.to_string()))?;

            token = Some(LastAccessToken {
                token: access_token,
                expires_at: calc_expires_at(token_duration),
            });
        };

        Ok(Self {
            id,
            host,
            token: RwLock::new(token),

            client_id,
        })
    }

    pub(crate) async fn access_token(
        &self,
        keyring: &Arc<dyn KeyringClient>,
        secrets: &AppSecretsProvider,
    ) -> joinerror::Result<String> {
        if let Some(token) = self.token.read().await.as_ref() {
            if token.expires_at > Instant::now() {
                return Ok(token.token.clone()); // Token is still valid
            }
        }

        let key = make_secret_key(GITLAB_PREFIX, &self.host, &self.id);
        let bytes = keyring
            .get_secret(&key)
            .map_err(|e| joinerror::Error::new::<()>(e.to_string()))?;

        let old_refresh_token = String::from_utf8(bytes.to_vec())?;
        let token = self
            .refresh_gitlab_token(old_refresh_token, secrets)
            .await?;

        let token_duration = token.expires_in().ok_or_join_err::<()>(
            "failed to perform refresh GitLab credentials operation: expires_in value not received",
        )?;

        let new_access_token = token.access_token().secret().to_owned();
        self.token.write().await.replace(LastAccessToken {
            token: new_access_token.clone(),

            // Force refreshing the access token half an hour before the actual expiry
            // To avoid any timing issue
            expires_at: calc_expires_at(token_duration),
        });

        if let Some(new_refresh_token) = token.refresh_token() {
            let new_refresh_token = new_refresh_token.secret().to_owned();

            keyring
                .set_secret(&key, &new_refresh_token)
                .map_err(|e| joinerror::Error::new::<()>(e.to_string()))?;
        }

        return Ok(new_access_token);
    }

    async fn refresh_gitlab_token(
        &self,
        refresh_token: String,
        secrets: &AppSecretsProvider,
    ) -> joinerror::Result<StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>> {
        let client_secret = secrets.gitlab_client_secret()?;
        let client = BasicClient::new(self.client_id.clone())
            .set_client_secret(client_secret)
            .set_auth_uri(AuthUrl::new(auth_url(&self.host))?)
            .set_token_uri(TokenUrl::new(token_url(&self.host))?);

        let token = client
            .exchange_refresh_token(&RefreshToken::new(refresh_token))
            .request_async(&reqwest::Client::new()) // TODO: reuse client instead of creating a new one
            .await
            .map_err(|e| joinerror::Error::new::<()>(e.to_string()))?;

        Ok(token)
    }
}
