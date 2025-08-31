use joinerror::OptionExt;
use moss_applib::AppRuntime;
use moss_git_hosting_provider::models::primitives::GitProviderKind;
use moss_logging::session;

use crate::{
    Workspace,
    errors::ErrorNotFound,
    models::{
        operations::{DescribeCollectionInput, DescribeCollectionOutput},
        types::{GitHubVcsInfo, GitLabVcsInfo, VcsInfo},
    },
};

impl<R: AppRuntime> Workspace<R> {
    pub async fn describe_collection(
        &self,
        _ctx: &R::AsyncContext,
        input: &DescribeCollectionInput,
    ) -> joinerror::Result<DescribeCollectionOutput> {
        let collection = self
            .collection_service
            .collection(&input.id)
            .await
            .ok_or_join_err_with::<ErrorNotFound>(|| {
                format!("collection `{}` not found", input.id.as_str())
            })?;

        let details = collection.details().await?;
        let vcs = if let Some(vcs) = collection.vcs() {
            match vcs.summary().await {
                Ok(summary) => Some(summary),
                Err(e) => {
                    session::warn!(format!(
                        "failed to get VCS summary for collection `{}`: {}",
                        input.id.as_str(),
                        e.to_string()
                    ));
                    None
                }
            }
        } else {
            None
        };

        let vcs = if let Some(vcs) = vcs {
            match vcs.kind {
                GitProviderKind::GitHub => Some(VcsInfo::GitHub(GitHubVcsInfo {
                    branch: vcs.branch,
                    url: vcs.url,
                    updated_at: vcs.updated_at,
                    owner: vcs.owner.map(|owner| owner.username),
                })),
                GitProviderKind::GitLab => Some(VcsInfo::GitLab(GitLabVcsInfo {
                    branch: vcs.branch,
                    url: vcs.url,
                    updated_at: vcs.updated_at,
                    owner: vcs.owner.map(|owner| owner.username),
                })),
            }
        } else {
            None
        };

        Ok(DescribeCollectionOutput {
            name: details.name,
            vcs,
            contributors: details.contributors,
            created_at: details.created_at,
        })
    }
}
