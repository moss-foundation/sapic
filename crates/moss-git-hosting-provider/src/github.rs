use anyhow::{Context, anyhow};
use async_trait::async_trait;
use moss_git::GitAuthAdapter;
use moss_user::AccountSession;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EmptyExtraTokenFields,
    PkceCodeChallenge, RedirectUrl, Scope, StandardTokenResponse, TokenUrl,
    basic::{BasicClient, BasicTokenType},
    http::header::{ACCEPT, AUTHORIZATION, USER_AGENT},
};
use reqwest::{Client as HttpClient, RequestBuilder};

use crate::{
    common::{
        GitUrl,
        utils::{create_auth_tcp_listener, receive_auth_code},
    },
    github::response::{GetContributorsResponse, GetRepositoryResponse, GetUserResponse},
};

pub mod auth;
pub mod client;
mod response;

trait GitHubHttpRequestBuilderExt {
    fn with_default_github_headers(self, access_token: String) -> Self;
}

impl GitHubHttpRequestBuilderExt for RequestBuilder {
    fn with_default_github_headers(self, access_token: String) -> Self {
        self.header(ACCEPT, "application/vnd.github+json")
            .header(USER_AGENT, "SAPIC/1.0")
            .header(AUTHORIZATION, format!("token {}", access_token))
    }
}

// TODO: add context to the client operations

#[derive(Clone)]
pub struct GitHubApiClient {
    client: HttpClient,
}

impl GitHubApiClient {
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }

    // TODO: refactor with constants and helpers
    pub async fn get_user(
        &self,
        account_handle: &AccountSession,
    ) -> joinerror::Result<GetUserResponse> {
        let access_token = account_handle.access_token().await?;
        let resp = self
            .client
            .get(format!("{GITHUB_API_URL}/user"))
            .with_default_github_headers(access_token)
            .send()
            .await?;

        let status = resp.status();
        if status.is_success() {
            Ok(resp.json().await?)
        } else {
            let error_text = resp.text().await?;
            eprintln!("GitHub API Error: Status {}, Body: {}", status, error_text);
            Err(joinerror::Error::new::<()>(error_text))
        }
    }

    // TODO: refactor with constants and helpers
    pub async fn get_contributors(
        &self,
        account_handle: &AccountSession,
        url: &GitUrl,
    ) -> joinerror::Result<GetContributorsResponse> {
        let access_token = account_handle.access_token().await?;
        let repo_url = format!("{}/{}", &url.owner, &url.name);
        let resp = self
            .client
            .get(format!("{GITHUB_API_URL}/repos/{repo_url}/contributors"))
            .with_default_github_headers(access_token)
            .send()
            .await?;

        let status = resp.status();
        if status.is_success() {
            Ok(resp.json().await?)
        } else {
            let error_text = resp.text().await?;
            eprintln!("GitHub API Error: Status {}, Body: {}", status, error_text);
            Err(joinerror::Error::new::<()>(error_text))
        }
    }

    // TODO: refactor with constants and helpers
    pub async fn get_repository(
        &self,
        account_handle: &AccountSession,
        url: &GitUrl,
    ) -> joinerror::Result<GetRepositoryResponse> {
        let access_token = account_handle.access_token().await?;
        let repo_url = format!("{}/{}", &url.owner, &url.name);
        let resp = self
            .client
            .get(format!("{GITHUB_API_URL}/repos/{repo_url}"))
            .with_default_github_headers(access_token)
            .send()
            .await?;

        let status = resp.status();
        if status.is_success() {
            Ok(resp.json().await?)
        } else {
            let error_text = resp.text().await?;
            eprintln!("GitHub API Error: Status {}, Body: {}", status, error_text);
            Err(joinerror::Error::new::<()>(error_text))
        }
    }
}

fn authorize_url(host: &str) -> String {
    format!("https://{host}/login/oauth/authorize")
}

fn token_url(host: &str) -> String {
    format!("https://{host}/login/oauth/access_token")
}

const GITHUB_API_URL: &'static str = "https://api.github.com";
const GITHUB_SCOPES: [&'static str; 3] = ["repo", "user:email", "read:user"];

pub struct GitHubAuthAdapter {}

impl GitHubAuthAdapter {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl GitAuthAdapter for GitHubAuthAdapter {
    type PkceToken = StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>;
    type PatToken = ();

    async fn auth_with_pkce(
        &self,
        client_id: ClientId,
        client_secret: ClientSecret,
        host: &str,
    ) -> anyhow::Result<Self::PkceToken> {
        let (listener, port) = create_auth_tcp_listener()?;
        let redirect = format!("http://127.0.0.1:{port}/callback");

        let client = BasicClient::new(client_id)
            .set_client_secret(client_secret)
            .set_auth_uri(AuthUrl::new(authorize_url(host))?)
            .set_token_uri(TokenUrl::new(token_url(host))?)
            .set_redirect_uri(RedirectUrl::new(redirect.clone())?);

        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
        let (auth_url, state) = client
            .authorize_url(CsrfToken::new_random)
            .add_scopes(GITHUB_SCOPES.iter().map(|s| Scope::new((*s).to_string())))
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

        Ok(token)
    }

    async fn auth_with_pat(&self) -> joinerror::Result<Self::PatToken> {
        todo!()
    }
}
