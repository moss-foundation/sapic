use anyhow::{anyhow, Result};
use git2::{Cred, RemoteCallbacks};
use moss_git::GitAuthAgent;
use moss_keyring::KeyringClient;
use oauth2::basic::BasicClient;
use oauth2::{
    AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl, Scope,
    TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};
use std::cell::OnceCell;
use std::net::TcpListener;
use std::string::ToString;
use std::sync::Arc;

use crate::common::utils;

use super::client::GitHubAuthAgent;

#[derive(Debug, Deserialize, Serialize)]
pub struct KeyringCredEntry {
    access_token: String,
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

    pub fn access_token(&self) -> &str {
        &self.access_token
    }
}

const GITHUB_AUTH_URL: &'static str = "https://github.com/login/oauth/authorize";
const GITHUB_TOKEN_URL: &'static str = "https://github.com/login/oauth/access_token";
const GITHUB_SCOPES: [&'static str; 3] = ["repo", "read:user", "user:email"];
const KEYRING_SECRET_KEY: &str = "github_auth_agent";

pub struct GitHubAuthAgentImpl {
    keyring: Arc<dyn KeyringClient>,
    cred: OnceCell<GitHubCred>,
}

impl GitHubAuthAgent for GitHubAuthAgentImpl {}

impl GitHubAuthAgentImpl {
    pub fn new(keyring: Arc<dyn KeyringClient>) -> Self {
        Self {
            keyring,
            cred: OnceCell::new(),
        }
    }
}

impl GitHubAuthAgentImpl {
    fn client_id() -> Result<ClientId> {
        dotenv::dotenv()?;
        Ok(ClientId::new(dotenv::var("GITHUB_CLIENT_ID")?))
    }
    fn client_secret() -> Result<ClientSecret> {
        dotenv::dotenv()?;
        Ok(ClientSecret::new(dotenv::var("GITHUB_CLIENT_SECRET")?))
    }

    fn credentials(&self) -> Result<&GitHubCred> {
        if let Some(cred) = self.cred.get() {
            return Ok(cred);
        }

        let cred = match self.keyring.get_secret(KEYRING_SECRET_KEY) {
            Ok(data) => {
                let entry: KeyringCredEntry = serde_json::from_slice(&data)?;

                GitHubCred {
                    access_token: entry.access_token,
                }
            }
            Err(keyring::Error::NoEntry) => {
                let cred = self.gen_initial_credentials()?;
                let cred_str = serde_json::to_string(&KeyringCredEntry {
                    access_token: cred.access_token.clone(),
                })?;
                self.keyring.set_secret(KEYRING_SECRET_KEY, &cred_str)?;

                cred
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

        let client = BasicClient::new(GitHubAuthAgentImpl::client_id()?)
            .set_client_secret(GitHubAuthAgentImpl::client_secret()?)
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
            Cred::userpass_plaintext("oauth2", cred.access_token())
        });

        Ok(())
    }
}

#[cfg(test)]
mod github_tests {
    use super::*;

    use moss_git::repo::RepoHandle;
    use moss_keyring::KeyringClientImpl;
    use std::path::Path;
    use std::sync::Arc;

    #[test]
    fn cloning_with_oauth() -> Result<()> {
        dotenv::dotenv().ok();
        let repo_url = &dotenv::var("GITHUB_TEST_REPO_HTTPS").unwrap();
        let repo_path = Path::new("test-repo");

        let keyring_client = Arc::new(KeyringClientImpl::new());
        let auth_agent = Arc::new(GitHubAuthAgentImpl::new(keyring_client));

        let repo = RepoHandle::clone(repo_url, repo_path, auth_agent)?;
        Ok(())
    }
}
