use moss_git::GitAuthAgent;
use std::sync::Arc;
use url::Url;

use crate::{common::SSHAuthAgent, GitHostingProvider};

pub trait GitLabAuthAgent: GitAuthAgent {}

pub struct GitLabClient {
    client_auth_agent: Arc<dyn GitAuthAgent>,
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

impl GitHostingProvider for GitLabClient {
    fn name(&self) -> String {
        "GitLab".to_string()
    }
    fn base_url(&self) -> Url {
        Url::parse("https://gitlab.com").unwrap()
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use git2::RemoteCallbacks;

    use super::*;

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

    #[test]
    #[ignore]
    fn manual_gitlab_client_with_ssh_auth_agent() {
        let client_auth_agent = DummyGitLabAuthAgent;
        let ssh_agent = DummySSHAuthAgent;
        let client = GitLabClient::new(client_auth_agent, Some(ssh_agent));

        assert_eq!(client.name(), "GitLab");
        let expected_url = Url::parse("https://gitlab.com").unwrap();
        assert_eq!(client.base_url(), expected_url);
    }
}
