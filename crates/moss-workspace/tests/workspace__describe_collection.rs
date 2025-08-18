#![cfg(feature = "integration-tests")]

use crate::shared::setup_test_workspace;
use moss_workspace::models::{
    operations::{DeleteCollectionInput, DescribeCollectionInput, ImportCollectionInput},
    types::{GitHubImportParams, ImportCollectionParams, ImportCollectionSource, VcsInfo},
};
use std::env;

mod shared;

#[tokio::test]
async fn describe_collection_with_repository() {
    let (ctx, workspace, cleanup) = setup_test_workspace().await;

    let import_collection_output = workspace
        .import_collection(
            &ctx,
            &ImportCollectionInput {
                inner: ImportCollectionParams {
                    name: "New Collection".to_string(),
                    order: 0,
                    external_path: None,
                    icon_path: None,
                    source: ImportCollectionSource::GitHub(GitHubImportParams {
                        repository: env::var("GITHUB_COLLECTION_REPO_HTTPS").unwrap(),
                        branch: None,
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

    assert_eq!(github_info.branch, "main");
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
