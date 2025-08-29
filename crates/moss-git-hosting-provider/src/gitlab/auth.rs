use crate::common::utils;
use anyhow::Result;
use git2::{Cred, RemoteCallbacks};
use moss_git::GitAuthAgent;
use moss_keyring::KeyringClient;
use oauth2::{
    AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl, RefreshToken,
    Scope, TokenResponse, TokenUrl, basic::BasicClient,
};
use serde::{Deserialize, Serialize};
use std::{
    net::TcpListener,
    sync::{Arc, RwLock},
    time::{Duration, Instant},
};

const GITLAB_AUTH_URL: &'static str = "https://gitlab.com/oauth/authorize";
const GITLAB_TOKEN_URL: &'static str = "https://gitlab.com/oauth/token";
const GITLAB_SCOPES: [&'static str; 6] = [
    "ai_features",
    "api",
    "read_api",
    "read_repository",
    "write_repository",
    "read_user",
];
const KEYRING_SECRET_KEY: &'static str = "gitlab_auth_agent";

#[derive(Debug, Deserialize, Serialize)]
pub struct KeyringCredEntry {
    refresh_token: String,
}

impl From<&GitLabCred> for KeyringCredEntry {
    fn from(value: &GitLabCred) -> Self {
        Self {
            refresh_token: value.refresh_token.clone(),
        }
    }
}

impl TryInto<String> for KeyringCredEntry {
    type Error = anyhow::Error;

    fn try_into(self) -> std::result::Result<String, Self::Error> {
        Ok(serde_json::to_string(&self)?)
    }
}

#[derive(Debug, Clone)]
pub struct GitLabCred {
    access_token: String,
    time_to_refresh: Instant,
    refresh_token: String,
}

pub struct GitLabAuthAgent {
    // We use ureq instead of blocking reqwest to avoid panicking when called from async environment
    sync_http_client: oauth2::ureq::Agent,
    client_id: ClientId,
    client_secret: ClientSecret,
    keyring: Arc<dyn KeyringClient + Send + Sync>,
    cred: RwLock<Option<GitLabCred>>,
}

impl GitLabAuthAgent {
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
            cred: RwLock::new(None),
        }
    }

    // FIXME: Find a better solution
    // We need a way to provide access_token to git provider client
    // Since the underlying authentication relies on a synchronous `reqwest` client
    // We will need to wrap it inside a tokio::spawn_blocking to avoid panic when called from an async environment
    pub(crate) fn access_token(self: Arc<Self>) -> Option<String> {
        let read = self.cred.read().ok()?;
        let cred = (*read).clone();
        cred.map(|cred| cred.access_token.clone())
    }

    pub fn is_logged_in(&self) -> joinerror::Result<bool> {
        Ok(self.cred.read()?.is_some())
    }
}

// TODO: Add timeout mechanism to handle OAuth failure

impl GitLabAuthAgent {
    pub(crate) fn credentials(&self) -> Result<GitLabCred> {
        if let Some(cached) = self.cred.read().expect("RwLock poisoned").clone() {
            if Instant::now() <= cached.time_to_refresh {
                return Ok(cached);
            }
        }

        // In tests and CI, fetch GITLAB_REFRESH_TOKEN and get new access_token
        if let Some(updated_cred) = self.try_refresh_from_env()? {
            return Ok(updated_cred);
        }

        let gen_initial_cred_fn: Box<dyn Fn() -> Result<GitLabCred>> = Box::new(|| {
            let initial_cred = self.gen_initial_credentials()?;
            let entry_str: String = KeyringCredEntry::from(&initial_cred).try_into()?;
            self.keyring.set_secret(KEYRING_SECRET_KEY, &entry_str)?;

            Ok(initial_cred)
        });

        let updated_cred = match self.keyring.get_secret(KEYRING_SECRET_KEY) {
            Ok(data) => {
                let stored_entry: KeyringCredEntry = serde_json::from_slice(&data)?;
                let refreshed_cred = match self.refresh_token_flow(stored_entry.refresh_token) {
                    Ok(cred) => cred,
                    Err(err) => {
                        // TODO: log her
                        println!("{}", err);

                        gen_initial_cred_fn()?
                    }
                };

                let updated_entry_str: String =
                    KeyringCredEntry::from(&refreshed_cred).try_into()?;
                self.keyring
                    .set_secret(KEYRING_SECRET_KEY, &updated_entry_str)?;

                refreshed_cred
            }
            // Err(keyring::Error::NoEntry) => gen_initial_cred_fn()?,
            Err(err) => return Err(err.into()),
        };

        *self.cred.write().expect("RwLock poisoned") = Some(updated_cred.clone());
        Ok(updated_cred)
    }

    pub(crate) async fn credentials_async(self: Arc<Self>) -> Result<GitLabCred> {
        let self_clone = self.clone();
        tokio::task::spawn_blocking(move || self_clone.credentials()).await?
    }

    // A helper method to avoid false positive about unreachable code
    // It will fetch the refresh token from the environment and generate a new access token
    #[cfg(any(test, feature = "integration-tests"))]
    fn try_refresh_from_env(&self) -> Result<Option<GitLabCred>> {
        dotenv::dotenv().ok();
        let refresh_token = dotenv::var(crate::envvar_keys::GITLAB_REFRESH_TOKEN)?;
        let updated_cred = match self.refresh_token_flow(refresh_token) {
            Ok(cred) => cred,
            Err(err) => {
                return Err(err);
            }
        };

        *self.cred.write().expect("RwLock poisoned") = Some(updated_cred.clone());
        Ok(Some(updated_cred))
    }

    #[cfg(not(any(test, feature = "integration-tests")))]
    fn try_refresh_from_env(&self) -> Result<Option<GitLabCred>> {
        Ok(None)
    }

    fn gen_initial_credentials(&self) -> Result<GitLabCred> {
        let (listener, callback_port) = utils::create_auth_tcp_listener()?;

        let client = BasicClient::new(self.client_id.clone())
            .set_client_secret(self.client_secret.clone())
            .set_auth_uri(AuthUrl::new(GITLAB_AUTH_URL.to_string())?)
            .set_token_uri(TokenUrl::new(GITLAB_TOKEN_URL.to_string())?)
            .set_redirect_uri(RedirectUrl::new(format!(
                "http://127.0.0.1:{}",
                callback_port.to_string()
            ))?);

        // https://datatracker.ietf.org/doc/html/rfc7636#section-1.1
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        // Generate the authorization URL to which we'll redirect the user.
        let (authorize_url, _csrf_state) = client
            .authorize_url(CsrfToken::new_random)
            .add_scopes(GITLAB_SCOPES.into_iter().map(|s| Scope::new(s.to_string())))
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
        let expires_in = token_res.expires_in().ok_or_else(|| {
            anyhow::anyhow!(
                "Failed to perform initial GitLab credentials setup: expires_in value not received"
            )
        })?;

        Ok(GitLabCred {
            access_token: token_res.access_token().secret().to_owned(),
            time_to_refresh: compute_time_to_refresh(expires_in),
            refresh_token: token_res
                .refresh_token()
                .ok_or_else(|| anyhow::anyhow!("Failed to perform initial GitLab credentials setup: refresh token not received"))?
                .secret()
                .to_owned(),
        })
    }

    fn refresh_token_flow(&self, refresh_token: String) -> Result<GitLabCred> {
        let listener = TcpListener::bind("127.0.0.1:0")?; // Setting the port as 0 automatically assigns a free port
        let callback_port = listener.local_addr()?.port();

        let client = BasicClient::new(self.client_id.clone())
            .set_client_secret(self.client_secret.clone())
            .set_auth_uri(AuthUrl::new(GITLAB_AUTH_URL.to_string())?)
            .set_token_uri(TokenUrl::new(GITLAB_TOKEN_URL.to_string())?)
            .set_redirect_uri(RedirectUrl::new(format!(
                "http://127.0.0.1:{}",
                callback_port.to_string()
            ))?);

        let token_res = client
            .exchange_refresh_token(&RefreshToken::new(refresh_token))
            .request(&self.sync_http_client)?;

        let expires_in = token_res.expires_in().ok_or_else(|| {
            anyhow::anyhow!(
                "Failed to perform refresh GitLab credentials operation: expires_in value not received"
            )
        })?;

        Ok(GitLabCred {
            access_token: token_res.access_token().secret().to_owned(),
            time_to_refresh: compute_time_to_refresh(expires_in),
            refresh_token: token_res
                .refresh_token()
                .ok_or_else(|| anyhow::anyhow!("Failed to perform refresh GitLab credentials operation: refresh token not received"))?
                .secret()
                .to_owned(),
        })
    }
}

impl GitAuthAgent for GitLabAuthAgent {
    fn generate_callback<'a>(&'a self, cb: &mut RemoteCallbacks<'a>) -> Result<()> {
        let cred = self.credentials()?;

        cb.credentials(move |_url, _username_from_url, _allowed_types| {
            Cred::userpass_plaintext("oauth2", &cred.access_token)
        });

        Ok(())
    }
}

fn compute_time_to_refresh(expires_in: Duration) -> Instant {
    // Force refreshing the access token half an hour before the actual expiry
    // To avoid any timing issue
    Instant::now()
        .checked_add(expires_in)
        .unwrap()
        .checked_sub(Duration::from_secs(30 * 60))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::{
        envvar_keys::{GITLAB_CLIENT_ID, GITLAB_CLIENT_SECRET},
        gitlab::auth::GitLabAuthAgent,
    };
    use moss_git::repo::RepoHandle;
    use moss_keyring::KeyringClientImpl;
    use std::{path::Path, sync::Arc};

    #[ignore]
    #[test]
    fn manual_cloning_with_oauth() {
        dotenv::dotenv().ok();
        let repo_url = &dotenv::var("GITLAB_TEST_REPO_HTTPS").unwrap();
        let repo_path = Path::new("test-repo-lab");

        let client_id = dotenv::var(GITLAB_CLIENT_ID).unwrap();
        let client_secret = dotenv::var(GITLAB_CLIENT_SECRET).unwrap();

        let keyring_client = Arc::new(KeyringClientImpl::new());
        let auth_agent = Arc::new(GitLabAuthAgent::new(
            oauth2::ureq::builder().redirects(0).build(),
            keyring_client,
            client_id,
            client_secret,
        ));

        let _repo = RepoHandle::clone(&repo_url, repo_path, auth_agent).unwrap();
    }
}
