pub mod shared;

use moss_app::{
    context::ctxkeys,
    dirs,
    models::operations::{CreateWorkspaceInput, OpenWorkspaceInput},
};
use moss_applib::context::Context;
use moss_common::api::OperationError;
use moss_storage::storage::operations::GetItem;
use moss_testutils::random_name::random_workspace_name;
use moss_workspace::models::types::WorkspaceMode;
use std::{path::Path, sync::Arc};
use uuid::Uuid;

use crate::shared::{set_up_test_app, workspace_key};

#[tokio::test]
async fn open_workspace_success() {
    let (app, ctx, cleanup, abs_path) = set_up_test_app().await;

    let workspace_name = random_workspace_name();

    // Create workspace WITHOUT opening it first
    let create_result = app
        .create_workspace(
            &ctx,
            &CreateWorkspaceInput {
                name: workspace_name.clone(),
                mode: WorkspaceMode::default(),
                open_on_creation: false,
            },
        )
        .await;
    let create_output = create_result.unwrap();

    let expected_path: Arc<Path> = abs_path
        .join(dirs::WORKSPACES_DIR)
        .join(&create_output.id.to_string())
        .into();
    assert!(expected_path.exists());

    // Open the workspace
    let open_result = app
        .open_workspace(
            &ctx,
            &OpenWorkspaceInput {
                id: create_output.id,
            },
        )
        .await;
    let open_output = open_result.unwrap();

    assert_eq!(open_output.id, create_output.id);
    assert_eq!(open_output.abs_path, expected_path);

    // Check active workspace
    let active_workspace = app.workspace().await;
    let (workspace_guard, _context) = active_workspace.as_ref().unwrap();
    let active_workspace_id = ctx.value::<ctxkeys::WorkspaceId>().map(|id| **id).unwrap();
    assert_eq!(active_workspace_id, create_output.id);
    assert_eq!(workspace_guard.abs_path(), &expected_path);
    assert_eq!(workspace_guard.manifest().await.name, workspace_name);

    // Check entry in the database
    let item_store = app.__storage().item_store();
    let _ = GetItem::get(item_store.as_ref(), workspace_key(create_output.id)).unwrap();

    cleanup().await;
}

#[tokio::test]
async fn open_workspace_already_opened() {
    let (app, ctx, cleanup, abs_path) = set_up_test_app().await;

    let workspace_name = random_workspace_name();

    // Create and open workspace
    let create_result = app
        .create_workspace(
            &ctx,
            &CreateWorkspaceInput {
                name: workspace_name.clone(),
                mode: WorkspaceMode::default(),
                open_on_creation: true,
            },
        )
        .await;
    let create_output = create_result.unwrap();

    let expected_path: Arc<Path> = abs_path
        .join(dirs::WORKSPACES_DIR)
        .join(&create_output.id.to_string())
        .into();

    // Try to open the same workspace again
    let open_result = app
        .open_workspace(
            &ctx,
            &OpenWorkspaceInput {
                id: create_output.id,
            },
        )
        .await;
    let open_output = open_result.unwrap();

    assert_eq!(open_output.id, create_output.id);
    assert_eq!(open_output.abs_path, expected_path);

    // Check active workspace is still the same
    let active_workspace_id = ctx.value::<ctxkeys::WorkspaceId>().map(|id| **id).unwrap();
    assert_eq!(active_workspace_id, create_output.id);

    cleanup().await;
}

#[tokio::test]
async fn open_workspace_switch_between_workspaces() {
    let (app, ctx, cleanup, _abs_path) = set_up_test_app().await;

    // Create first workspace
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

    // Create second workspace
    let workspace_name2 = random_workspace_name();
    let create_output2 = app
        .create_workspace(
            &ctx,
            &CreateWorkspaceInput {
                name: workspace_name2.clone(),
                mode: WorkspaceMode::default(),
                open_on_creation: false,
            },
        )
        .await
        .unwrap();

    // Open first workspace
    let open_result1 = app
        .open_workspace(
            &ctx,
            &OpenWorkspaceInput {
                id: create_output1.id,
            },
        )
        .await
        .unwrap();
    assert_eq!(open_result1.id, create_output1.id);

    // Check first workspace is active
    let active_workspace_id = ctx.value::<ctxkeys::WorkspaceId>().map(|id| **id).unwrap();
    assert_eq!(active_workspace_id, create_output1.id);

    // Open second workspace (should replace first)
    let open_result2 = app
        .open_workspace(
            &ctx,
            &OpenWorkspaceInput {
                id: create_output2.id,
            },
        )
        .await
        .unwrap();
    assert_eq!(open_result2.id, create_output2.id);

    // Check second workspace is now active
    let active_workspace_id = ctx.value::<ctxkeys::WorkspaceId>().map(|id| **id).unwrap();
    assert_eq!(active_workspace_id, create_output2.id);

    // Open first workspace again
    let open_result1_again = app
        .open_workspace(
            &ctx,
            &OpenWorkspaceInput {
                id: create_output1.id,
            },
        )
        .await
        .unwrap();
    assert_eq!(open_result1_again.id, create_output1.id);

    // Check first workspace is active again
    let active_workspace_id = ctx.value::<ctxkeys::WorkspaceId>().map(|id| **id).unwrap();
    assert_eq!(active_workspace_id, create_output1.id);

    cleanup().await;
}

#[tokio::test]
async fn open_workspace_nonexistent() {
    let (app, ctx, cleanup, _abs_path) = set_up_test_app().await;

    let nonexistent_id = Uuid::new_v4();

    let open_result = app
        .open_workspace(&ctx, &OpenWorkspaceInput { id: nonexistent_id })
        .await;

    assert!(open_result.is_err());
    assert!(matches!(open_result, Err(OperationError::NotFound(_))));

    cleanup().await;
}

#[tokio::test]
async fn open_workspace_filesystem_does_not_exist() {
    let (app, ctx, cleanup, abs_path) = set_up_test_app().await;

    let workspace_name = random_workspace_name();

    // Create workspace
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
    tokio::fs::remove_dir_all(&workspace_path).await.unwrap();
    assert!(!workspace_path.exists());

    // Try to open the workspace (should fail)
    let open_result = app
        .open_workspace(
            &ctx,
            &OpenWorkspaceInput {
                id: create_output.id,
            },
        )
        .await;

    assert!(open_result.is_err());
    assert!(matches!(open_result, Err(OperationError::NotFound(_))));

    cleanup().await;
}
