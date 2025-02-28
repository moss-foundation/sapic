use std::sync::Arc;

use moss_git::GitAuthAgent;
use url::Url;

use crate::GitHostingProvider;

pub struct GitHubClient {
    auth_agent: Arc<dyn GitAuthAgent>,
}

impl GitHubClient {
    pub fn new(auth_agent: Arc<dyn GitAuthAgent>) -> Self {
        Self { auth_agent }
    }
}

impl GitHostingProvider for GitHubClient {
    fn name(&self) -> Option<String> {
        Some("GitHub".to_string())
    }

    fn base_url(&self) -> Option<Url> {
        Some(Url::parse("https://github.com").unwrap())
    }
}
