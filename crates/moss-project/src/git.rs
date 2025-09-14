mod types;

use std::sync::Arc;

use moss_applib::AppRuntime;
pub use types::*;

use moss_git::url::GitUrl;
use moss_git_hosting_provider::{
    GitProviderKind, github::client::GitHubApiClient, gitlab::client::GitLabApiClient,
};
use moss_user::{AccountSession, account::Account, models::primitives::AccountId};

#[derive(Clone)]
pub enum GitClient<R: AppRuntime> {
    GitHub {
        account: Account<R>,
        api: Arc<dyn GitHubApiClient<R>>,
    },
    GitLab {
        account: Account<R>,
        api: Arc<dyn GitLabApiClient<R>>,
    },
}

impl<R: AppRuntime> GitClient<R> {
    pub fn account_id(&self) -> AccountId {
        match self {
            GitClient::GitHub { account, .. } => account.id(),
            GitClient::GitLab { account, .. } => account.id(),
        }
    }

    pub fn kind(&self) -> GitProviderKind {
        match self {
            GitClient::GitHub { .. } => GitProviderKind::GitHub,
            GitClient::GitLab { .. } => GitProviderKind::GitLab,
        }
    }

    pub fn session(&self) -> &AccountSession<R> {
        match self {
            GitClient::GitHub { account, .. } => account.session(),
            GitClient::GitLab { account, .. } => account.session(),
        }
    }

    pub fn username(&self) -> String {
        match self {
            GitClient::GitHub { account, .. } => account.username(),
            GitClient::GitLab { account, .. } => account.username(),
        }
    }

    pub async fn repository(
        &self,
        ctx: &R::AsyncContext,
        url: &GitUrl,
    ) -> joinerror::Result<RepositoryInfo> {
        match self {
            GitClient::GitHub { account, api } => {
                let resp = api.get_repository(ctx, account.session(), url).await?;

                Ok(RepositoryInfo {
                    updated_at: resp.updated_at,
                    owner: OwnerInfo {
                        username: resp.owner.login,
                    },
                })
            }
            GitClient::GitLab { account, api } => {
                let resp = api.get_repository(ctx, account.session(), url).await?;

                Ok(RepositoryInfo {
                    updated_at: resp.updated_at,
                    owner: OwnerInfo {
                        username: resp.owner.username,
                    },
                })
            }
        }
    }

    pub async fn contributors(
        &self,
        ctx: &R::AsyncContext,
        url: &GitUrl,
    ) -> joinerror::Result<Vec<ContributorInfo>> {
        match self {
            GitClient::GitHub { account, api } => {
                let resp = api.get_contributors(ctx, account.session(), url).await?;

                let mut result = Vec::with_capacity(resp.items.len());
                for item in resp.items {
                    result.push(ContributorInfo {
                        username: item.login,
                        avatar_url: Some(item.avatar_url),
                    });
                }

                Ok(result)
            }
            GitClient::GitLab { account, api } => {
                let resp = api.get_contributors(ctx, account.session(), url).await?;

                let mut result = Vec::with_capacity(resp.items.len());
                for item in resp.items {
                    result.push(ContributorInfo {
                        username: item.name,
                        avatar_url: None, // FIXME: GitLab does not provide avatar URL when fetching contributors, so we have to fetch it separately
                    });
                }

                Ok(result)
            }
        }
    }
}
