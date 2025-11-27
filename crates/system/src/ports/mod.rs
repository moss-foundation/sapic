pub mod github_api;
pub mod gitlab_api;
pub mod server_api;

use async_trait::async_trait;
use sapic_core::context::AnyAsyncContext;

#[derive(Debug, Clone)]
pub enum GitProviderKind {
    GitHub,
    GitLab,
}

#[async_trait]
pub trait GitAuthAdapter: Send + Sync {
    type PkceToken;

    async fn auth_with_pkce(&self, ctx: &dyn AnyAsyncContext)
    -> joinerror::Result<Self::PkceToken>;
}
