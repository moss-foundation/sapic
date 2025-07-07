pub mod shared;

use crate::shared::set_up_test_app;
use moss_app::{
    context::ctxkeys,
    dirs,
    models::operations::{CreateWorkspaceInput, DeleteWorkspaceInput},
};
use moss_applib::context::Context;
use moss_common::{api::OperationError, new_nanoid_string};
use moss_fs::{FileSystem, RealFileSystem};
use moss_testutils::random_name::random_workspace_name;
use moss_workspace::models::types::WorkspaceMode;
use std::{path::Path, sync::Arc};

#[tokio::test]
async fn delete_workspace_success() {
    let (app, ctx, _services, cleanup, abs_path) = set_up_test_app().await;

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

    let workspace_path: Arc<Path> = abs_path
        .join(dirs::WORKSPACES_DIR)
        .join(&create_output.id.to_string())
        .into();
    assert!(workspace_path.exists());

    // Verify workspace is in list
    let list_workspaces = app.list_workspaces(&ctx).await.unwrap();
    assert_eq!(list_workspaces.len(), 1);
    assert_eq!(list_workspaces[0].id, create_output.id);

    // Delete the workspace
    let delete_result = app
        .delete_workspace(
            &ctx,
            &DeleteWorkspaceInput {
                id: create_output.id,
            },
        )
        .await;

    assert!(delete_result.is_ok());

    // Verify workspace directory was deleted
    assert!(!workspace_path.exists());

    // Verify workspace is not in list
    let list_workspaces = app.list_workspaces(&ctx).await.unwrap();
    assert!(list_workspaces.is_empty());

    cleanup().await;
}

#[tokio::test]
async fn delete_workspace_filesystem_only() {
    let (app, ctx, _services, cleanup, abs_path) = set_up_test_app().await;

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

    let workspace_path: Arc<Path> = abs_path
        .join(dirs::WORKSPACES_DIR)
        .join(&create_output.id.to_string())
        .into();
    assert!(workspace_path.exists());

    // Delete workspace
    let delete_result = app
        .delete_workspace(
            &ctx,
            &DeleteWorkspaceInput {
                id: create_output.id,
            },
        )
        .await;

    assert!(delete_result.is_ok());

    // Verify workspace directory was deleted
    assert!(!workspace_path.exists());

    // Verify workspace is not in list
    let list_workspaces = app.list_workspaces(&ctx).await.unwrap();
    assert!(list_workspaces.is_empty());

    cleanup().await;
}

#[tokio::test]
async fn delete_workspace_opened() {
    let (app, ctx, _services, cleanup, abs_path) = set_up_test_app().await;

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

    let workspace_path: Arc<Path> = abs_path
        .join(dirs::WORKSPACES_DIR)
        .join(&create_output.id.to_string())
        .into();
    assert!(workspace_path.exists());

    // Verify workspace is active
    let active_workspace_id = ctx
        .value::<ctxkeys::WorkspaceId>()
        .map(|id| id.to_string())
        .unwrap();
    assert_eq!(active_workspace_id, create_output.id);

    // Delete the workspace (should succeed and deactivate it)
    let delete_result = app
        .delete_workspace(
            &ctx,
            &DeleteWorkspaceInput {
                id: create_output.id,
            },
        )
        .await;

    assert!(delete_result.is_ok());

    // Verify workspace directory was deleted
    assert!(!workspace_path.exists());

    // Verify workspace is not in list
    let list_workspaces = app.list_workspaces(&ctx).await.unwrap();
    assert!(list_workspaces.is_empty());

    // Verify that no workspace is active after deletion
    assert!(ctx.value::<ctxkeys::WorkspaceId>().is_none());

    cleanup().await;
}

#[tokio::test]
async fn delete_workspace_nonexistent() {
    let (app, ctx, _services, cleanup, _abs_path) = set_up_test_app().await;

    let nonexistent_id = new_nanoid_string();

    let delete_result = app
        .delete_workspace(&ctx, &DeleteWorkspaceInput { id: nonexistent_id })
        .await;

    assert!(delete_result.is_err());
    assert!(matches!(delete_result, Err(OperationError::NotFound(_))));

    cleanup().await;
}

#[tokio::test]
async fn delete_workspace_filesystem_does_not_exist() {
    let (app, ctx, _services, cleanup, abs_path) = set_up_test_app().await;

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

    // Manually delete the filesystem directory
    let workspace_path: Arc<Path> = abs_path
        .join(dirs::WORKSPACES_DIR)
        .join(&create_output.id.to_string())
        .into();
    let fs = RealFileSystem::new();
    fs.remove_dir(
        &workspace_path,
        moss_fs::RemoveOptions {
            recursive: true,
            ignore_if_not_exists: false,
        },
    )
    .await
    .unwrap();
    assert!(!workspace_path.exists());

    // Delete the workspace (should still succeed)
    let delete_result = app
        .delete_workspace(
            &ctx,
            &DeleteWorkspaceInput {
                id: create_output.id,
            },
        )
        .await;

    assert!(delete_result.is_ok());

    // Verify workspace is not in list
    let list_workspaces = app.list_workspaces(&ctx).await.unwrap();
    assert!(list_workspaces.is_empty());

    cleanup().await;
}
