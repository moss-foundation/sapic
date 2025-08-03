use anyhow::anyhow;
use async_trait::async_trait;
use moss_git::GitAuthAgent;
use reqwest::header::{ACCEPT, HeaderMap};
use std::sync::Arc;
use url::Url;

use crate::{
    GitHostingProvider,
    common::SSHAuthAgent,
    constants::GITLAB_API_URL,
    models::types::{Contributor, RepositoryInfo},
};

pub trait GitLabAuthAgent: GitAuthAgent {}

pub struct GitLabClient {
    #[allow(dead_code)]
    client_auth_agent: Arc<dyn GitAuthAgent>,
    #[allow(dead_code)]
    ssh_auth_agent: Option<Arc<dyn SSHAuthAgent>>,
}

impl GitLabClient {
    pub fn new(
        client_auth_agent: impl GitLabAuthAgent + 'static,
        ssh_auth_agent: Option<impl SSHAuthAgent + 'static>,
    ) -> Self {
        Self {
            client_auth_agent: Arc::new(client_auth_agent),
            ssh_auth_agent: ssh_auth_agent.map(|agent| Arc::new(agent) as Arc<dyn SSHAuthAgent>),
        }
    }
}

unsafe impl Send for GitLabClient {}
unsafe impl Sync for GitLabClient {}

#[async_trait]
impl GitHostingProvider for GitLabClient {
    fn name(&self) -> String {
        "GitLab".to_string()
    }
    fn base_url(&self) -> Url {
        Url::parse("https://gitlab.com").unwrap()
    }

    async fn contributors(&self, repo_url: &str) -> anyhow::Result<Vec<Contributor>> {
        // TODO: Support token auth for private repos
        let client = reqwest::ClientBuilder::new().user_agent("SAPIC").build()?;
        let encoded_url = urlencoding::encode(repo_url);

        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, "application/json".parse()?);

        let contributors_response: serde_json::Value = client
            .get(format!(
                "{GITLAB_API_URL}/projects/{encoded_url}/repository/contributors"
            ))
            .headers(headers.clone())
            .send()
            .await?
            .json()
            .await?;

        dbg!(&contributors_response);

        // FIXME: Is there a better strategy?
        // Gitlab contributor endpoint only provides the contributor email
        // We will need to fetch their avatar separately

        let mut list = Vec::new();
        for contributor in contributors_response
            .as_array()
            .ok_or(anyhow!("failed to get contributor array"))?
        {
            let name = contributor
                .get("name")
                .and_then(|name| name.as_str())
                .ok_or(anyhow!("failed to get contributor name"))?
                .to_string();

            let email = contributor
                .get("email")
                .and_then(|name| name.as_str())
                .ok_or(anyhow!("failed to get contributor email"))?
                .to_string();

            let avatar_response: serde_json::Value = client
                .get(format!("{GITLAB_API_URL}/avatar"))
                .query(&[("email", &email)])
                .send()
                .await?
                .json()
                .await?;

            let avatar_url = avatar_response
                .get("avatar_url")
                .and_then(|avatar_url| avatar_url.as_str())
                .ok_or(anyhow!("failed to get avatar url"))?
                .to_string();

            list.push(Contributor { name, avatar_url });
        }
        Ok(list)
    }

    async fn repository_info(&self, repo_url: &str) -> anyhow::Result<RepositoryInfo> {
        // TODO: Looks like we can't get `updated_at` without authenticating the API call first
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use git2::RemoteCallbacks;

    use super::*;

    const REPO_URL: &'static str = "brutusyhy/test-public-repo";

    struct DummyGitLabAuthAgent;

    impl GitAuthAgent for DummyGitLabAuthAgent {
        fn generate_callback<'a>(&'a self, _cb: &mut RemoteCallbacks<'a>) -> Result<()> {
            Ok(())
        }
    }
    impl GitLabAuthAgent for DummyGitLabAuthAgent {}

    struct DummySSHAuthAgent;

    impl GitAuthAgent for DummySSHAuthAgent {
        fn generate_callback<'a>(&'a self, _cb: &mut RemoteCallbacks<'a>) -> Result<()> {
            Ok(())
        }
    }
    impl SSHAuthAgent for DummySSHAuthAgent {}

    #[test]
    fn gitlab_client_name() {
        let client_auth_agent = DummyGitLabAuthAgent;
        let ssh_auth_agent: Option<DummySSHAuthAgent> = None;
        let client = GitLabClient::new(client_auth_agent, ssh_auth_agent);

        assert_eq!(client.name(), "GitLab");
    }

    #[test]
    fn gitlab_client_base_url() {
        let client_auth_agent = DummyGitLabAuthAgent;
        let ssh_auth_agent: Option<DummySSHAuthAgent> = None;
        let client = GitLabClient::new(client_auth_agent, ssh_auth_agent);
        let expected_url = Url::parse("https://gitlab.com").unwrap();

        assert_eq!(client.base_url(), expected_url);
    }

    #[tokio::test]
    async fn gitlab_client_contributors() {
        let client_auth_agent = DummyGitLabAuthAgent;
        let ssh_auth_agent: Option<DummySSHAuthAgent> = None;
        let client = GitLabClient::new(client_auth_agent, ssh_auth_agent);
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
    fn manual_gitlab_client_with_ssh_auth_agent() {
        let client_auth_agent = DummyGitLabAuthAgent;
        let ssh_agent = DummySSHAuthAgent;
        let client = GitLabClient::new(client_auth_agent, Some(ssh_agent));

        assert_eq!(client.name(), "GitLab");
        let expected_url = Url::parse("https://gitlab.com").unwrap();
        assert_eq!(client.base_url(), expected_url);
    }
}
