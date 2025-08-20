use moss_applib::AppRuntime;
use moss_git_hosting_provider::{
    GitHostingProvider,
    common::GitUrlForAPI,
    models::{primitives::GitProviderType, types::Contributor},
};
use std::sync::Arc;

use crate::{Collection, collection::VcsSummary};

pub(crate) async fn fetch_contributors(
    repo_ref: &GitUrlForAPI,
    client: Arc<dyn GitHostingProvider>,
) -> joinerror::Result<Vec<Contributor>> {
    // TODO: In the future we might support non-VCS contributors?
    match client.contributors(repo_ref).await {
        Ok(contributors) => Ok(contributors),
        Err(e) => {
            // TODO: Tell the frontend provider API call fails
            println!("git provider api call fails: {}", e);
            Ok(Vec::new())
        }
    }
}

pub(crate) async fn fetch_vcs_summary<R: AppRuntime>(
    collection: &Collection<R>, // We need to get the current branch name
    repo_ref: &GitUrlForAPI,
    git_provider_type: GitProviderType,
    client: Arc<dyn GitHostingProvider>,
) -> joinerror::Result<VcsSummary> {
    let branch = collection.get_current_branch_info().await?;

    let repository_metadata = client.repository_metadata(repo_ref).await;
    let url = repo_ref.to_string();

    // Even if provider API call fails, we want to return repo_url and current branch
    let (updated_at, owner) = match repository_metadata {
        Ok(repository_metadata) => (
            Some(repository_metadata.updated_at),
            Some(repository_metadata.owner),
        ),
        Err(e) => {
            // TODO: Tell the frontend provider API call fails
            println!("git provider api call fails: {}", e);

            (None, None)
        }
    };

    match git_provider_type {
        GitProviderType::GitHub => Ok(VcsSummary::GitHub {
            branch,
            url,
            updated_at,
            owner,
        }),
        GitProviderType::GitLab => Ok(VcsSummary::GitLab {
            branch,
            url,
            updated_at,
            owner,
        }),
    }
}
