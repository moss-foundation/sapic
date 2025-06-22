use moss_git::GitAuthAgent;
use std::sync::Arc;
use url::Url;

use crate::{GitHostingProvider, common::SSHAuthAgent};

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

impl GitHostingProvider for GitHubClient {
    fn name(&self) -> String {
        "GitHub".to_string()
    }

    fn base_url(&self) -> Url {
        Url::parse("https://github.com").unwrap()
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use git2::RemoteCallbacks;

    use super::*;

    struct DummyGitHubAuthAgent;

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
