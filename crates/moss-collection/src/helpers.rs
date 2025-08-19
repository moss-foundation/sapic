use moss_git_hosting_provider::{
    GitHostingProvider,
    common::{GITHUB_DOMAIN, GITLAB_DOMAIN, GitUrlForAPI},
    github::client::GitHubClient,
    gitlab::client::GitLabClient,
    models::{primitives::GitProviderType, types::Contributor},
};
use std::sync::Arc;

use crate::{collection::Vcs, services::git_service::GitService};

pub(crate) async fn fetch_contributors(
    repo_ref: &GitUrlForAPI,
    github_client: Arc<GitHubClient>,
    gitlab_client: Arc<GitLabClient>,
) -> joinerror::Result<Vec<Contributor>> {
    // TODO: In the future we might support non-VCS contributors?
    let client: Arc<dyn GitHostingProvider> = match repo_ref.domain.as_str() {
        GITHUB_DOMAIN => github_client,
        GITLAB_DOMAIN => gitlab_client,
        other => {
            return Err(joinerror::Error::new::<()>(format!(
                "unsupported git provider domain: {}",
                other
            )));
        }
    };

    match client.contributors(repo_ref).await {
        Ok(contributors) => Ok(contributors),
        Err(e) => {
            // TODO: Tell the frontend provider API call fails
            println!("git provider api call fails: {}", e);
            Ok(Vec::new())
        }
    }
}

pub(crate) async fn fetch_vcs_info(
    repo_ref: &GitUrlForAPI,
    git_service: Arc<GitService>,
    github_client: Arc<GitHubClient>,
    gitlab_client: Arc<GitLabClient>,
) -> joinerror::Result<Vcs> {
    let branch = git_service.get_current_branch().await?;

    let (client, provider_type) = match repo_ref.domain.as_str() {
        GITHUB_DOMAIN => (
            github_client as Arc<dyn GitHostingProvider>,
            GitProviderType::GitHub,
        ),
        GITLAB_DOMAIN => (
            gitlab_client as Arc<dyn GitHostingProvider>,
            GitProviderType::GitLab,
        ),
        other => {
            return Err(joinerror::Error::new::<()>(format!(
                "unsupported git provider domain: {}",
                other
            )));
        }
    };

    let repository_metadata = client.repository_metadata(repo_ref).await;
    let url = repo_ref.to_string();

    // Even if provider API call fails, we want to return repo_url and current branch
    match repository_metadata {
        Ok(repository_metadata) => {
            let updated_at = Some(repository_metadata.updated_at);
            let owner = Some(repository_metadata.owner);

            Ok(Vcs::new(provider_type, branch, url, updated_at, owner))
        }
        Err(e) => {
            // TODO: Tell the frontend provider API call fails
            println!("git provider api call fails: {}", e);

            Ok(Vcs::new(provider_type, branch, url, None, None))
        }
    }
}
