use anyhow::{Context, anyhow};
use async_trait::async_trait;
use moss_git::AuthProvider;
use moss_user::{AccessToken, TokenType};
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl,
    RefreshToken, Scope, TokenResponse, TokenUrl, basic::BasicClient, ureq,
};
use reqwest::Client as HttpClient;
use std::time::{Duration, SystemTime};

use crate::{
    common::utils::{create_auth_tcp_listener, receive_auth_code},
    github::response::GetUserResponse,
};

pub mod auth;
pub mod client;
mod response;

pub struct GitHubApiClient {
    client: HttpClient,
}

impl GitHubApiClient {
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }

    pub async fn user(&self) -> joinerror::Result<GetUserResponse> {
        todo!()
    }
}

fn authorize_url(host: &str) -> String {
    format!("https://{host}/login/oauth/authorize")
}

fn token_url(host: &str) -> String {
    format!("https://{host}/login/oauth/access_token")
}

// const AUTH_URL: &str = "https://github.com/login/oauth/authorize";
// const TOKEN_URL: &str = "https://github.com/login/oauth/access_token";
const GITHUB_SCOPES: [&'static str; 3] = ["repo", "user:email", "read:user"];
const KEYRING_SECRET_KEY: &str = "github_auth_agent";

type Host = String;
pub struct GithubAuthProvider {}

impl GithubAuthProvider {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl AuthProvider for GithubAuthProvider {
    async fn login_pkce(
        &self,
        client_id: &str,
        client_secret: Option<&str>,
        host: &str,
        scopes: &[&str],
    ) -> anyhow::Result<AccessToken> {
        let (listener, port) = create_auth_tcp_listener()?;
        let redirect = format!("http://127.0.0.1:{port}/callback");

        let mut client = BasicClient::new(ClientId::new(client_id.to_string()))
            .set_auth_uri(AuthUrl::new(authorize_url(host))?)
            .set_token_uri(TokenUrl::new(token_url(host))?)
            .set_redirect_uri(RedirectUrl::new(redirect.clone())?);

        if let Some(cs) = client_secret {
            client = client.set_client_secret(ClientSecret::new(cs.to_string()));
        }

        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
        let (auth_url, state) = client
            .authorize_url(CsrfToken::new_random)
            .add_scopes(scopes.iter().map(|s| Scope::new((*s).to_string())))
            .add_extra_param("prompt", "select_account")
            .set_pkce_challenge(pkce_challenge)
            .url();

        if webbrowser::open(auth_url.as_str()).is_err() {
            eprintln!("Open this URL:\n{}\n", auth_url);
        }

        let (code, returned_state) =
            receive_auth_code(&listener).context("failed to receive OAuth callback")?;
        if state.secret() != returned_state.secret() {
            return Err(anyhow!("state mismatch"));
        }

        let token = client
            .exchange_code(AuthorizationCode::new(code.secret().to_string()))
            .set_pkce_verifier(pkce_verifier)
            .request_async(&reqwest::Client::new()) // TODO: reuse client instead of creating a new one
            .await?;

        let access_token = AccessToken {
            token: token.access_token().secret().to_string(),
            token_type: TokenType::OAuth,
            expires_at: token
                .expires_in()
                .map(|sec| SystemTime::now() + Duration::from_secs(sec.as_secs())),
            refresh_token: token.refresh_token().map(|r| r.secret().to_string()),
            scopes: token
                .scopes()
                .map(|s| s.iter().map(|sc| sc.to_string()).collect())
                .unwrap_or_default(),
        };

        Ok(access_token)
    }

    async fn login_pat(&self) -> joinerror::Result<()> {
        todo!()
    }

    async fn refresh(&self) -> joinerror::Result<()> {
        todo!()
    }
}
