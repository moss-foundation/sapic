#![cfg(feature = "integration-tests")]

use moss_applib::context::AnyAsyncContext;
use moss_git::models::types::BranchInfo;
use moss_user::models::primitives::AccountId;
use moss_workspace::models::{
    operations::{DeleteCollectionInput, DescribeCollectionInput, ImportCollectionInput},
    types::{GitHubImportParams, ImportCollectionParams, ImportCollectionSource, VcsInfo},
};
use std::{env, ops::Deref};

use crate::shared::setup_test_workspace;

mod shared;

#[ignore]
#[tokio::test]
async fn describe_collection_with_repository() {
    let (ctx, app_delegate, workspace, cleanup) = setup_test_workspace().await;

    let account_id = ctx
        .value::<AccountId>("account_id")
        .unwrap()
        .deref()
        .clone();

    let import_collection_output = workspace
        .import_collection(
            &ctx,
            &app_delegate,
            &ImportCollectionInput {
                inner: ImportCollectionParams {
                    name: "New Collection".to_string(),
                    order: 0,
                    external_path: None,
                    icon_path: None,
                    source: ImportCollectionSource::GitHub(GitHubImportParams {
                        repository: env::var("GITHUB_COLLECTION_REPO_HTTPS").unwrap(),
                        branch: None,
                        account_id,
                    }),
                },
            },
        )
        .await
        .unwrap();

    let description = workspace
        .describe_collection(
            &ctx,
            &DescribeCollectionInput {
                id: import_collection_output.id.clone(),
            },
        )
        .await
        .unwrap();

    dbg!(&description);
    assert_eq!(description.name, "New Collection");

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
    assert_eq!(github_info.url, "github.com/brutusyhy/test-collection");
    assert!(github_info.updated_at.is_some());
    assert_eq!(github_info.owner.unwrap(), "brutusyhy");

    workspace
        .delete_collection(
            &ctx,
            &DeleteCollectionInput {
                id: import_collection_output.id,
            },
        )
        .await
        .unwrap();
    cleanup().await;
}
