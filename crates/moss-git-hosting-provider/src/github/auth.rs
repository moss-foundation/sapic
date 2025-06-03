use anyhow::{Result, anyhow};
use git2::{Cred, RemoteCallbacks};
use moss_git::GitAuthAgent;
use moss_keyring::KeyringClient;
use oauth2::{
    AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl, Scope,
    TokenResponse, TokenUrl, basic::BasicClient,
};
use serde::{Deserialize, Serialize};
use std::{cell::OnceCell, string::ToString, sync::Arc};

use crate::common::utils;

use super::client::GitHubAuthAgent;

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

#[derive(Debug)]
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

const GITHUB_AUTH_URL: &'static str = "https://github.com/login/oauth/authorize";
const GITHUB_TOKEN_URL: &'static str = "https://github.com/login/oauth/access_token";
const GITHUB_SCOPES: [&'static str; 3] = ["repo", "read:user", "user:email"];
const KEYRING_SECRET_KEY: &str = "github_auth_agent";

pub struct GitHubAuthAgentImpl {
    client_id: ClientId,
    client_secret: ClientSecret,
    keyring: Arc<dyn KeyringClient>,
    cred: OnceCell<GitHubCred>,
}

impl GitHubAuthAgent for GitHubAuthAgentImpl {}

impl GitHubAuthAgentImpl {
    pub fn new(keyring: Arc<dyn KeyringClient>, client_id: String, client_secret: String) -> Self {
        Self {
            client_id: ClientId::new(client_id),
            client_secret: ClientSecret::new(client_secret),
            keyring,
            cred: OnceCell::new(),
        }
    }
}

impl GitHubAuthAgentImpl {
    fn credentials(&self) -> Result<&GitHubCred> {
        if let Some(cred) = self.cred.get() {
            return Ok(cred);
        }

        let cred = match self.keyring.get_secret(KEYRING_SECRET_KEY) {
            Ok(data) => {
                let stored_entry: KeyringCredEntry = serde_json::from_slice(&data)?;

                GitHubCred::from(stored_entry)
            }
            Err(keyring::Error::NoEntry) => {
                let initial_cred = self.gen_initial_credentials()?;
                let entry_str: String = KeyringCredEntry::from(&initial_cred).try_into()?;
                self.keyring.set_secret(KEYRING_SECRET_KEY, &entry_str)?;

                initial_cred
            }
            Err(err) => return Err(err.into()),
        };

        let _ = self.cred.set(cred);
        self.cred.get().ok_or_else(|| {
            anyhow!("Failed to set GitHubAuthAgent credentials because they have already been set")
        })
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

        let http_client = reqwest::blocking::ClientBuilder::new()
            .redirect(reqwest::redirect::Policy::none())
            .build()?;

        // Exchange the code + PKCE verifier with access & refresh token.
        let token_res = client
            .exchange_code(code)
            .set_pkce_verifier(pkce_verifier)
            .request(&http_client)?;

        let access_token = token_res.access_token().secret().as_str();

        Ok(GitHubCred::new(access_token))
    }
}

impl GitAuthAgent for GitHubAuthAgentImpl {
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
    use std::{path::Path, sync::Arc};

    #[test]
    #[ignore]
    fn manual_cloning_with_oauth() -> Result<()> {
        dotenv::dotenv().ok();
        let repo_url = &dotenv::var("GITHUB_TEST_REPO_HTTPS").unwrap();
        let repo_path = Path::new("test-repo");

        let client_id = dotenv::var("GITHUB_CLIENT_ID").unwrap();
        let client_secret = dotenv::var("GITHUB_CLIENT_SECRET").unwrap();

        let keyring_client = Arc::new(KeyringClientImpl::new());
        let auth_agent = Arc::new(GitHubAuthAgentImpl::new(
            keyring_client,
            client_id,
            client_secret,
        ));

        let repo = RepoHandle::clone(repo_url, repo_path, auth_agent)?;
        Ok(())
    }
}
