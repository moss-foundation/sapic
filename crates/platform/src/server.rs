mod auth_github_account_api;
mod auth_gitlab_account_api;
mod extensions_api;
mod types;

use reqwest::Client as HttpClient;
use sapic_system::ports::server_api::ServerApiClient;

pub struct HttpServerApiClient {
    base_url: String,
    client: HttpClient,
}

impl HttpServerApiClient {
    pub fn new(base_url: String, client: HttpClient) -> Self {
        Self { base_url, client }
    }
}

impl ServerApiClient for HttpServerApiClient {}
