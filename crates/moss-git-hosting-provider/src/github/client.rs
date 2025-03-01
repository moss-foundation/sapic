use moss_git::GitAuthAgent;
use std::sync::Arc;
use url::Url;

use crate::{common::SHHAuthAgent, GitHostingProvider};

pub trait GitHubAuthAgent: GitAuthAgent {}

pub struct GitHubClient {
    client_auth_agent: Arc<dyn GitHubAuthAgent>,
    ssh_auth_agent: Option<Arc<dyn SHHAuthAgent>>,
}

impl GitHubClient {
    pub fn new(
        client_auth_agent: impl GitHubAuthAgent + 'static,
        ssh_auth_agent: Option<impl SHHAuthAgent + 'static>,
    ) -> Self {
        Self {
            client_auth_agent: Arc::new(client_auth_agent),
            ssh_auth_agent: ssh_auth_agent.map(|agent| Arc::new(agent) as Arc<dyn SHHAuthAgent>),
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
