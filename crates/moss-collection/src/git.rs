use moss_git_hosting_provider::{github::GitHubApiClient, gitlab::GitLabApiClient};
use moss_user::account::Account;

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
