use crate::{
    GitAuthProvider, GitHostingProvider,
    common::SSHAuthAgent,
    constants::GITHUB_API_URL,
    github::{
        auth::GitHubAuthAgent,
        response::{ContributorsResponse, RepositoryResponse, UserResponse},
    },
    models::types::{Contributor, RepositoryInfo, UserInfo},
};
use async_trait::async_trait;
use joinerror::OptionExt;
use moss_git::GitAuthAgent;
use oauth2::http::header::ACCEPT;
use reqwest::{Client, header::AUTHORIZATION};
use std::sync::Arc;
use url::Url;

const CONTENT_TYPE: &'static str = "application/vnd.github+json";

pub struct GitHubClient {
    client: Client,
    #[allow(dead_code)]
    // TODO: Support multiple accounts?
    client_auth_agent: Arc<GitHubAuthAgent>,
    #[allow(dead_code)]
    ssh_auth_agent: Option<Arc<dyn SSHAuthAgent>>,
}

impl GitHubClient {
    pub fn new(
        client: Client,
        client_auth_agent: Arc<GitHubAuthAgent>,
        ssh_auth_agent: Option<impl SSHAuthAgent + 'static>,
    ) -> Self {
        Self {
            client,
            client_auth_agent,
            ssh_auth_agent: ssh_auth_agent.map(|agent| Arc::new(agent) as Arc<dyn SSHAuthAgent>),
        }
    }

    pub fn is_logged_in(&self) -> joinerror::Result<bool> {
        self.client_auth_agent.is_logged_in()
    }

    // Try to fetch/generate credentials and return currently logged-in user info
    // This will trigger an initial OAuth authorization
    // Or will fetch the stored access_token
    pub async fn login(&self) -> joinerror::Result<UserInfo> {
        let _ = self.client_auth_agent.clone().credentials_async().await?;
        self.current_user().await
    }
}

unsafe impl Send for GitHubClient {}
unsafe impl Sync for GitHubClient {}

impl GitAuthProvider for GitHubClient {
    fn git_auth_agent(&self) -> Arc<dyn GitAuthAgent> {
        self.client_auth_agent.clone()
    }
}

// TODO: Handle authentication expiration and reauthentication
#[async_trait]
impl GitHostingProvider for GitHubClient {
    fn name(&self) -> String {
        "GitHub".to_string()
    }

    fn base_url(&self) -> Url {
        Url::parse("https://github.com").unwrap()
    }

    async fn current_user(&self) -> joinerror::Result<UserInfo> {
        let access_token = self
            .client_auth_agent
            .clone()
            .access_token()
            .ok_or_join_err::<()>("github is not logged in yet")?;

        let user_response: UserResponse = self
            .client
            .get(format!("{GITHUB_API_URL}/user"))
            .header(ACCEPT, CONTENT_TYPE)
            .header(AUTHORIZATION, format!("Bearer {}", access_token))
            .send()
            .await?
            .json()
            .await?;

        // If the user's email is private, we will use their noreply email
        let email = user_response.email.unwrap_or(format!(
            "{}+{}@users.noreply.github.com",
            user_response.id, user_response.login
        ));

        Ok(UserInfo {
            name: user_response.login,
            email,
        })
    }

    async fn contributors(&self, repo_url: &str) -> joinerror::Result<Vec<Contributor>> {
        let access_token = self
            .client_auth_agent
            .clone()
            .access_token()
            .ok_or_join_err::<()>("github is not logged in yet")?;

        let contributors_response: ContributorsResponse = self
            .client
            .get(format!("{GITHUB_API_URL}/repos/{repo_url}/contributors"))
            .header(ACCEPT, CONTENT_TYPE)
            .header(AUTHORIZATION, format!("Bearer {}", access_token))
            .send()
            .await?
            .json()
            .await?;

        Ok(contributors_response
            .items
            .into_iter()
            .map(|item| Contributor {
                name: item.login,
                avatar_url: item.avatar_url,
            })
            .collect())
    }

    async fn repository_info(&self, repo_url: &str) -> joinerror::Result<RepositoryInfo> {
        let access_token = self
            .client_auth_agent
            .clone()
            .access_token()
            .ok_or_join_err::<()>("github is not logged in yet")?;

        let repo_response: RepositoryResponse = self
            .client
            .get(format!("{GITHUB_API_URL}/repos/{repo_url}"))
            .header(ACCEPT, CONTENT_TYPE)
            .header(AUTHORIZATION, format!("Bearer {}", access_token))
            .send()
            .await?
            .json()
            .await?;

        Ok(RepositoryInfo {
            created_at: repo_response.created_at,
            updated_at: repo_response.updated_at,
            owner: repo_response.owner.login,
        })
    }
}
