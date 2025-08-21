pub mod common;
pub mod github;
pub mod gitlab;
pub mod models;

use async_trait::async_trait;
use moss_git::GitAuthAgent;
use std::sync::Arc;
use url::Url;

use crate::{
    common::GitUrl,
    models::types::{Contributor, RepositoryMetadata, UserInfo},
};

#[async_trait]
pub trait GitHostingProvider: GitAuthProvider + Send + Sync {
    fn name(&self) -> String;
    fn base_url(&self) -> Url;

    // FIXME: Where's the best place to put Provider REST APIs?
    async fn current_user(&self) -> joinerror::Result<UserInfo>;

    async fn contributors(&self, repo_ref: &GitUrl) -> joinerror::Result<Vec<Contributor>>;

    async fn repository_metadata(&self, repo_ref: &GitUrl)
    -> joinerror::Result<RepositoryMetadata>;
}

pub trait GitAuthProvider {
    fn git_auth_agent(&self) -> Arc<dyn GitAuthAgent>;
}

pub(crate) mod constants {
    pub const GITHUB_API_URL: &'static str = "https://api.github.com";
    pub const GITLAB_API_URL: &'static str = "https://gitlab.com/api/v4";
}

#[cfg(any(test, feature = "integration-tests"))]
pub mod envvar_keys {
    /// Environment variable keys
    pub const GITHUB_CLIENT_ID: &'static str = "GITHUB_CLIENT_ID";
    pub const GITHUB_CLIENT_SECRET: &'static str = "GITHUB_CLIENT_SECRET";
    pub const GITHUB_ACCESS_TOKEN: &'static str = "GITHUB_ACCESS_TOKEN";
    pub const GITLAB_CLIENT_ID: &'static str = "GITLAB_CLIENT_ID";
    pub const GITLAB_CLIENT_SECRET: &'static str = "GITLAB_CLIENT_SECRET";
    pub const GITLAB_REFRESH_TOKEN: &'static str = "GITLAB_REFRESH_TOKEN";
}
