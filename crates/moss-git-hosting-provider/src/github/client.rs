use anyhow::anyhow;
use async_trait::async_trait;
use moss_git::GitAuthAgent;
use oauth2::http::{HeaderMap, header::ACCEPT};
use std::sync::Arc;
use url::Url;

use crate::{
    GitHostingProvider,
    common::SSHAuthAgent,
    constants::GITHUB_API_URL,
    models::types::{Contributor, RepositoryInfo},
};

pub trait GitHubAuthAgent: GitAuthAgent {}

pub struct GitHubClient {
    #[allow(dead_code)]
    client_auth_agent: Arc<dyn GitHubAuthAgent>,
    #[allow(dead_code)]
    ssh_auth_agent: Option<Arc<dyn SSHAuthAgent>>,
}

impl GitHubClient {
    pub fn new(
        client_auth_agent: impl GitHubAuthAgent + 'static,
        ssh_auth_agent: Option<impl SSHAuthAgent + 'static>,
    ) -> Self {
        Self {
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
        let client = reqwest::ClientBuilder::new().user_agent("SAPIC").build()?;

        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, "application/vnd.github+json".parse()?);

        let contributors_response: serde_json::Value = client
            .get(format!("{GITHUB_API_URL}/repos/{repo_url}/contributors"))
            .headers(headers.clone())
            .send()
            .await?
            .json()
            .await?;

        let mut list = Vec::new();

        for contributor in contributors_response
            .as_array()
            .ok_or(anyhow!("failed to get contributor array"))?
        {
            let name = contributor
                .get("login")
                .and_then(|name| name.as_str())
                .ok_or(anyhow!("failed to get contributor name"))?
                .to_string();
            let avatar_url = contributor
                .get("avatar_url")
                .and_then(|url| url.as_str())
                .ok_or(anyhow!("failed to get contributor avatar url"))?
                .to_string();

            list.push(Contributor { name, avatar_url });
        }
        Ok(list)
    }

    async fn repository_info(&self, repo_url: &str) -> anyhow::Result<RepositoryInfo> {
        // TODO: Support token auth for private repos
        let client = reqwest::ClientBuilder::new().user_agent("SAPIC").build()?;

        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, "application/vnd.github+json".parse()?);

        let repo_response: serde_json::Value = client
            .get(format!("{GITHUB_API_URL}/repos/{repo_url}"))
            .headers(headers.clone())
            .send()
            .await?
            .json()
            .await?;

        let created_at = repo_response
            .get("created_at")
            .and_then(|time| time.as_str())
            .ok_or(anyhow!("failed to get repository created_at timestamp"))?
            .to_string();
        let updated_at = repo_response
            .get("updated_at")
            .and_then(|time| time.as_str())
            .ok_or(anyhow!("failed to get repository updated_at timestamp"))?
            .to_string();
        let owner = repo_response
            .get("owner")
            .and_then(|owner| owner.get("login"))
            .and_then(|name| name.as_str())
            .ok_or(anyhow!("failed to get repository owner name"))?
            .to_string();

        Ok(RepositoryInfo {
            created_at,
            updated_at,
            owner,
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
        let client = GitHubClient::new(client_auth_agent, ssh_auth_agent);

        assert_eq!(client.name(), "GitHub");
    }

    #[test]
    fn github_client_base_url() {
        let client_auth_agent = DummyGitHubAuthAgent;
        let ssh_auth_agent: Option<DummySSHAuthAgent> = None;
        let client = GitHubClient::new(client_auth_agent, ssh_auth_agent);
        let expected_url = Url::parse("https://github.com").unwrap();

        assert_eq!(client.base_url(), expected_url);
    }

    #[tokio::test]
    async fn github_client_contributors() {
        let client_auth_agent = DummyGitHubAuthAgent;
        let ssh_auth_agent: Option<DummySSHAuthAgent> = None;
        let client = GitHubClient::new(client_auth_agent, ssh_auth_agent);
        let contributors = client.contributors(REPO_URL).await.unwrap();
        for contributor in contributors {
            println!(
                "Contributor {}, avatar_url: {}",
                contributor.name, contributor.avatar_url
            );
        }
    }

    #[ignore]
    #[test]
    fn manual_github_client_with_ssh_auth_agent() {
        let client_auth_agent = DummyGitHubAuthAgent;
        let ssh_agent = DummySSHAuthAgent;
        let client = GitHubClient::new(client_auth_agent, Some(ssh_agent));

        assert_eq!(client.name(), "GitHub");
        let expected_url = Url::parse("https://github.com").unwrap();
        assert_eq!(client.base_url(), expected_url);
    }
}
