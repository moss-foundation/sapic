use async_trait::async_trait;
use joinerror::Error;
use moss_server_api::account_auth_gateway::{
    GitLabPkceTokenExchangeApiReq, GitLabPkceTokenExchangeResponse, TokenExchangeRequest,
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
pub struct GitLabPkceTokenCredentials {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
}

impl From<GitLabPkceTokenExchangeResponse> for GitLabPkceTokenCredentials {
    fn from(response: GitLabPkceTokenExchangeResponse) -> Self {
        Self {
            access_token: response.access_token,
            refresh_token: response.refresh_token,
            expires_in: response.expires_in,
        }
    }
}

pub struct GitLabAuthAdapter {
    api_client: Arc<dyn GitLabPkceTokenExchangeApiReq>,
    url: Arc<String>,
    callback_port: u16,
}

impl GitLabAuthAdapter {
    pub fn new(
        api_client: Arc<dyn GitLabPkceTokenExchangeApiReq>,
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
impl GitAuthAdapter for GitLabAuthAdapter {
    type PkceToken = GitLabPkceTokenCredentials;
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
            "{}/auth/gitlab/authorize?redirect_uri={}&state={}",
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
            .map_err(|e| Error::new::<()>(format!("Failed to read request: {}", e)))?;

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
            .ok_or_else(|| Error::new::<()>("state parameter not found"))?;

        let returned_state = url
            .query_pairs()
            .find(|(key, _)| key == "state")
            .map(|(_, value)| value.to_string())
            .ok_or_else(|| Error::new::<()>("State parameter not found"))?;

        if state.secret() != &returned_state {
            return Err(Error::new::<()>("State mismatch - possible CSRF attack"));
        }

        let response = "HTTP/1.1 200 OK\r\n\r\n<html><body><h1>Authorization successful!</h1><p>You can close this window.</p><script>window.close();</script></body></html>";
        let mut stream = stream;
        stream
            .write_all(response.as_bytes())
            .map_err(|e| Error::new::<()>(format!("failed to send response: {}", e)))?;

        self.api_client
            .gitlab_pkce_token_exchange(TokenExchangeRequest {
                code: code.clone(),
                state: returned_state.clone(),
            })
            .await
            .map(Into::into)
    }

    async fn auth_with_pat(&self) -> joinerror::Result<Self::PatToken> {
        unimplemented!()
    }
}
