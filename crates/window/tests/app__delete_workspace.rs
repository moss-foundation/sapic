// TODO: restore this in the crate where these operations will be moved.

// #![cfg(feature = "integration-tests")]

// pub mod shared;

// use moss_testutils::random_name::random_workspace_name;
// use moss_workspace::models::primitives::{WorkspaceId, WorkspaceMode};
// use std::{path::Path, sync::Arc};
// use window::{
//     dirs,
//     models::operations::{CreateWorkspaceInput, DeleteWorkspaceInput},
// };

// use crate::shared::set_up_test_app;

// #[tokio::test]
// async fn delete_workspace_success() {
//     let (app, app_delegate, ctx, cleanup) = set_up_test_app().await;

//     // Create a workspace
//     let workspace_name = random_workspace_name();
//     let create_output = app
//         .create_workspace(
//             &ctx,
//             &app_delegate,
//             &CreateWorkspaceInput {
//                 name: workspace_name.clone(),
//                 mode: WorkspaceMode::default(),
//                 open_on_creation: false,
//             },
//         )
//         .await
//         .unwrap();

//     let workspace_path: Arc<Path> = app_delegate
//         .user_dir()
//         .join(dirs::WORKSPACES_DIR)
//         .join(&create_output.id.to_string())
//         .into();
//     assert!(workspace_path.exists());

//     // Verify workspace is in list
//     let list_workspaces = app.list_workspaces(&ctx).await.unwrap();
//     assert_eq!(list_workspaces.len(), 1);
//     assert_eq!(list_workspaces[0].id, create_output.id);

//     // Delete the workspace
//     let delete_result = app
//         .delete_workspace(
//             &ctx,
//             &app_delegate,
//             &DeleteWorkspaceInput {
//                 id: create_output.id,
//             },
//         )
//         .await;

//     assert!(delete_result.is_ok());

//     // Verify workspace directory was deleted
//     assert!(!workspace_path.exists());

//     // Verify workspace is not in list
//     let list_workspaces = app.list_workspaces(&ctx).await.unwrap();
//     assert!(list_workspaces.is_empty());

//     cleanup().await;
// }

// #[tokio::test]
// async fn delete_workspace_filesystem_only() {
//     let (app, app_delegate, ctx, cleanup) = set_up_test_app().await;

//     // Create a workspace
//     let workspace_name = random_workspace_name();
//     let create_output = app
//         .create_workspace(
//             &ctx,
//             &app_delegate,
//             &CreateWorkspaceInput {
//                 name: workspace_name.clone(),
//                 mode: WorkspaceMode::default(),
//                 open_on_creation: false,
//             },
//         )
//         .await
//         .unwrap();

//     let workspace_path: Arc<Path> = app_delegate
//         .user_dir()
//         .join(dirs::WORKSPACES_DIR)
//         .join(&create_output.id.to_string())
//         .into();
//     assert!(workspace_path.exists());

//     // Delete workspace
//     let delete_result = app
//         .delete_workspace(
//             &ctx,
//             &app_delegate,
//             &DeleteWorkspaceInput {
//                 id: create_output.id,
//             },
//         )
//         .await;

//     assert!(delete_result.is_ok());

//     // Verify workspace directory was deleted
//     assert!(!workspace_path.exists());

//     // Verify workspace is not in list
//     let list_workspaces = app.list_workspaces(&ctx).await.unwrap();
//     assert!(list_workspaces.is_empty());

//     cleanup().await;
// }

// #[tokio::test]
// async fn delete_workspace_opened() {
//     let (app, app_delegate, ctx, cleanup) = set_up_test_app().await;

//     // Create and open a workspace
//     let workspace_name = random_workspace_name();
//     let create_output = app
//         .create_workspace(
//             &ctx,
//             &app_delegate,
//             &CreateWorkspaceInput {
//                 name: workspace_name.clone(),
//                 mode: WorkspaceMode::default(),
//                 open_on_creation: true,
//             },
//         )
//         .await
//         .unwrap();

//     let workspace_path: Arc<Path> = app_delegate
//         .user_dir()
//         .join(dirs::WORKSPACES_DIR)
//         .join(&create_output.id.to_string())
//         .into();
//     assert!(workspace_path.exists());

//     // Verify workspace is active
//     let active_workspace_id = app.workspace().await.unwrap().id();
//     assert_eq!(active_workspace_id, create_output.id);

//     // Delete the workspace (should succeed and deactivate it)
//     let delete_result = app
//         .delete_workspace(
//             &ctx,
//             &app_delegate,
//             &DeleteWorkspaceInput {
//                 id: create_output.id,
//             },
//         )
//         .await;

//     assert!(delete_result.is_ok());

//     // Verify workspace directory was deleted
//     assert!(!workspace_path.exists());

//     // Verify workspace is not in list
//     let list_workspaces = app.list_workspaces(&ctx).await.unwrap();
//     assert!(list_workspaces.is_empty());

//     // Verify that no workspace is active after deletion
//     assert!(app.workspace().await.is_none());

//     cleanup().await;
// }

// #[tokio::test]
// async fn delete_workspace_nonexistent() {
//     let (app, app_delegate, ctx, cleanup) = set_up_test_app().await;

//     let nonexistent_id = WorkspaceId::new();
//     let delete_result = app
//         .delete_workspace(
//             &ctx,
//             &app_delegate,
//             &DeleteWorkspaceInput { id: nonexistent_id },
//         )
//         .await;

//     assert!(delete_result.is_err());

//     cleanup().await;
// }

// #[tokio::test]
// async fn delete_workspace_filesystem_does_not_exist() {
//     let (app, app_delegate, ctx, cleanup) = set_up_test_app().await;

//     // Create a workspace
//     let workspace_name = random_workspace_name();
//     let create_output = app
//         .create_workspace(
//             &ctx,
//             &app_delegate,
//             &CreateWorkspaceInput {
//                 name: workspace_name.clone(),
//                 mode: WorkspaceMode::default(),
//                 open_on_creation: false,
//             },
//         )
//         .await
//         .unwrap();

//     // Manually delete the filesystem directory
//     let workspace_path: Arc<Path> = app_delegate
//         .user_dir()
//         .join(dirs::WORKSPACES_DIR)
//         .join(&create_output.id.to_string())
//         .into();

//     tokio::fs::remove_dir_all(&workspace_path).await.unwrap();
//     assert!(!workspace_path.exists());

//     // Delete the workspace (should still succeed)
//     let delete_result = app
//         .delete_workspace(
//             &ctx,
//             &app_delegate,
//             &DeleteWorkspaceInput {
//                 id: create_output.id,
//             },
//         )
//         .await;

//     assert!(delete_result.is_ok());

//     // Verify workspace is not in list
//     let list_workspaces = app.list_workspaces(&ctx).await.unwrap();
//     assert!(list_workspaces.is_empty());

//     cleanup().await;
// }
