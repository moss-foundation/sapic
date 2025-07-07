pub mod shared;

use moss_app::{
    context::ctxkeys,
    dirs,
    models::operations::{CreateWorkspaceInput, OpenWorkspaceInput},
    services::{storage_service::StorageService, workspace_service::WorkspaceService},
    storage::segments::{SEGKEY_LAST_ACTIVE_WORKSPACE, segkey_last_opened_at},
};
use moss_applib::context::Context;
use moss_common::{api::OperationError, new_nanoid_string};
use moss_storage::storage::operations::GetItem;
use moss_testutils::random_name::random_workspace_name;
use moss_workspace::models::types::WorkspaceMode;
use std::{path::Path, sync::Arc};
use tauri::test::MockRuntime;

use crate::shared::set_up_test_app;

#[tokio::test]
async fn open_workspace_success() {
    let (app, ctx, _services, cleanup, abs_path) = set_up_test_app().await;

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
                id: create_output.id.clone(),
            },
        )
        .await;
    let open_output = open_result.unwrap();

    assert_eq!(open_output.id, create_output.id);
    assert_eq!(open_output.abs_path, expected_path);

    // Check active workspace
    let workspace_service = _services.get::<WorkspaceService<MockRuntime>>();
    let storage_service = _services.get::<StorageService>();

    // Check workspace is open
    assert!(workspace_service.is_workspace_open().await.is_some());
    let active_workspace_id_from_service = workspace_service.is_workspace_open().await.unwrap();
    assert_eq!(
        active_workspace_id_from_service.to_string(),
        create_output.id
    );

    // Check workspace ID in context
    let active_workspace_id = ctx.value::<ctxkeys::WorkspaceId>().unwrap();
    assert_eq!(active_workspace_id.to_string(), create_output.id);

    // Check entry in the database - verify last opened at timestamp is saved
    let item_store = storage_service.__storage().item_store();
    let _ = GetItem::get(
        item_store.as_ref(),
        segkey_last_opened_at(&active_workspace_id),
    )
    .unwrap();

    // Check that last active workspace is set in database
    let last_active_workspace = GetItem::get(
        item_store.as_ref(),
        SEGKEY_LAST_ACTIVE_WORKSPACE.to_segkey_buf(),
    )
    .unwrap();
    let last_active_workspace_id: String = last_active_workspace.deserialize().unwrap();
    assert_eq!(last_active_workspace_id, create_output.id.to_string());

    cleanup().await;
}

#[tokio::test]
async fn open_workspace_already_opened() {
    let (app, ctx, _services, cleanup, _abs_path) = set_up_test_app().await;

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

    // Verify workspace is currently open
    let active_workspace_id = ctx
        .value::<ctxkeys::WorkspaceId>()
        .map(|id| id.to_string())
        .unwrap();
    assert_eq!(active_workspace_id, create_output.id);

    // Try to open the same workspace again - should fail
    let open_result = app
        .open_workspace(
            &ctx,
            &OpenWorkspaceInput {
                id: create_output.id.clone(),
            },
        )
        .await;

    // Opening an already opened workspace should return an error
    assert!(open_result.is_err());

    // Active workspace should remain unchanged
    let active_workspace_id = ctx
        .value::<ctxkeys::WorkspaceId>()
        .map(|id| id.to_string())
        .unwrap();
    assert_eq!(active_workspace_id, create_output.id);

    cleanup().await;
}

#[tokio::test]
async fn open_workspace_switch_between_workspaces() {
    let (app, ctx, _services, cleanup, _abs_path) = set_up_test_app().await;

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
                id: create_output1.id.clone(),
            },
        )
        .await
        .unwrap();
    assert_eq!(open_result1.id, create_output1.id);

    // Check first workspace is active
    let active_workspace_id = ctx
        .value::<ctxkeys::WorkspaceId>()
        .map(|id| id.to_string())
        .unwrap();
    assert_eq!(active_workspace_id, create_output1.id);

    // Open second workspace (should replace first)
    let open_result2 = app
        .open_workspace(
            &ctx,
            &OpenWorkspaceInput {
                id: create_output2.id.clone(),
            },
        )
        .await
        .unwrap();
    assert_eq!(open_result2.id, create_output2.id);

    // Check second workspace is now active
    let active_workspace_id = ctx
        .value::<ctxkeys::WorkspaceId>()
        .map(|id| id.to_string())
        .unwrap();
    assert_eq!(active_workspace_id, create_output2.id);

    // Open first workspace again
    let open_result1_again = app
        .open_workspace(
            &ctx,
            &OpenWorkspaceInput {
                id: create_output1.id.clone(),
            },
        )
        .await
        .unwrap();
    assert_eq!(open_result1_again.id, create_output1.id);

    // Check first workspace is active again
    let active_workspace_id = ctx
        .value::<ctxkeys::WorkspaceId>()
        .map(|id| id.to_string())
        .unwrap();
    assert_eq!(active_workspace_id, create_output1.id);

    // Check that last active workspace is set correctly in database (first workspace)
    let storage_service = _services.get::<StorageService>();
    let item_store = storage_service.__storage().item_store();
    let last_active_workspace = GetItem::get(
        item_store.as_ref(),
        SEGKEY_LAST_ACTIVE_WORKSPACE.to_segkey_buf(),
    )
    .unwrap();
    let last_active_workspace_id: String = last_active_workspace.deserialize().unwrap();
    assert_eq!(last_active_workspace_id, create_output1.id.to_string());

    cleanup().await;
}

#[tokio::test]
async fn open_workspace_nonexistent() {
    let (app, ctx, _services, cleanup, _abs_path) = set_up_test_app().await;

    let nonexistent_id = new_nanoid_string();

    let open_result = app
        .open_workspace(&ctx, &OpenWorkspaceInput { id: nonexistent_id })
        .await;

    assert!(open_result.is_err());
    assert!(matches!(open_result, Err(OperationError::NotFound(_))));

    cleanup().await;
}

#[tokio::test]
async fn open_workspace_filesystem_does_not_exist() {
    let (app, ctx, _services, cleanup, abs_path) = set_up_test_app().await;

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
    // When filesystem doesn't exist, it returns Internal error instead of NotFound
    assert!(matches!(open_result, Err(OperationError::Internal(_))));

    cleanup().await;
}
