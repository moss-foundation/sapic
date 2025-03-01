use moss_git::GitAuthAgent;
use std::sync::Arc;
use url::Url;

use crate::{common::SHHAuthAgent, GitHostingProvider};

pub trait GitLabAuthAgent: GitAuthAgent {}

pub struct GitLabClient {
    client_auth_agent: Arc<dyn GitAuthAgent>,
    ssh_auth_agent: Option<Arc<dyn SHHAuthAgent>>,
}

impl GitLabClient {
    pub fn new(
        client_auth_agent: impl GitLabAuthAgent + 'static,
        ssh_auth_agent: Option<impl SHHAuthAgent + 'static>,
    ) -> Self {
        Self {
            client_auth_agent: Arc::new(client_auth_agent),
            ssh_auth_agent: ssh_auth_agent.map(|agent| Arc::new(agent) as Arc<dyn SHHAuthAgent>),
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
