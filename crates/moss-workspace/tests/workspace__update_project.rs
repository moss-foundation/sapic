#![cfg(feature = "integration-tests")]
pub mod shared;

use moss_applib::mock::MockAppRuntime;
use moss_bindingutils::primitives::ChangePath;
use moss_testutils::random_name::random_project_name;
use moss_workspace::models::{
    operations::{CreateProjectInput, UpdateProjectInput},
    types::{CreateProjectParams, UpdateProjectParams},
};
use sapic_base::project::types::primitives::ProjectId;

use crate::shared::{generate_random_icon, setup_test_workspace};

#[tokio::test]
async fn rename_project_success() {
    let (ctx, app_delegate, workspace, cleanup) = setup_test_workspace().await;

    let old_project_name = random_project_name();
    let create_project_output = workspace
        .create_project(
            &ctx,
            &app_delegate,
            &CreateProjectInput {
                inner: CreateProjectParams {
                    name: old_project_name.clone(),
                    order: 0,
                    external_path: None,
                    git_params: None,
                    icon_path: None,
                },
            },
        )
        .await
        .unwrap();

    let new_project_name = random_project_name();
    let _ = workspace
        .update_project::<MockAppRuntime>(
            &ctx,
            UpdateProjectInput {
                inner: UpdateProjectParams {
                    id: create_project_output.id.clone(),
                    name: Some(new_project_name.clone()),
                    repository: None,
                    icon_path: None,
                    order: None,
                    expanded: None,
                },
            },
        )
        .await
        .unwrap();

    // Verify the manifest is updated
    let project = workspace
        .project(&create_project_output.id.into())
        .await
        .unwrap();
    assert_eq!(project.details().await.unwrap().name, new_project_name);

    cleanup().await;
}

#[tokio::test]
async fn rename_project_empty_name() {
    let (ctx, app_delegate, workspace, cleanup) = setup_test_workspace().await;

    let old_project_name = random_project_name();
    let create_project_output = workspace
        .create_project(
            &ctx,
            &app_delegate,
            &CreateProjectInput {
                inner: CreateProjectParams {
                    name: old_project_name.clone(),
                    order: 0,
                    external_path: None,
                    git_params: None,
                    icon_path: None,
                },
            },
        )
        .await
        .unwrap();

    let new_project_name = "".to_string();
    let rename_project_result = workspace
        .update_project::<MockAppRuntime>(
            &ctx,
            UpdateProjectInput {
                inner: UpdateProjectParams {
                    id: create_project_output.id,
                    name: Some(new_project_name.clone()),
                    repository: None,
                    icon_path: None,
                    order: None,
                    expanded: None,
                },
            },
        )
        .await;

    assert!(rename_project_result.is_err());
    cleanup().await;
}

#[tokio::test]
async fn rename_project_unchanged() {
    let (ctx, app_delegate, workspace, cleanup) = setup_test_workspace().await;

    let old_project_name = random_project_name();
    let create_project_output = workspace
        .create_project(
            &ctx,
            &app_delegate,
            &CreateProjectInput {
                inner: CreateProjectParams {
                    name: old_project_name.clone(),
                    order: 0,
                    external_path: None,
                    git_params: None,
                    icon_path: None,
                },
            },
        )
        .await
        .unwrap();

    let new_project_name = old_project_name;
    let _ = workspace
        .update_project::<MockAppRuntime>(
            &ctx,
            UpdateProjectInput {
                inner: UpdateProjectParams {
                    id: create_project_output.id,
                    name: Some(new_project_name),
                    repository: None,
                    icon_path: None,
                    order: None,
                    expanded: None,
                },
            },
        )
        .await
        .unwrap();

    cleanup().await;
}

#[tokio::test]
async fn rename_project_nonexistent_id() {
    let (ctx, _, workspace, cleanup) = setup_test_workspace().await;

    // Use a random ID that doesn't exist
    let nonexistent_id = ProjectId::new();

    let result = workspace
        .update_project::<MockAppRuntime>(
            &ctx,
            UpdateProjectInput {
                inner: UpdateProjectParams {
                    id: nonexistent_id,
                    name: Some(random_project_name()),
                    repository: None,
                    icon_path: None,
                    order: None,
                    expanded: None,
                },
            },
        )
        .await;

    assert!(result.is_err());

    cleanup().await;
}

#[tokio::test]
async fn update_project_new_icon() {
    let (ctx, app_delegate, workspace, cleanup) = setup_test_workspace().await;
    let project_name = random_project_name();
    let id = workspace
        .create_project(
            &ctx,
            &app_delegate,
            &CreateProjectInput {
                inner: CreateProjectParams {
                    name: project_name.to_string(),
                    order: 0,
                    external_path: None,
                    git_params: None,
                    icon_path: None,
                },
            },
        )
        .await
        .unwrap()
        .id;

    let icon_path = workspace.abs_path().join("test_icon.png");
    generate_random_icon(&icon_path);

    let _ = workspace
        .update_project::<MockAppRuntime>(
            &ctx,
            UpdateProjectInput {
                inner: UpdateProjectParams {
                    id: id.clone(),
                    name: None,
                    repository: None,
                    icon_path: Some(ChangePath::Update(icon_path.clone())),
                    order: None,
                    expanded: None,
                },
            },
        )
        .await
        .unwrap();

    // Verify the icon is generated
    let project = workspace.project(&id).await.unwrap();
    assert!(project.icon_path().is_some());

    cleanup().await;
}

#[tokio::test]
async fn update_project_remove_icon() {
    let (ctx, app_delegate, workspace, cleanup) = setup_test_workspace().await;
    let project_name = random_project_name();

    let icon_path = workspace.abs_path().join("test_icon.png");
    generate_random_icon(&icon_path);

    let id = workspace
        .create_project(
            &ctx,
            &app_delegate,
            &CreateProjectInput {
                inner: CreateProjectParams {
                    name: project_name.clone(),
                    order: 0,
                    external_path: None,
                    git_params: None,
                    icon_path: Some(icon_path.clone()),
                },
            },
        )
        .await
        .unwrap()
        .id;

    let _ = workspace
        .update_project::<MockAppRuntime>(
            &ctx,
            UpdateProjectInput {
                inner: UpdateProjectParams {
                    id: id.clone(),
                    name: None,
                    repository: None,
                    icon_path: Some(ChangePath::Remove),
                    order: None,
                    expanded: None,
                },
            },
        )
        .await
        .unwrap();

    // Verify the icon is removed
    let project = workspace.project(&id).await.unwrap();
    assert!(project.icon_path().is_none());

    cleanup().await;
}

// TODO: Reenable this test once we introduce relinking a project with a new remote repo

// #[tokio::test]
// async fn update_collection_repo() {
//     let (ctx, workspace, cleanup) = setup_test_workspace().await;
//
//     let project_name = random_project_name();
//     let old_repo = "https://github.com/xxx/1.git".to_string();
//     let new_repo = "https://github.com/xxx/2.git".to_string();
//     let new_normalized_repo = "github.com/xxx/2";
//     let create_project_output = workspace
//         .create_collection(
//             &ctx,
//             &CreateCollectionInput {
//                 name: project_name,
//                 order: 0,
//                 external_path: None,
//                 repository: Some(old_repo),
//                 icon_path: None,
//             },
//         )
//         .await
//         .unwrap();
//
//     let _ = workspace
//         .update_collection(
//             &ctx,
//             UpdateCollectionInput {
//                 id: create_project_output.id.clone(),
//                 name: None,
//                 repository: Some(ChangeString::Update(new_repo.clone())),
//                 icon_path: None,
//                 order: None,
//                 pinned: None,
//                 expanded: None,
//             },
//         )
//         .await
//         .unwrap();
//
//     // Verify the manifest is updated
//     let collection = workspace
//         .collection(&create_project_output.id.into())
//         .await
//         .unwrap();
//
//     assert_eq!(
//         collection.describe().await.unwrap().repository,
//         Some(new_normalized_repo.to_owned())
//     );
//
//     cleanup().await;
// }
