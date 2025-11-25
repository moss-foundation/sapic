// pub mod github;
// pub mod gitlab;

// use async_trait::async_trait;
// use moss_applib::AppRuntime;

// #[derive(Debug, Clone)]
// pub enum GitProviderKind {
//     GitHub,
//     GitLab,
// }

// #[async_trait]
// pub trait GitAuthAdapter<R: AppRuntime> {
//     type PkceToken;

//     async fn auth_with_pkce(&self, ctx: &R::AsyncContext) -> joinerror::Result<Self::PkceToken>;
// }

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
