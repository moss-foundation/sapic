use moss_applib::AppRuntime;
use moss_logging::session;
use sapic_base::other::GitProviderKind;
use sapic_ipc::contracts::main::project::{
    Contributor, DescribeProjectInput, DescribeProjectOutput, GitHubVcsInfo, GitLabVcsInfo, VcsInfo,
};

use crate::MainWindow;

impl<R: AppRuntime> MainWindow<R> {
    pub async fn describe_project(
        &self,
        ctx: &R::AsyncContext,
        input: &DescribeProjectInput,
    ) -> joinerror::Result<DescribeProjectOutput> {
        let project = self
            .workspace
            .load()
            .project(ctx, &input.id)
            .await?
            .handle
            .clone();

        let details = project.details(ctx).await?;
        let (vcs_summary, contributors) = if let Some(vcs) = project.vcs::<R>() {
            let summary = match vcs.summary(ctx).await {
                Ok(summary) => Some(summary),
                Err(e) => {
                    session::warn!(format!(
                        "failed to get VCS summary for project `{}`: {}",
                        input.id.as_str(),
                        e.to_string()
                    ));
                    None
                }
            };

            let contributors = match vcs.contributors(ctx).await {
                Ok(contributors) => Some(contributors),
                Err(e) => {
                    session::warn!(format!(
                        "failed to get VCS contributors for project `{}`: {}",
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

        Ok(DescribeProjectOutput {
            name: details.name,
            vcs,
            contributors,
            created_at: details.created_at,
        })
    }
}
