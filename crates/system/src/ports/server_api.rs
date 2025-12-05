pub mod types;

use async_trait::async_trait;
use sapic_base::extension::types::{ExtensionInfo, ExtensionVersionInfo};
use sapic_core::context::AnyAsyncContext;
use std::path::{Path, PathBuf};
use types::*;

#[async_trait]
pub trait ExtensionApiOperations: Send + Sync {
    async fn list_extensions(
        &self,
        ctx: &dyn AnyAsyncContext,
    ) -> joinerror::Result<Vec<ExtensionInfo>>;

    // Returns path to the archive file and the extension folder name
    async fn download_extension(
        &self,
        ctx: &dyn AnyAsyncContext,
        extension_id: &str,
        version: &str,
        archive_folder: &Path,
    ) -> joinerror::Result<(PathBuf, ExtensionVersionInfo)>;
}

// pub trait AuthRevokeApiReq:
//     Send
//     + Sync
//     + auth_github_account_api::GitHubRevokeApiReq
//     + auth_gitlab_account_api::GitLabRevokeApiReq
// {
// }

pub mod auth_gitlab_account_api {
    use super::*;

    #[async_trait]
    pub trait GitLabPkceTokenExchangeApiReq: Send + Sync {
        async fn gitlab_pkce_token_exchange(
            &self,
            ctx: &dyn AnyAsyncContext,
            request: TokenExchangeRequest,
        ) -> joinerror::Result<GitLabPkceTokenExchangeResponse>;
    }

    #[async_trait]
    pub trait GitLabTokenRefreshApiReq: Send + Sync {
        async fn gitlab_token_refresh(
            &self,
            ctx: &dyn AnyAsyncContext,
            request: GitLabTokenRefreshRequest,
        ) -> joinerror::Result<GitLabTokenRefreshResponse>;
    }

    #[async_trait]
    pub trait GitLabRevokeApiReq: Send + Sync {
        async fn gitlab_revoke(
            &self,
            ctx: &dyn AnyAsyncContext,
            request: GitLabRevokeRequest,
        ) -> joinerror::Result<()>;
    }

    #[async_trait]
    pub trait AuthGitLabAccountApiOperations:
        Send + Sync + GitLabPkceTokenExchangeApiReq + GitLabTokenRefreshApiReq + GitLabRevokeApiReq
    {
    }
}

pub mod auth_github_account_api {
    use super::*;

    #[async_trait]
    pub trait GitHubPkceTokenExchangeApiReq: Send + Sync {
        async fn github_pkce_token_exchange(
            &self,
            ctx: &dyn AnyAsyncContext,
            request: TokenExchangeRequest,
        ) -> joinerror::Result<GitHubPkceTokenExchangeResponse>;
    }

    #[async_trait]
    pub trait GitHubRevokeApiReq: Send + Sync {
        async fn github_revoke(
            &self,
            ctx: &dyn AnyAsyncContext,
            request: GitHubRevokeRequest,
        ) -> joinerror::Result<()>;
    }

    #[async_trait]
    pub trait AuthGitHubAccountApiOperations:
        Send + Sync + GitHubPkceTokenExchangeApiReq + GitHubRevokeApiReq
    {
    }
}

pub trait AccountAuthApiOperations:
    Send
    + Sync
    + auth_github_account_api::AuthGitHubAccountApiOperations
    + auth_gitlab_account_api::AuthGitLabAccountApiOperations
{
}

pub trait ServerApiClient: Send + Sync + ExtensionApiOperations + AccountAuthApiOperations {}
