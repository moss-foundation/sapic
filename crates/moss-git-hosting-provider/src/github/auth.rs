use async_trait::async_trait;
use joinerror::Error;
use moss_user::account::auth_gateway_api::{
    GitHubPkceTokenExchangeApiReq, GitHubPkceTokenExchangeResponse, TokenExchangeRequest,
};
use oauth2::CsrfToken;
use serde::Deserialize;
use std::{
    io::{BufRead, BufReader, Write},
    net::TcpListener,
    sync::Arc,
};
use url::Url;

use crate::GitAuthAdapter;

#[derive(Debug, Deserialize)]
pub struct GitHubPkceTokenCredentials {
    pub access_token: String,
}

impl From<GitHubPkceTokenExchangeResponse> for GitHubPkceTokenCredentials {
    fn from(response: GitHubPkceTokenExchangeResponse) -> Self {
        Self {
            access_token: response.access_token,
        }
    }
}

pub struct GitHubAuthAdapter {
    api_client: Arc<dyn GitHubPkceTokenExchangeApiReq>,
    url: Arc<String>,
    callback_port: u16,
}

impl GitHubAuthAdapter {
    pub fn new(
        api_client: Arc<dyn GitHubPkceTokenExchangeApiReq>,
        url: Arc<String>,
        callback_port: u16,
    ) -> Self {
        Self {
            api_client,
            url,
            callback_port,
        }
    }
}

#[async_trait]
impl GitAuthAdapter for GitHubAuthAdapter {
    type PkceToken = GitHubPkceTokenCredentials;
    type PatToken = ();

    async fn auth_with_pkce(&self) -> joinerror::Result<Self::PkceToken> {
        let listener = {
            let addr = format!("127.0.0.1:{}", self.callback_port);
            TcpListener::bind(&addr)
                .map_err(|e| Error::new::<()>(format!("failed to bind to port {}: {}", addr, e)))?
        };

        let state = CsrfToken::new_random();
        let callback_url = format!("http://127.0.0.1:{}/oauth/callback", self.callback_port);
        let auth_url = format!(
            "{}/auth/github/authorize?redirect_uri={}&state={}",
            self.url,
            urlencoding::encode(&callback_url),
            state.secret()
        );

        if webbrowser::open(auth_url.as_str()).is_err() {
            eprintln!("Open this URL:\n{}\n", auth_url);
        }

        let (stream, _) = listener
            .accept()
            .map_err(|e| Error::new::<()>(format!("failed to accept connection: {}", e)))?;

        let mut rdr = BufReader::new(&stream);
        let mut buf = String::new();
        rdr.read_line(&mut buf)
            .map_err(|e| Error::new::<()>(format!("failed to read request: {}", e)))?;

        let url_path = buf
            .split_whitespace()
            .nth(1)
            .ok_or_else(|| Error::new::<()>("invalid HTTP request"))?;

        if !url_path.starts_with("/oauth/callback") {
            return Err(Error::new::<()>(format!(
                "unexpected callback path: {}",
                url_path
            )));
        }

        let url = Url::parse(&format!("http://localhost{}", url_path))
            .map_err(|e| Error::new::<()>(format!("failed to parse URL: {}", e)))?;

        let code = url
            .query_pairs()
            .find(|(key, _)| key == "code")
            .map(|(_, value)| value.to_string())
            .ok_or_else(|| Error::new::<()>("authorization code not found"))?;

        let returned_state = url
            .query_pairs()
            .find(|(key, _)| key == "state")
            .map(|(_, value)| value.to_string())
            .ok_or_else(|| Error::new::<()>("state parameter not found"))?;

        if state.secret() != &returned_state {
            return Err(Error::new::<()>("state mismatch - possible CSRF attack"));
        }

        let response = "HTTP/1.1 200 OK\r\n\r\n<html><body><h1>Authorization successful!</h1><p>You can close this window.</p><script>window.close();</script></body></html>";
        let mut stream = stream;
        stream
            .write_all(response.as_bytes())
            .map_err(|e| Error::new::<()>(format!("failed to send response: {}", e)))?;

        self.api_client
            .github_pkce_token_exchange(TokenExchangeRequest {
                code: code.clone(),
                state: returned_state.clone(),
            })
            .await
            .map(Into::into)
    }

    async fn auth_with_pat(&self) -> joinerror::Result<Self::PatToken> {
        todo!()
    }
}
