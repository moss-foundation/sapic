use moss_git::url::GitUrl;
use moss_user::AccountSession;
use oauth2::http::header::{ACCEPT, AUTHORIZATION, USER_AGENT};
use reqwest::{Client as HttpClient, RequestBuilder};

use crate::github::response::{GetContributorsResponse, GetRepositoryResponse, GetUserResponse};

const GITHUB_API_URL: &'static str = "https://api.github.com";

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
