use anyhow::{Result, anyhow};
use git2::{Cred, RemoteCallbacks};
use moss_git::GitAuthAgent;
use moss_keyring::KeyringClient;
use oauth2::{
    AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl, Scope,
    TokenResponse, TokenUrl, basic::BasicClient,
};
use serde::{Deserialize, Serialize};
use std::{
    string::ToString,
    sync::{Arc, OnceLock},
};

use crate::common::utils;

const GITHUB_AUTH_URL: &'static str = "https://github.com/login/oauth/authorize";
const GITHUB_TOKEN_URL: &'static str = "https://github.com/login/oauth/access_token";
const GITHUB_SCOPES: [&'static str; 3] = ["repo", "user:email", "read:user"];
const KEYRING_SECRET_KEY: &str = "github_auth_agent";

#[derive(Debug, Deserialize, Serialize)]
pub struct KeyringCredEntry {
    access_token: String,
}

impl From<&GitHubCred> for KeyringCredEntry {
    fn from(value: &GitHubCred) -> Self {
        Self {
            access_token: value.access_token.clone(),
        }
    }
}

impl TryInto<String> for KeyringCredEntry {
    type Error = anyhow::Error;

    fn try_into(self) -> std::result::Result<String, Self::Error> {
        Ok(serde_json::to_string(&self)?)
    }
}

#[derive(Clone, Debug)]
pub struct GitHubCred {
    access_token: String,
}

impl GitHubCred {
    pub fn new(access_token: &str) -> Self {
        Self {
            access_token: access_token.to_string(),
        }
    }
}

impl From<KeyringCredEntry> for GitHubCred {
    fn from(value: KeyringCredEntry) -> Self {
        Self {
            access_token: value.access_token,
        }
    }
}

pub struct GitHubAuthAgent {
    // We use ureq instead of blocking reqwest to avoid panicking when called from async environment
    sync_http_client: oauth2::ureq::Agent,
    client_id: ClientId,
    client_secret: ClientSecret,
    keyring: Arc<dyn KeyringClient + Send + Sync>,
    cred: OnceLock<GitHubCred>,
}

impl GitHubAuthAgent {
    pub fn new(
        sync_http_client: oauth2::ureq::Agent,
        keyring: Arc<dyn KeyringClient + Send + Sync>,
        client_id: String,
        client_secret: String,
    ) -> Self {
        Self {
            sync_http_client,
            client_id: ClientId::new(client_id),
            client_secret: ClientSecret::new(client_secret),
            keyring,
            cred: OnceLock::new(),
        }
    }

    pub(crate) fn access_token(self: Arc<Self>) -> Option<String> {
        self.cred.get().map(|cred| cred.access_token.clone())
    }

    pub fn is_logged_in(&self) -> joinerror::Result<bool> {
        // We consider the user to be logged in if we have the credentials
        Ok(self.cred.get().is_some())
    }
}

// TODO: Add timeout mechanism to handle OAuth failure

impl GitHubAuthAgent {
    // FIXME: Maybe we really need to figure out how to use a non-blocking `reqwest` for auth_agent
    /// Do not call the sync version from an async environment
    /// Call `credentials_async` instead
    pub(crate) fn credentials(&self) -> Result<&GitHubCred> {
        if let Some(cred) = self.cred.get() {
            return Ok(cred);
        }

        // In tests and CI, fetch GITHUB_ACCESS_TOKEN from the environment
        if let Some(fetched_cred) = self.try_fetch_from_env()? {
            return Ok(fetched_cred);
        }

        let cred = match self.keyring.get_secret(KEYRING_SECRET_KEY) {
            Ok(data) => {
                let stored_entry: KeyringCredEntry = serde_json::from_slice(&data)?;

                GitHubCred::from(stored_entry)
            }
            // Err(keyring::Error::NoEntry) => {
            //     let initial_cred = self.gen_initial_credentials()?;
            //     let entry_str: String = KeyringCredEntry::from(&initial_cred).try_into()?;
            //     self.keyring.set_secret(KEYRING_SECRET_KEY, &entry_str)?;

            //     initial_cred
            // }
            Err(err) => return Err(err.into()),
        };

        let _ = self.cred.set(cred);
        self.cred.get().ok_or_else(|| {
            anyhow!("Failed to set GitHubAuthAgent credentials because they have already been set")
        })
    }

    pub(crate) async fn credentials_async(self: Arc<Self>) -> Result<GitHubCred> {
        let self_clone = self.clone();
        tokio::task::spawn_blocking(move || self_clone.credentials().map(|cred| cred.to_owned()))
            .await?
    }

    // A helper method to avoid false positive about unreachable code
    // It will fetch the access token from the environment
    #[cfg(any(test, feature = "integration-tests"))]
    fn try_fetch_from_env(&self) -> Result<Option<&GitHubCred>> {
        dotenv::dotenv().ok();
        let cred = GitHubCred {
            access_token: dotenv::var(crate::envvar_keys::GITHUB_ACCESS_TOKEN)?,
        };
        let _ = self.cred.set(cred);

        let fetched_cred = self.cred.get();
        Ok(fetched_cred)
    }

    #[cfg(not(any(test, feature = "integration-tests")))]
    fn try_fetch_from_env(&self) -> Result<Option<&GitHubCred>> {
        Ok(None)
    }

    fn gen_initial_credentials(&self) -> Result<GitHubCred> {
        let (listener, callback_port) = utils::create_auth_tcp_listener()?;

        let client = BasicClient::new(self.client_id.clone())
            .set_client_secret(self.client_secret.clone())
            .set_auth_uri(AuthUrl::new(GITHUB_AUTH_URL.to_string())?)
            .set_token_uri(TokenUrl::new(GITHUB_TOKEN_URL.to_string())?)
            .set_redirect_uri(RedirectUrl::new(format!(
                "http://127.0.0.1:{}",
                callback_port.to_string()
            ))?);

        // https://datatracker.ietf.org/doc/html/rfc7636#section-1.1
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        // Generate the authorization URL to which we'll redirect the user.
        let (authorize_url, _csrf_state) = client
            .authorize_url(CsrfToken::new_random)
            .add_scopes(GITHUB_SCOPES.into_iter().map(|s| Scope::new(s.to_string())))
            // Let the user select which account to authorize
            // https://docs.github.com/en/apps/oauth-apps/building-oauth-apps/authorizing-oauth-apps
            .add_extra_param("prompt", "select_account")
            .set_pkce_challenge(pkce_challenge)
            .url();

        if webbrowser::open(&authorize_url.to_string()).is_err() {
            println!("Open this URL in your browser:\n{authorize_url}\n");
        }

        let (code, _state) = utils::receive_auth_code(&listener)?;

        // Exchange the code + PKCE verifier with access & refresh token.
        let token_res = client
            .exchange_code(code)
            .set_pkce_verifier(pkce_verifier)
            .request(&self.sync_http_client)?;

        let access_token = token_res.access_token().secret().as_str();

        Ok(GitHubCred::new(access_token))
    }
}

impl GitAuthAgent for GitHubAuthAgent {
    fn generate_callback<'a>(&'a self, cb: &mut RemoteCallbacks<'a>) -> Result<()> {
        let cred = self.credentials()?;

        cb.credentials(move |_url, _username_from_url, _allowed_types| {
            Cred::userpass_plaintext("oauth2", &cred.access_token)
        });

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use moss_git::repo::RepoHandle;
    use moss_keyring::KeyringClientImpl;
    use std::{path::PathBuf, sync::Arc};

    use crate::envvar_keys::{GITHUB_CLIENT_ID, GITHUB_CLIENT_SECRET};

    #[ignore]
    #[test]
    fn manual_cloning_with_oauth() -> Result<()> {
        dotenv::dotenv().ok();
        let repo_url = &dotenv::var("GITHUB_TEST_REPO_HTTPS").unwrap();
        let repo_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("data")
            .join("test-repo");

        let client_id = dotenv::var(GITHUB_CLIENT_ID).unwrap();
        let client_secret = dotenv::var(GITHUB_CLIENT_SECRET).unwrap();

        let keyring_client = Arc::new(KeyringClientImpl::new());
        let auth_agent = Arc::new(GitHubAuthAgent::new(
            oauth2::ureq::builder().redirects(0).build(),
            keyring_client,
            client_id,
            client_secret,
        ));

        let _repo = RepoHandle::clone(&repo_url, &repo_path, auth_agent)?;
        Ok(())
    }
}
