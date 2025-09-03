use async_trait::async_trait;
use joinerror::{Error, ResultExt};
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EmptyExtraTokenFields,
    PkceCodeChallenge, RedirectUrl, Scope, StandardTokenResponse, TokenUrl,
    basic::{BasicClient, BasicTokenType},
};
use reqwest::Client as HttpClient;

use crate::{
    GitAuthAdapter,
    utils::{create_auth_tcp_listener, receive_auth_code},
};

const GITLAB_SCOPES: [&'static str; 4] =
    ["api", "read_user", "read_repository", "write_repository"];

fn authorize_url(host: &str) -> String {
    format!("https://{host}/oauth/authorize")
}

fn token_url(host: &str) -> String {
    format!("https://{host}/oauth/token")
}

pub struct GitLabAuthAdapter {
    client: HttpClient,
}

impl GitLabAuthAdapter {
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }
}

#[async_trait]
impl GitAuthAdapter for GitLabAuthAdapter {
    type PkceToken = StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>;
    type PatToken = ();

    async fn auth_with_pkce(
        &self,
        client_id: ClientId,
        client_secret: ClientSecret,
        host: &str,
    ) -> joinerror::Result<Self::PkceToken> {
        let (listener, port) = create_auth_tcp_listener()?;

        let client = BasicClient::new(client_id)
            .set_client_secret(client_secret)
            .set_auth_uri(AuthUrl::new(authorize_url(host))?)
            .set_token_uri(TokenUrl::new(token_url(host))?)
            .set_redirect_uri(RedirectUrl::new(format!(
                "http://127.0.0.1:{}",
                port.to_string()
            ))?);

        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
        let (auth_url, state) = client
            .authorize_url(CsrfToken::new_random)
            .add_scopes(GITLAB_SCOPES.into_iter().map(|s| Scope::new(s.to_string())))
            .set_pkce_challenge(pkce_challenge)
            .url();

        if webbrowser::open(auth_url.as_str()).is_err() {
            eprintln!("Open this URL:\n{}\n", auth_url);
        }

        let (code, returned_state) =
            receive_auth_code(&listener).join_err::<()>("failed to receive OAuth callback")?;
        if state.secret() != returned_state.secret() {
            return Err(Error::new::<()>("state mismatch"));
        }

        let token = client
            .exchange_code(AuthorizationCode::new(code.secret().to_string()))
            .set_pkce_verifier(pkce_verifier)
            .request_async(&self.client)
            .await
            .map_err(|e| Error::new::<()>(e.to_string()))
            .join_err::<()>("failed to exchange code")?;

        Ok(token)
    }

    async fn auth_with_pat(&self) -> joinerror::Result<Self::PatToken> {
        unimplemented!()
    }
}
