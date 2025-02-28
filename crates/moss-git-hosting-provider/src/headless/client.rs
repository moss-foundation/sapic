use std::sync::Arc;
use url::Url;
use moss_git::GitAuthAgent;
use crate::GitHostingProvider;

pub struct HeadlessClient {
    auth_agent: Arc<dyn GitAuthAgent>
}

impl HeadlessClient {
    pub fn new(auth_agent: Arc<dyn GitAuthAgent>) -> Self {
        Self { auth_agent }
    }
}

impl GitHostingProvider for HeadlessClient {
    fn name(&self) -> Option<String> {
        None
    }

    fn base_url(&self) -> Option<Url> {
        None
    }
}