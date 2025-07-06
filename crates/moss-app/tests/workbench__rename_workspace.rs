pub mod shared;

use moss_app::models::operations::{CreateWorkspaceInput, UpdateWorkspaceInput};
use moss_common::api::OperationError;
use moss_testutils::random_name::random_workspace_name;
use moss_workspace::models::types::WorkspaceMode;

use crate::shared::set_up_test_app;

#[tokio::test]
async fn rename_workspace_success() {
    let (app, ctx, _services, cleanup, _abs_path) = set_up_test_app().await;

    let workspace_name = random_workspace_name();
    let create_output = app
        .create_workspace(
            &ctx,
            &CreateWorkspaceInput {
                name: workspace_name.clone(),
                mode: WorkspaceMode::default(),
                open_on_creation: true,
            },
        )
        .await
        .unwrap();

    // Verify initial name
    let list_workspaces = app.list_workspaces(&ctx).await.unwrap();
    assert_eq!(list_workspaces.len(), 1);
    assert_eq!(list_workspaces[0].id, create_output.id);
    assert_eq!(list_workspaces[0].name, workspace_name);

    // Rename the workspace
    let new_name = random_workspace_name();
    let rename_result = app
        .update_workspace(
            &ctx,
            &UpdateWorkspaceInput {
                name: Some(new_name.clone()),
            },
        )
        .await;

    assert!(rename_result.is_ok());

    // Verify the workspace was renamed
    let list_workspaces = app.list_workspaces(&ctx).await.unwrap();
    assert_eq!(list_workspaces.len(), 1);
    assert_eq!(list_workspaces[0].id, create_output.id);
    assert_eq!(list_workspaces[0].name, new_name);

    cleanup().await;
}

#[tokio::test]
async fn rename_workspace_empty_name() {
    let (app, ctx, _services, cleanup, _abs_path) = set_up_test_app().await;

    let workspace_name = random_workspace_name();
    let _create_output = app
        .create_workspace(
            &ctx,
            &CreateWorkspaceInput {
                name: workspace_name.clone(),
                mode: WorkspaceMode::default(),
                open_on_creation: true,
            },
        )
        .await
        .unwrap();

    // Try to rename with empty name
    let rename_result = app
        .update_workspace(
            &ctx,
            &UpdateWorkspaceInput {
                name: Some("".to_string()),
            },
        )
        .await;

    assert!(rename_result.is_err());
    assert!(matches!(
        rename_result,
        Err(OperationError::InvalidInput(_))
    ));

    // Verify workspace name didn't change
    let list_workspaces = app.list_workspaces(&ctx).await.unwrap();
    assert_eq!(list_workspaces.len(), 1);
    assert_eq!(list_workspaces[0].name, workspace_name);

    cleanup().await;
}

#[tokio::test]
async fn rename_workspace_same_name() {
    let (app, ctx, _services, cleanup, _abs_path) = set_up_test_app().await;

    let workspace_name = random_workspace_name();
    let create_output = app
        .create_workspace(
            &ctx,
            &CreateWorkspaceInput {
                name: workspace_name.clone(),
                mode: WorkspaceMode::default(),
                open_on_creation: true,
            },
        )
        .await
        .unwrap();

    // Rename with the same name
    let rename_result = app
        .update_workspace(
            &ctx,
            &UpdateWorkspaceInput {
                name: Some(workspace_name.clone()),
            },
        )
        .await;

    assert!(rename_result.is_ok());

    // Verify workspace is still there with same name
    let list_workspaces = app.list_workspaces(&ctx).await.unwrap();
    assert_eq!(list_workspaces.len(), 1);
    assert_eq!(list_workspaces[0].id, create_output.id);
    assert_eq!(list_workspaces[0].name, workspace_name);

    cleanup().await;
}

#[tokio::test]
async fn rename_workspace_no_active_workspace() {
    let (app, ctx, _services, cleanup, _abs_path) = set_up_test_app().await;

    // Try to rename when no workspace is active
    let rename_result = app
        .update_workspace(
            &ctx,
            &UpdateWorkspaceInput {
                name: Some(random_workspace_name()),
            },
        )
        .await;

    assert!(rename_result.is_err());
    assert!(matches!(
        rename_result,
        Err(OperationError::FailedPrecondition(_))
    ));

    cleanup().await;
}
