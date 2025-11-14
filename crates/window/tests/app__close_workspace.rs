// TODO: restore this in the crate where these operations will be moved.

// #![cfg(feature = "integration-tests")]

// pub mod shared;

// use moss_storage2::{Storage, models::primitives::StorageScope};
// use moss_testutils::random_name::random_workspace_name;
// use moss_workspace::models::primitives::{WorkspaceId, WorkspaceMode};
// use window::{
//     models::operations::{CloseWorkspaceInput, CreateWorkspaceInput, OpenWorkspaceInput},
//     storage::KEY_LAST_ACTIVE_WORKSPACE,
// };

// use crate::shared::set_up_test_app;

// #[tokio::test]
// async fn close_workspace_success() {
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

//     // Close the workspace
//     let close_result = app
//         .close_workspace(
//             &ctx,
//             &app_delegate,
//             &CloseWorkspaceInput {
//                 id: create_output.id.clone(),
//             },
//         )
//         .await;

//     assert!(close_result.is_ok());
//     let close_output = close_result.unwrap();
//     assert_eq!(close_output.id, create_output.id);

//     // Check that no workspace is active
//     assert!(app.workspace().await.is_none());

//     // Check that last active workspace is removed from database
//     let storage = <dyn Storage>::global(&app_delegate);
//     assert!(
//         storage
//             .get(StorageScope::Application, KEY_LAST_ACTIVE_WORKSPACE)
//             .await
//             .unwrap()
//             .is_none()
//     );

//     cleanup().await;
// }

// #[tokio::test]
// async fn close_workspace_not_open() {
//     let (app, app_delegate, ctx, cleanup) = set_up_test_app().await;

//     // Create a workspace without opening it
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

//     // Attempt to close the workspace (should fail because it's not open)
//     let close_result = app
//         .close_workspace(
//             &ctx,
//             &app_delegate,
//             &CloseWorkspaceInput {
//                 id: create_output.id,
//             },
//         )
//         .await;

//     assert!(close_result.is_err());

//     cleanup().await;
// }

// #[tokio::test]
// async fn close_workspace_after_another_opened() {
//     let (app, app_delegate, ctx, cleanup) = set_up_test_app().await;

//     // Create and open first workspace
//     let workspace_name1 = random_workspace_name();
//     let create_output1 = app
//         .create_workspace(
//             &ctx,
//             &app_delegate,
//             &CreateWorkspaceInput {
//                 name: workspace_name1.clone(),
//                 mode: WorkspaceMode::default(),
//                 open_on_creation: false,
//             },
//         )
//         .await
//         .unwrap();

//     // Open first workspace
//     app.open_workspace(
//         &ctx,
//         &app_delegate,
//         &OpenWorkspaceInput {
//             id: create_output1.id.clone(),
//         },
//     )
//     .await
//     .unwrap();

//     // Create and open second workspace
//     let workspace_name2 = random_workspace_name();
//     let create_output2 = app
//         .create_workspace(
//             &ctx,
//             &app_delegate,
//             &CreateWorkspaceInput {
//                 name: workspace_name2.clone(),
//                 mode: WorkspaceMode::default(),
//                 open_on_creation: true,
//             },
//         )
//         .await
//         .unwrap();

//     // Check that the second workspace is active

//     let maybe_active_id = app.workspace().await.map(|w| w.id());
//     assert!(maybe_active_id.is_some());
//     let active_id = maybe_active_id.unwrap();

//     assert_eq!(active_id, create_output2.id);

//     // Attempt to close the first workspace (should fail because it's not active)
//     let close_result1 = app
//         .close_workspace(
//             &ctx,
//             &app_delegate,
//             &CloseWorkspaceInput {
//                 id: create_output1.id.clone(),
//             },
//         )
//         .await;

//     assert!(close_result1.is_err());

//     // Close the second workspace (should succeed)
//     let close_result2 = app
//         .close_workspace(
//             &ctx,
//             &app_delegate,
//             &CloseWorkspaceInput {
//                 id: create_output2.id.clone(),
//             },
//         )
//         .await;

//     assert!(close_result2.is_ok());
//     let close_output = close_result2.unwrap();
//     assert_eq!(close_output.id, create_output2.id);

//     // Check that no workspace is active
//     assert!(app.workspace().await.is_none());

//     // Check that last active workspace is removed from database
//     let storage = <dyn Storage>::global(&app_delegate);
//     assert!(
//         storage
//             .get(StorageScope::Application, KEY_LAST_ACTIVE_WORKSPACE)
//             .await
//             .unwrap()
//             .is_none()
//     );

//     cleanup().await;
// }

// #[tokio::test]
// async fn close_workspace_nonexistent() {
//     let (app, app_delegate, ctx, cleanup) = set_up_test_app().await;

//     let nonexistent_id = WorkspaceId::new();

//     let close_result = app
//         .close_workspace(
//             &ctx,
//             &app_delegate,
//             &CloseWorkspaceInput { id: nonexistent_id },
//         )
//         .await;

//     assert!(close_result.is_err());

//     cleanup().await;
// }

// #[tokio::test]
// async fn close_workspace_from_different_session() {
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

//     // Open the workspace
//     app.open_workspace(
//         &ctx,
//         &app_delegate,
//         &OpenWorkspaceInput {
//             id: create_output.id,
//         },
//     )
//     .await
//     .unwrap();

//     // Try to close a workspace with wrong id
//     let wrong_id = WorkspaceId::new();
//     let close_result = app
//         .close_workspace(&ctx, &app_delegate, &CloseWorkspaceInput { id: wrong_id })
//         .await;

//     assert!(close_result.is_err());

//     cleanup().await;
// }
