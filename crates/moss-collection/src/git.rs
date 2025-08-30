use moss_git_hosting_provider::{
    github::GitHubApiClient, gitlab::GitLabApiClient, models::primitives::GitProviderType,
};
use moss_user::{account::Account, models::primitives::AccountId};

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
    pub fn owner(&self) -> AccountId {
        match self {
            GitClient::GitHub { account, .. } => account.id(),
            GitClient::GitLab { account, .. } => account.id(),
        }
    }

    pub fn provider(&self) -> GitProviderType {
        match self {
            GitClient::GitHub { .. } => GitProviderType::GitHub,
            GitClient::GitLab { .. } => GitProviderType::GitLab,
        }
    }
}
