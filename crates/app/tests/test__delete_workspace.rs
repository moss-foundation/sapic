#![cfg(feature = "integration-tests")]

use moss_applib::AppRuntime;
use moss_testutils::random_name::random_workspace_name;
use sapic_base::workspace::types::primitives::WorkspaceId;
use sapic_ipc::contracts::workspace::DeleteWorkspaceInput;

use crate::shared::setup_test_app;

mod shared;

#[tokio::test]
async fn test_delete_workspace_success() {
    let (app, delegate, ctx, cleanup) = setup_test_app().await;

    let name = random_workspace_name();
    let id = app.create_workspace(&name).await.unwrap();

    let workspaces = app.list_workspaces(&ctx, &delegate).await.unwrap().0;

    assert_eq!(workspaces.len(), 1);

    app.delete_workspace(&ctx, &DeleteWorkspaceInput { id })
        .await
        .unwrap();

    let workspaces = app.list_workspaces(&ctx, &delegate).await.unwrap().0;

    assert!(workspaces.is_empty());

    cleanup().await;
}

#[tokio::test]
async fn test_delete_workspace_non_existent() {
    let (app, _delegate, ctx, cleanup) = setup_test_app().await;

    let id = WorkspaceId::new();
    let result = app
        .delete_workspace(&ctx, &DeleteWorkspaceInput { id })
        .await
        .unwrap();

    // When deleting a nonexistent workspace, its absolute path will be none
    assert!(result.abs_path.is_none());

    cleanup().await;
}

// FIXME: This test doesn't work until we address the issue with database file lock
// The functionality works on the frontend, just that not all files are properly cleaned up
// #[tokio::test]
// async fn test_delete_workspace_opened_in_main_window() {
//     let (app, delegate, ctx, cleanup) = setup_test_app().await;
//
//     let name = random_workspace_name();
//     let id = app.create_workspace(&name).await.unwrap();
//
//     app.ensure_main_for_workspace(&ctx, &delegate, id.clone())
//         .await
//         .unwrap();
//
//     // Try deleting a workspace when there's a main window associated with it
//
//     app.delete_workspace(&ctx, &DeleteWorkspaceInput { id }).await.unwrap();
//
// }
