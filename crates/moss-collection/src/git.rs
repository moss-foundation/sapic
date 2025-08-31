mod types;

pub use types::*;

use moss_git_hosting_provider::{
    common::GitUrl, github::GitHubApiClient, gitlab::GitLabApiClient,
    models::primitives::GitProviderKind,
};
use moss_user::{AccountSession, account::Account, models::primitives::AccountId};

#[derive(Clone)]
pub enum GitClient {
    GitHub {
        account: Account,
        api: GitHubApiClient,
    },
    GitLab {
        account: Account,
        api: GitLabApiClient,
    },
}

impl GitClient {
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

    pub fn session(&self) -> &AccountSession {
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

    pub async fn repository(&self, url: &GitUrl) -> joinerror::Result<RepositoryInfo> {
        match self {
            GitClient::GitHub { account, api } => {
                let resp = api.get_repository(account.session(), url).await?;

                Ok(RepositoryInfo {
                    updated_at: resp.updated_at,
                    owner: OwnerInfo {
                        username: resp.owner.login,
                    },
                })
            }
            GitClient::GitLab { account, api } => {
                let resp = api.get_repository(account.session(), url).await?;

                Ok(RepositoryInfo {
                    updated_at: resp.updated_at,
                    owner: OwnerInfo {
                        username: resp.owner.username,
                    },
                })
            }
        }
    }
}
