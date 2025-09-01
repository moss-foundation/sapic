use joinerror::OptionExt;
use moss_applib::AppRuntime;
use moss_git_hosting_provider::GitProviderKind;
use moss_logging::session;

use crate::{
    Workspace,
    errors::ErrorNotFound,
    models::{
        operations::{DescribeCollectionInput, DescribeCollectionOutput},
        types::{Contributor, GitHubVcsInfo, GitLabVcsInfo, VcsInfo},
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
        let (vcs_summary, contributors) = if let Some(vcs) = collection.vcs() {
            let summary = match vcs.summary().await {
                Ok(summary) => Some(summary),
                Err(e) => {
                    session::warn!(format!(
                        "failed to get VCS summary for collection `{}`: {}",
                        input.id.as_str(),
                        e.to_string()
                    ));
                    None
                }
            };

            let contributors = match vcs.contributors().await {
                Ok(contributors) => Some(contributors),
                Err(e) => {
                    session::warn!(format!(
                        "failed to get VCS contributors for collection `{}`: {}",
                        input.id.as_str(),
                        e.to_string()
                    ));
                    None
                }
            };

            (summary, contributors)
        } else {
            (None, None)
        };

        let vcs = if let Some(summary) = vcs_summary {
            match summary.kind {
                GitProviderKind::GitHub => Some(VcsInfo::GitHub(GitHubVcsInfo {
                    branch: summary.branch,
                    url: summary.url,
                    updated_at: summary.updated_at,
                    owner: summary.owner.map(|owner| owner.username),
                })),
                GitProviderKind::GitLab => Some(VcsInfo::GitLab(GitLabVcsInfo {
                    branch: summary.branch,
                    url: summary.url,
                    updated_at: summary.updated_at,
                    owner: summary.owner.map(|owner| owner.username),
                })),
            }
        } else {
            None
        };

        let contributors = contributors
            .map(|contributors| {
                contributors
                    .into_iter()
                    .map(|contributor| Contributor {
                        name: contributor.username,
                        avatar_url: contributor.avatar_url,
                    })
                    .collect()
            })
            .unwrap_or_default();

        Ok(DescribeCollectionOutput {
            name: details.name,
            vcs,
            contributors,
            created_at: details.created_at,
        })
    }
}
