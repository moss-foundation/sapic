use joinerror::OptionExt;
use moss_applib::AppRuntime;
use moss_collection::collection::VcsSummary;

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

        let details = collection.describe_details().await?;

        let vcs = if let Some(vcs) = details.vcs {
            match vcs {
                VcsSummary::GitHub {
                    branch,
                    url,
                    updated_at,
                    owner,
                } => Some(VcsInfo::GitHub(GitHubVcsInfo {
                    branch,
                    url,
                    updated_at,
                    owner,
                })),
                VcsSummary::GitLab {
                    branch,
                    url,
                    updated_at,
                    owner,
                } => Some(VcsInfo::GitLab(GitLabVcsInfo {
                    branch,
                    url,
                    updated_at,
                    owner,
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
