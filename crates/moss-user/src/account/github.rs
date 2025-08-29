use moss_keyring::KeyringClient;
use oauth2::{EmptyExtraTokenFields, StandardTokenResponse, TokenResponse, basic::BasicTokenType};
use std::sync::Arc;

use crate::{account::common::make_secret_key, models::primitives::AccountId};

const GITHUB_PREFIX: &str = "gh";

pub(crate) struct GitHubSessionHandle {
    pub id: AccountId,
    pub host: String,
}

impl GitHubSessionHandle {
    pub fn new(
        id: AccountId,
        host: String,
        initial_token: Option<StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>>,

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
                    &initial_token.access_token().secret().to_string(),
                )
                .map_err(|e| joinerror::Error::new::<()>(e.to_string()))?;
        };

        Ok(Self { id, host })
    }

    pub async fn access_token(
        &self,
        keyring: &Arc<dyn KeyringClient>,
    ) -> joinerror::Result<String> {
        let key = make_secret_key(GITHUB_PREFIX, &self.host, &self.id);
        let bytes = keyring
            .get_secret(&key)
            .map_err(|e| joinerror::Error::new::<()>(e.to_string()))?;

        let access_token = String::from_utf8(bytes.to_vec())?;

        // A GitHub OAuth App doesn’t issue a `refresh_token`;
        // instead, it provides a long-lived `access_token`.
        // So we store it in the keyring and return it immediately.
        return Ok(access_token);
    }
}
