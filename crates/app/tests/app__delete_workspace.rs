#![cfg(feature = "integration-tests")]

use crate::shared::setup_test_app;
use moss_testutils::random_name::random_workspace_name;
use sapic_base::workspace::types::primitives::WorkspaceId;
use sapic_ipc::contracts::workspace::DeleteWorkspaceInput;
use sapic_system::workspace::WorkspaceCreateOp;

mod shared;

#[tokio::test]
async fn test_delete_workspace_success() {
    let (app, delegate, ctx, cleanup) = setup_test_app().await;

    let name = random_workspace_name();

    let result = app
        .services()
        .workspace_service
        .create(&ctx, name.clone())
        .await
        .unwrap();

    let workspaces = app.list_workspaces(&ctx, &delegate).await.unwrap().0;

    assert_eq!(workspaces.len(), 1);
    assert_eq!(workspaces[0].id, result.id);
    assert_eq!(workspaces[0].name, name);
    assert_eq!(workspaces[0].abs_path.to_path_buf(), result.abs_path);

    app.delete_workspace(
        &ctx,
        &DeleteWorkspaceInput {
            id: result.id.clone(),
        },
    )
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

#[tokio::test]
async fn test_delete_workspace_opened_in_main_window() {
    let (app, delegate, ctx, cleanup) = setup_test_app().await;

    let name = random_workspace_name();
    let id = app
        .services()
        .workspace_service
        .create(&ctx, name.clone())
        .await
        .unwrap()
        .id;

    app.ensure_main_for_workspace(&ctx, &delegate, id.clone())
        .await
        .unwrap();

    // Try deleting a workspace when there's a main window associated with it

    app.delete_workspace(&ctx, &DeleteWorkspaceInput { id })
        .await
        .unwrap();

    cleanup().await;
}
