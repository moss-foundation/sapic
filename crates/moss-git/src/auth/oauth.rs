// https://datatracker.ietf.org/doc/html/rfc7636#section-1.1
// https://datatracker.ietf.org/doc/html/rfc8252#section-7

use crate::auth::{AuthAgent, TestStorage};
use crate::cred::oauth::OAuthCred;
use anyhow::Result;
use git2::{Cred, RemoteCallbacks};
use oauth2::basic::BasicClient;
use oauth2::url::Url;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl,
    RefreshToken, Scope, TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::time::{Duration, SystemTime};

#[derive(Serialize, Deserialize)]
pub struct OAuthAgent {
    auth_url: AuthUrl,
    token_url: TokenUrl,
    client_id: ClientId,
    client_secret: ClientSecret,
    scopes: Vec<Scope>,
    cred: Option<OAuthCred>,
}

impl OAuthAgent {
    pub fn new(
        auth_url: &str,
        token_url: &str,
        client_id: &str,
        client_secret: &str,
        scopes: Vec<&str>,
        cred: Option<OAuthCred>,
    ) -> OAuthAgent {
        OAuthAgent {
            auth_url: AuthUrl::new(auth_url.to_string()).unwrap(),
            token_url: TokenUrl::new(token_url.to_string()).unwrap(),
            client_id: ClientId::new(client_id.to_string()),
            client_secret: ClientSecret::new(client_secret.to_string()),
            scopes: scopes
                .into_iter()
                .map(|s| Scope::new(s.to_string()))
                .collect(),
            cred,
        }
    }
}

impl OAuthAgent {
    fn initial_auth(&mut self) -> Result<()> {
        println!("Initial OAuth Protocol");
        // Setting the port as 0 automatically assigns a free port
        let listener = TcpListener::bind("127.0.0.1:0")?;
        let callback_port = listener.local_addr()?.port();

        let client = BasicClient::new(self.client_id.clone())
            .set_client_secret(self.client_secret.clone())
            .set_auth_uri(self.auth_url.clone())
            .set_token_uri(self.token_url.clone())
            .set_redirect_uri(RedirectUrl::new(format!(
                "http://127.0.0.1:{}",
                callback_port.to_string()
            ))?);

        // https://datatracker.ietf.org/doc/html/rfc7636#section-1.1
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        // Generate the authorization URL to which we'll redirect the user.
        let (authorize_url, _csrf_state) = client
            .authorize_url(CsrfToken::new_random)
            .add_scopes(self.scopes.clone())
            .set_pkce_challenge(pkce_challenge)
            .url();

        // TODO: Open a browser for this url
        println!("Open this URL in your browser:\n{authorize_url}\n");

        let (code, state) = {
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

        let access_token = token_res.access_token().secret().to_owned();
        let refresh_token = token_res.refresh_token().unwrap().secret().to_owned();

        // Force refreshing the access token half an hour before the actual expiry
        // To avoid any timing issue
        let time_to_refresh = SystemTime::now()
            .checked_add(token_res.expires_in().unwrap())
            .unwrap()
            .checked_sub(Duration::from_secs(30 * 60))
            .unwrap();

        self.cred = Some(OAuthCred::new(
            &access_token,
            time_to_refresh,
            &refresh_token,
        ));

        Ok(())
    }

    fn refresh_token(&mut self) -> Result<()> {
        println!("Refreshing Access Token");
        // Setting the port as 0 automatically assigns a free port
        let listener = TcpListener::bind("127.0.0.1:0")?;
        let callback_port = listener.local_addr()?.port();

        let client = BasicClient::new(self.client_id.clone())
            .set_client_secret(self.client_secret.clone())
            .set_auth_uri(self.auth_url.clone())
            .set_token_uri(self.token_url.clone())
            .set_redirect_uri(RedirectUrl::new(format!(
                "http://127.0.0.1:{}",
                callback_port.to_string()
            ))?);

        let http_client = reqwest::blocking::ClientBuilder::new()
            .redirect(reqwest::redirect::Policy::none())
            .build()?;

        let refresh_token = self.cred.clone().unwrap().refresh_token();

        let token_res = client
            .exchange_refresh_token(&RefreshToken::new(refresh_token.clone()))
            .request(&http_client)?;

        let access_token = token_res.access_token().secret().to_owned();
        // Force refreshing the access token half an hour before the actual expiry
        let time_to_refresh = SystemTime::now()
            .checked_add(token_res.expires_in().unwrap())
            .unwrap()
            .checked_sub(Duration::from_secs(30 * 60))
            .unwrap();

        self.cred = Some(OAuthCred::new(
            &access_token,
            time_to_refresh,
            &refresh_token,
        ));

        Ok(())
    }
}

impl AuthAgent for OAuthAgent {
    fn authorize<'a>(mut self, remote_callbacks: &mut RemoteCallbacks<'a>) -> Result<()> {
        remote_callbacks.credentials(move |_url, _username_from_url, _allowed_types| {
            if self.cred.is_none() {
                self.initial_auth().unwrap();
            }
            let time_to_refresh = self.cred.clone().unwrap().time_to_refresh();
            if SystemTime::now() > time_to_refresh {
                self.refresh_token().unwrap();
            }
            self.write_to_file().unwrap();
            let access_token = self.cred.clone().unwrap().access_token();
            Cred::userpass_plaintext("oauth2", &access_token)
        });

        Ok(())
    }
}

impl TestStorage for OAuthAgent {
    fn write_to_file(&self) -> Result<()> {
        println!("Writing to file");
        std::fs::write("oauth.json", serde_json::to_string(&self)?);
        Ok(())
    }

    fn read_from_file() -> Result<Box<Self>> {
        Ok(Box::new(serde_json::from_str(&std::fs::read_to_string(
            "oauth.json",
        )?)?))
    }
}
