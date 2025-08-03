pub mod common;
pub mod github;
pub mod gitlab;
pub mod models;

use anyhow::Result;
use async_trait::async_trait;
use url::Url;

use crate::models::types::{Contributor, RepositoryInfo};

#[async_trait]
pub trait GitHostingProvider {
    fn name(&self) -> String;
    fn base_url(&self) -> Url;

    // FIXME: Where's the best place to put Provider REST APIs?
    async fn contributors(&self, repo_url: &str) -> Result<Vec<Contributor>>;

    async fn repository_info(&self, repo_url: &str) -> Result<RepositoryInfo>;
}

pub(crate) mod constants {
    pub const GITHUB_API_URL: &'static str = "https://api.github.com";
    pub const GITLAB_API_URL: &'static str = "https://gitlab.com/api/v4";
}
