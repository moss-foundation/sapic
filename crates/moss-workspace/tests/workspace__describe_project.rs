#![cfg(feature = "integration-tests")]

use moss_git::models::types::BranchInfo;
use moss_user::models::primitives::AccountId;
use moss_workspace::models::{
    operations::{DeleteProjectInput, DescribeProjectInput, ImportProjectInput},
    types::{ImportGitHubParams, ImportProjectParams, ImportProjectSource, VcsInfo},
};
use sapic_core::context::AnyAsyncContext;
use std::{env, ops::Deref};

use crate::shared::setup_test_workspace;

mod shared;

#[ignore]
#[tokio::test]
async fn describe_project_with_repository() {
    let (ctx, app_delegate, workspace, cleanup) = setup_test_workspace().await;

    let account_id = ctx
        .value("account_id")
        .unwrap()
        .downcast::<AccountId>()
        .unwrap()
        .deref()
        .clone();

    let import_project_output = workspace
        .import_project(
            &ctx,
            &app_delegate,
            &ImportProjectInput {
                inner: ImportProjectParams {
                    name: "New Project".to_string(),
                    order: 0,
                    icon_path: None,
                    source: ImportProjectSource::GitHub(ImportGitHubParams {
                        repository: env::var("GITHUB_PROJECT_REPO_HTTPS").unwrap(),
                        branch: None,
                        account_id,
                    }),
                },
            },
        )
        .await
        .unwrap();

    let description = workspace
        .describe_project(
            &ctx,
            &DescribeProjectInput {
                id: import_project_output.id.clone(),
            },
        )
        .await
        .unwrap();

    assert_eq!(description.name, "New Project");

    let vcs = description.vcs.unwrap();
    let github_info = match vcs {
        VcsInfo::GitHub(info) => info,
        VcsInfo::GitLab(_) => {
            unreachable!()
        }
    };

    assert_eq!(
        github_info.branch,
        BranchInfo {
            name: "main".to_string(),
            ahead: Some(0),
            behind: Some(0),
        }
    );
    assert_eq!(github_info.url, "github.com/brutusyhy/test-project");
    assert!(github_info.updated_at.is_some());
    assert_eq!(github_info.owner.unwrap(), "brutusyhy");

    workspace
        .delete_project(
            &ctx,
            &DeleteProjectInput {
                id: import_project_output.id,
            },
        )
        .await
        .unwrap();
    cleanup().await;
}
