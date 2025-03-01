use anyhow::anyhow;
use anyhow::Result;
use git2::Cred;
use git2::RemoteCallbacks;
use moss_git::GitAuthAgent;
use moss_keyring::KeyringClient;
use oauth2::basic::BasicClient;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl,
    RefreshToken, Scope, TokenResponse, TokenUrl,
};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::cell::OnceCell;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::sync::Arc;
use std::time::{Duration, Instant};
use url::Url;

#[derive(Debug, Deserialize, Serialize)]
pub struct KeyringCredEntry {
    refresh_token: String,
}

// Since `Instant` is opaque and cannot be serialized
// We will only store the refresh_token when serializing OAuth Credential
// Forcing refreshing of tokens for new sessions
#[derive(Debug, Clone)]
pub struct GitLabCred {
    access_token: String,
    time_to_refresh: Instant,
    refresh_token: String,
}

impl GitLabCred {
    // pub fn new(
    //     access_token: Option<&str>,
    //     time_to_refresh: Option<Instant>,
    //     refresh_token: &str,
    // ) -> Self {
    //     Self {
    //         access_token: access_token.map(|s| s.to_string()),
    //         time_to_refresh,
    //         refresh_token: refresh_token.to_string(),
    //     }
    // }

    // pub fn access_token(&self) -> Option<&str> {
    //     self.access_token.as_deref()
    // }
    // pub fn time_to_refresh(&self) -> Option<Instant> {
    //     self.time_to_refresh
    // }
    // pub fn refresh_token(&self) -> &str {
    //     self.refresh_token.as_str()
    // }
}

const GITLAB_AUTH_URL: &'static str = "https://gitlab.com/oauth/authorize";
const GITLAB_TOKEN_URL: &'static str = "https://gitlab.com/oauth/token";
const GITLAB_SCOPES: [&'static str; 2] = ["write_repository", "read_user"];
const KEYRING_SECRET_KEY: &str = "gitlab_auth_agent";

pub struct GitLabAuthAgentImpl {
    keyring: Arc<dyn KeyringClient>,
    cred: RwLock<Option<GitLabCred>>,
}

impl GitLabAuthAgentImpl {
    pub fn new(keyring: Arc<dyn KeyringClient>) -> Self {
        Self {
            keyring,
            cred: RwLock::new(None),
        }
    }
}

impl GitLabAuthAgentImpl {
    fn client_id() -> Result<ClientId> {
        dotenv::dotenv()?;
        Ok(ClientId::new(dotenv::var("GITLAB_CLIENT_ID")?))
    }
    fn client_secret() -> Result<ClientSecret> {
        dotenv::dotenv()?;
        Ok(ClientSecret::new(dotenv::var("GITLAB_CLIENT_SECRET")?))
    }

    fn credentials(&self) -> Result<GitLabCred> {
        if let Some(existing) = self.cred.read().clone() {
            if Instant::now() <= existing.time_to_refresh {
                return Ok(existing);
            }
        }

        let new_cred = {
            let loaded_cred = match self.keyring.get_secret(KEYRING_SECRET_KEY) {
                Ok(data) => {
                    let entry: KeyringCredEntry = serde_json::from_slice(&data)?;
                    self.refresh_token_flow(entry.refresh_token)?
                }
                Err(keyring::Error::NoEntry) => {
                    let cred = self.gen_initial_credentials()?;
                    let cred_str = serde_json::to_string(&KeyringCredEntry {
                        refresh_token: cred.refresh_token.clone(),
                    })?;
                    self.keyring.set_secret(KEYRING_SECRET_KEY, &cred_str)?;
                    cred
                }
                Err(err) => return Err(err.into()),
            };

            // Always trigger an update to ensure credentials are up to date
            self.refresh_token_flow(loaded_cred.refresh_token)?
        };

        *self.cred.write() = Some(new_cred.clone());
        Ok(new_cred)
    }

    fn gen_initial_credentials(&self) -> Result<GitLabCred> {
        let listener = TcpListener::bind("127.0.0.1:0")?; // Setting the port as 0 automatically assigns a free port
        let callback_port = listener.local_addr()?.port();

        let client = BasicClient::new(GitLabAuthAgentImpl::client_id()?)
            .set_client_secret(GitLabAuthAgentImpl::client_secret()?)
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

        let (code, _state) = {
            let Some(mut stream) = listener.incoming().flatten().next() else {
                panic!("listener terminated without accepting a connection");
            };

            let mut reader = BufReader::new(&stream);
            let mut request_line = String::new();
            reader.read_line(&mut request_line)?;

            // GET /?code=*** HTTP/1.1
            let redirect_url = request_line.split_whitespace().nth(1).unwrap();
            let url = Url::parse(&("http://127.0.0.1".to_string() + redirect_url))?;

            let code = url
                .query_pairs()
                .find(|(key, _)| key == "code")
                .map(|(_, code)| AuthorizationCode::new(code.into_owned()))
                .unwrap();

            let state = url
                .query_pairs()
                .find(|(key, _)| key == "state")
                .map(|(_, state)| CsrfToken::new(state.into_owned()))
                .unwrap();

            // TODO: Once the code is received, the focus should switch back to the main application
            let message = "Go back to your terminal :)";
            let response = format!(
                "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
                message.len(),
                message
            );
            stream.write_all(response.as_bytes())?;

            (code, state)
        };

        let http_client = reqwest::blocking::ClientBuilder::new()
            .redirect(reqwest::redirect::Policy::none())
            .build()?;

        // Exchange the code + PKCE verifier with access & refresh token.
        let token_res = client
            .exchange_code(code)
            .set_pkce_verifier(pkce_verifier)
            .request(&http_client)?;

        let access_token = token_res.access_token().secret().as_str();
        // let mut write = self.cred.write();

        // GitLab's access token expires in 2 hours, so we need refreshing
        let refresh_token = token_res.refresh_token().unwrap().secret().as_str();

        // Force refreshing the access token half an hour before the actual expiry
        // To avoid any timing issue
        let time_to_refresh = Instant::now()
            .checked_add(token_res.expires_in().unwrap())
            .unwrap()
            .checked_sub(Duration::from_secs(30 * 60))
            .unwrap();

        Ok(GitLabCred {
            access_token: access_token.to_string(),
            time_to_refresh,
            refresh_token: refresh_token.to_string(),
        })
    }

    fn refresh_token_flow(&self, refresh_token: String) -> Result<GitLabCred> {
        println!("Refreshing Access Token");
        let listener = TcpListener::bind("127.0.0.1:0")?; // Setting the port as 0 automatically assigns a free port
        let callback_port = listener.local_addr()?.port();

        let client = BasicClient::new(GitLabAuthAgentImpl::client_id()?)
            .set_client_secret(GitLabAuthAgentImpl::client_secret()?)
            .set_auth_uri(AuthUrl::new(GITLAB_AUTH_URL.to_string())?)
            .set_token_uri(TokenUrl::new(GITLAB_TOKEN_URL.to_string())?)
            .set_redirect_uri(RedirectUrl::new(format!(
                "http://127.0.0.1:{}",
                callback_port.to_string()
            ))?);

        let http_client = reqwest::blocking::ClientBuilder::new()
            .redirect(reqwest::redirect::Policy::none())
            .build()?;

        // let refresh_token = (*self.cred.read())
        //     .clone()
        //     .unwrap()
        //     .refresh_token()
        //     .to_string();

        let token_res = client
            .exchange_refresh_token(&RefreshToken::new(refresh_token.clone()))
            .request(&http_client)?;

        let access_token = token_res.access_token().secret().as_str();
        // Force refreshing the access token half an hour before the actual expiry
        let time_to_refresh = Instant::now()
            .checked_add(token_res.expires_in().unwrap())
            .unwrap()
            .checked_sub(Duration::from_secs(30 * 60))
            .unwrap();
        let refresh_token = token_res.refresh_token().unwrap().secret().as_str();

        Ok(GitLabCred {
            access_token: access_token.to_string(),
            time_to_refresh,
            refresh_token: refresh_token.to_string(),
        })
    }
}

impl GitAuthAgent for GitLabAuthAgentImpl {
    fn generate_callback<'a>(&'a self, cb: &mut RemoteCallbacks<'a>) -> Result<()> {
        // if self.cred.read().is_none() {
        //     self.initial_auth()
        //         .expect("Unable to finish initial authentication");
        // }

        let cred = self.credentials()?;

        // let cred = self.cred.read().clone().unwrap();
        // if cred.time_to_refresh().is_none() || Instant::now() > cred.time_to_refresh().unwrap() {
        //     self.refresh_token_flow()?;
        // }
        // let access_token = self
        //     .cred
        //     .read()
        //     .clone()
        //     .unwrap()
        //     .access_token()
        //     .unwrap()
        //     .to_string();

        cb.credentials(move |_url, _username_from_url, _allowed_types| {
            Cred::userpass_plaintext("oauth2", &cred.access_token)
        });

        // self.write_to_file();
        Ok(())
    }
}

// impl TestStorage for GitLabAuthAgentImpl {
//     fn write_to_file(&self) -> Result<()> {
//         println!("Writing to file");
//         std::fs::write("gitlab_oauth.json", serde_json::to_string(&self)?)?;
//         Ok(())
//     }

//     fn read_from_file() -> Result<Arc<Self>> {
//         dbg!("-----------");
//         dbg!(&std::fs::read_to_string("gitlab_oauth.json",)?);

//         Ok(Arc::new(serde_json::from_str(&std::fs::read_to_string(
//             "gitlab_oauth.json",
//         )?)?))
//     }
// }

#[cfg(test)]
mod gitlab_tests {
    use std::path::Path;
    use std::sync::Arc;

    use crate::gitlab::auth::GitLabAuthAgentImpl;
    use moss_git::repo::RepoHandle;
    use moss_keyring::KeyringClientImpl;
    // use moss_git::TestStorage;

    #[test]
    fn cloning_with_oauth() {
        dotenv::dotenv().ok();
        let repo_url = &dotenv::var("GITLAB_TEST_REPO_HTTPS").unwrap();
        let repo_path = Path::new("test-repo-lab");

        let keyring_client = Arc::new(KeyringClientImpl::new());
        let auth_agent = Arc::new(GitLabAuthAgentImpl::new(keyring_client));

        let repo = RepoHandle::clone(repo_url, repo_path, auth_agent).unwrap();
    }
}
