use async_trait::async_trait;
use moss_git::GitAuthAgent;
use oauth2::http::{HeaderMap, header::ACCEPT};
use reqwest::Client;
use std::sync::Arc;
use url::Url;

use crate::{
    GitHostingProvider,
    common::SSHAuthAgent,
    constants::GITHUB_API_URL,
    github::response::{ContributorsResponse, RepositoryResponse},
    models::types::{Contributor, RepositoryInfo},
};

pub trait GitHubAuthAgent: GitAuthAgent {}

pub struct GitHubClient {
    client: Client,
    #[allow(dead_code)]
    client_auth_agent: Arc<dyn GitHubAuthAgent>,
    #[allow(dead_code)]
    ssh_auth_agent: Option<Arc<dyn SSHAuthAgent>>,
}

impl GitHubClient {
    pub fn new(
        client: Client,
        client_auth_agent: impl GitHubAuthAgent + 'static,
        ssh_auth_agent: Option<impl SSHAuthAgent + 'static>,
    ) -> Self {
        Self {
            client,
            client_auth_agent: Arc::new(client_auth_agent),
            ssh_auth_agent: ssh_auth_agent.map(|agent| Arc::new(agent) as Arc<dyn SSHAuthAgent>),
        }
    }
}

unsafe impl Send for GitHubClient {}
unsafe impl Sync for GitHubClient {}
#[async_trait]
impl GitHostingProvider for GitHubClient {
    fn name(&self) -> String {
        "GitHub".to_string()
    }

    fn base_url(&self) -> Url {
        Url::parse("https://github.com").unwrap()
    }

    async fn contributors(&self, repo_url: &str) -> anyhow::Result<Vec<Contributor>> {
        // TODO: Support token auth for private repos
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, "application/vnd.github+json".parse()?);

        let contributors_response: ContributorsResponse = self
            .client
            .get(format!("{GITHUB_API_URL}/repos/{repo_url}/contributors"))
            .headers(headers)
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

    async fn repository_info(&self, repo_url: &str) -> anyhow::Result<RepositoryInfo> {
        // TODO: Support token auth for private repo
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, "application/vnd.github+json".parse()?);

        let repo_response: RepositoryResponse = self
            .client
            .get(format!("{GITHUB_API_URL}/repos/{repo_url}"))
            .headers(headers.clone())
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

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use git2::RemoteCallbacks;

    use super::*;

    struct DummyGitHubAuthAgent;

    const REPO_URL: &'static str = "moss-foundation/sapic";

    impl GitAuthAgent for DummyGitHubAuthAgent {
        fn generate_callback<'a>(&'a self, _cb: &mut RemoteCallbacks<'a>) -> Result<()> {
            Ok(())
        }
    }
    impl GitHubAuthAgent for DummyGitHubAuthAgent {}

    struct DummySSHAuthAgent;

    impl GitAuthAgent for DummySSHAuthAgent {
        fn generate_callback<'a>(&'a self, _cb: &mut RemoteCallbacks<'a>) -> Result<()> {
            Ok(())
        }
    }
    impl SSHAuthAgent for DummySSHAuthAgent {}

    #[test]
    fn github_client_name() {
        let client_auth_agent = DummyGitHubAuthAgent;
        let ssh_auth_agent: Option<DummySSHAuthAgent> = None;
        let reqwest_client = reqwest::ClientBuilder::new()
            .user_agent("SAPIC")
            .build()
            .unwrap();

        let client = GitHubClient::new(reqwest_client, client_auth_agent, ssh_auth_agent);

        assert_eq!(client.name(), "GitHub");
    }

    #[test]
    fn github_client_base_url() {
        let client_auth_agent = DummyGitHubAuthAgent;
        let ssh_auth_agent: Option<DummySSHAuthAgent> = None;
        let reqwest_client = reqwest::ClientBuilder::new()
            .user_agent("SAPIC")
            .build()
            .unwrap();

        let client = GitHubClient::new(reqwest_client, client_auth_agent, ssh_auth_agent);
        let expected_url = Url::parse("https://github.com").unwrap();

        assert_eq!(client.base_url(), expected_url);
    }

    #[tokio::test]
    async fn github_client_contributors() {
        let client_auth_agent = DummyGitHubAuthAgent;
        let ssh_auth_agent: Option<DummySSHAuthAgent> = None;
        let reqwest_client = reqwest::ClientBuilder::new()
            .user_agent("SAPIC")
            .build()
            .unwrap();
        let client = GitHubClient::new(reqwest_client, client_auth_agent, ssh_auth_agent);
        let contributors = client.contributors(REPO_URL).await.unwrap();
        for contributor in contributors {
            println!(
                "Contributor {}, avatar_url: {}",
                contributor.name, contributor.avatar_url
            );
        }
    }

    #[tokio::test]
    async fn github_client_repository_info() {
        let client_auth_agent = DummyGitHubAuthAgent;
        let ssh_auth_agent: Option<DummySSHAuthAgent> = None;
        let reqwest_client = reqwest::ClientBuilder::new()
            .user_agent("SAPIC")
            .build()
            .unwrap();
        let client = GitHubClient::new(reqwest_client, client_auth_agent, ssh_auth_agent);
        let repo_info = client.repository_info(REPO_URL).await.unwrap();
        println!("Repository created at {}", repo_info.created_at);
        println!("Repository updated at {}", repo_info.updated_at);
        println!("Repository owner {}", repo_info.owner);
    }

    #[ignore]
    #[test]
    fn manual_github_client_with_ssh_auth_agent() {
        let client_auth_agent = DummyGitHubAuthAgent;
        let ssh_agent = DummySSHAuthAgent;
        let reqwest_client = reqwest::ClientBuilder::new()
            .user_agent("SAPIC")
            .build()
            .unwrap();
        let client = GitHubClient::new(reqwest_client, client_auth_agent, Some(ssh_agent));

        assert_eq!(client.name(), "GitHub");
        let expected_url = Url::parse("https://github.com").unwrap();
        assert_eq!(client.base_url(), expected_url);
    }
}
