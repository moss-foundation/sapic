pub mod shared;

use moss_app::{
    context::ctxkeys,
    models::operations::{CloseWorkspaceInput, CreateWorkspaceInput, OpenWorkspaceInput},
};
use moss_applib::context::Context;
use moss_common::{api::OperationError, nanoid::new_nanoid};
use moss_testutils::random_name::random_workspace_name;
use moss_workspace::models::types::WorkspaceMode;

use crate::shared::set_up_test_app;

#[tokio::test]
async fn close_workspace_success() {
    let (app, ctx, cleanup, _abs_path) = set_up_test_app().await;

    // Create and open a workspace
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

    // Close the workspace
    let close_result = app
        .close_workspace(
            &ctx,
            &CloseWorkspaceInput {
                id: create_output.id.clone(),
            },
        )
        .await;

    assert!(close_result.is_ok());
    let close_output = close_result.unwrap();
    assert_eq!(close_output.id, create_output.id);

    // Check that no workspace is active
    assert!(app.workspace().await.is_none());

    cleanup().await;
}

#[tokio::test]
async fn close_workspace_not_open() {
    let (app, ctx, cleanup, _abs_path) = set_up_test_app().await;

    // Create a workspace without opening it
    let workspace_name = random_workspace_name();
    let create_output = app
        .create_workspace(
            &ctx,
            &CreateWorkspaceInput {
                name: workspace_name.clone(),
                mode: WorkspaceMode::default(),
                open_on_creation: false,
            },
        )
        .await
        .unwrap();

    // Attempt to close the workspace (should fail because it's not open)
    let close_result = app
        .close_workspace(
            &ctx,
            &CloseWorkspaceInput {
                id: create_output.id,
            },
        )
        .await;

    assert!(close_result.is_err());
    assert!(matches!(
        close_result,
        Err(OperationError::FailedPrecondition(_))
    ));

    cleanup().await;
}

#[tokio::test]
async fn close_workspace_after_another_opened() {
    let (app, ctx, cleanup, _abs_path) = set_up_test_app().await;

    // Create and open first workspace
    let workspace_name1 = random_workspace_name();
    let create_output1 = app
        .create_workspace(
            &ctx,
            &CreateWorkspaceInput {
                name: workspace_name1.clone(),
                mode: WorkspaceMode::default(),
                open_on_creation: false,
            },
        )
        .await
        .unwrap();

    // Open first workspace
    app.open_workspace(
        &ctx,
        &OpenWorkspaceInput {
            id: create_output1.id.clone(),
        },
    )
    .await
    .unwrap();

    // Create and open second workspace
    let workspace_name2 = random_workspace_name();
    let create_output2 = app
        .create_workspace(
            &ctx,
            &CreateWorkspaceInput {
                name: workspace_name2.clone(),
                mode: WorkspaceMode::default(),
                open_on_creation: true,
            },
        )
        .await
        .unwrap();

    // Check that the second workspace is active

    let maybe_active_id = ctx.value::<ctxkeys::WorkspaceId>().map(|id| id.clone());
    assert!(maybe_active_id.is_some());
    let active_id = maybe_active_id.unwrap();

    assert_eq!(active_id.to_string(), create_output2.id);

    // Attempt to close the first workspace (should fail because it's not active)
    let close_result1 = app
        .close_workspace(
            &ctx,
            &CloseWorkspaceInput {
                id: create_output1.id.clone(),
            },
        )
        .await;

    assert!(close_result1.is_err());
    assert!(matches!(
        close_result1,
        Err(OperationError::InvalidInput(_))
    ));

    // Close the second workspace (should succeed)
    let close_result2 = app
        .close_workspace(
            &ctx,
            &CloseWorkspaceInput {
                id: create_output2.id.clone(),
            },
        )
        .await;

    assert!(close_result2.is_ok());
    let close_output = close_result2.unwrap();
    assert_eq!(close_output.id, create_output2.id);

    // Check that no workspace is active
    assert!(app.workspace().await.is_none());

    cleanup().await;
}

#[tokio::test]
async fn close_workspace_nonexistent() {
    let (app, ctx, cleanup, _abs_path) = set_up_test_app().await;

    let nonexistent_id = new_nanoid();

    let close_result = app
        .close_workspace(&ctx, &CloseWorkspaceInput { id: nonexistent_id })
        .await;

    assert!(close_result.is_err());
    assert!(matches!(
        close_result,
        Err(OperationError::FailedPrecondition(_))
    ));

    cleanup().await;
}

#[tokio::test]
async fn close_workspace_from_different_session() {
    let (app, ctx, cleanup, _abs_path) = set_up_test_app().await;

    // Create a workspace
    let workspace_name = random_workspace_name();
    let create_output = app
        .create_workspace(
            &ctx,
            &CreateWorkspaceInput {
                name: workspace_name.clone(),
                mode: WorkspaceMode::default(),
                open_on_creation: false,
            },
        )
        .await
        .unwrap();

    // Open the workspace
    app.open_workspace(
        &ctx,
        &OpenWorkspaceInput {
            id: create_output.id,
        },
    )
    .await
    .unwrap();

    // Try to close a workspace with wrong id
    let wrong_id = new_nanoid();
    let close_result = app
        .close_workspace(&ctx, &CloseWorkspaceInput { id: wrong_id })
        .await;

    assert!(close_result.is_err());
    assert!(matches!(close_result, Err(OperationError::InvalidInput(_))));

    cleanup().await;
}
