use std::sync::Arc;
use url::Url;
use moss_git::GitAuthAgent;
use crate::GitHostingProvider;

pub struct GitLabClient {
    auth_agent: Arc<dyn GitAuthAgent>,
}

impl GitLabClient {
    pub fn new(auth_agent: Arc<dyn GitAuthAgent>) -> Self {
        Self { auth_agent }
    }
}

impl GitHostingProvider for GitLabClient {
    fn name(&self) -> Option<String> {Some("GitLab".to_string())}
    fn base_url(&self) -> Option<Url> {Some(Url::parse("https://gitlab.com").unwrap())}
}