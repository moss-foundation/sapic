pub mod common;
pub mod github;
pub mod gitlab;

#[derive(Debug, Clone)]
pub enum GitProviderKind {
    GitHub,
    GitLab,
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
