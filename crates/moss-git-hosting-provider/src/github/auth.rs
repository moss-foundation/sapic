use async_trait::async_trait;
use joinerror::Error;
use oauth2::{
    AccessToken, ClientId, ClientSecret, CsrfToken, EmptyExtraTokenFields, StandardTokenResponse,
    basic::BasicTokenType,
};
use reqwest::Client as HttpClient;
use serde::{Deserialize, Serialize};
use std::{
    io::{BufRead, BufReader, Write},
    net::TcpListener,
};
use url::Url;

use crate::GitAuthAdapter;

#[derive(Debug, Deserialize)]
pub struct WorkerAuthResponse {
    pub access_token: String,
    // pub token_type: String,
    // pub scope: String,
}

#[derive(Debug, Serialize)]
pub struct TokenExchangeRequest {
    pub code: String,
    pub state: String,
}

pub struct GitHubAuthAdapter {
    client: HttpClient,
    url: String,
    callback_port: u16,
}

impl GitHubAuthAdapter {
    pub fn new(client: HttpClient, url: String, callback_port: u16) -> Self {
        Self {
            client,
            url,
            callback_port,
        }
    }
}

#[async_trait]
impl GitAuthAdapter for GitHubAuthAdapter {
    type PkceToken = StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>;
    type PatToken = ();

    async fn auth_with_pkce(
        &self,
        _client_id: ClientId,
        _client_secret: ClientSecret,
        _host: &str,
    ) -> joinerror::Result<Self::PkceToken> {
        let listener = {
            let addr = format!("127.0.0.1:{}", self.callback_port);
            TcpListener::bind(&addr)
                .map_err(|e| Error::new::<()>(format!("failed to bind to port {}: {}", addr, e)))?
        };

        let state = CsrfToken::new_random();
        let redirect = format!("http://127.0.0.1:{}/oauth/callback", self.callback_port);
        let auth_url = format!(
            "{}/auth/github/authorize?redirect_uri={}&state={}",
            self.url,
            urlencoding::encode(&redirect),
            state.secret()
        );

        if webbrowser::open(auth_url.as_str()).is_err() {
            eprintln!("Open this URL:\n{}\n", auth_url);
        }

        let (stream, _) = listener
            .accept()
            .map_err(|e| Error::new::<()>(format!("failed to accept connection: {}", e)))?;

        let mut reader = BufReader::new(&stream);
        let mut request_line = String::new();
        reader
            .read_line(&mut request_line)
            .map_err(|e| Error::new::<()>(format!("failed to read request: {}", e)))?;

        let url_path = request_line
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

        let token_exchange_url = format!("{}/auth/github/token", self.url);
        let request_body = TokenExchangeRequest {
            code: code.clone(),
            state: returned_state.clone(),
        };

        let response = self
            .client
            .post(&token_exchange_url)
            .json(&request_body)
            .send()
            .await
            .map_err(|e| Error::new::<()>(format!("failed to exchange token: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "unknown error".to_string());
            return Err(Error::new::<()>(format!(
                "Token exchange failed: {}",
                error_text
            )));
        }

        let worker_response: WorkerAuthResponse = response
            .json()
            .await
            .map_err(|e| Error::new::<()>(format!("failed to parse worker response: {}", e)))?;

        let token_response = StandardTokenResponse::new(
            AccessToken::new(worker_response.access_token),
            BasicTokenType::Bearer,
            EmptyExtraTokenFields {},
        );

        Ok(token_response)
    }

    async fn auth_with_pat(&self) -> joinerror::Result<Self::PatToken> {
        todo!()
    }
}
