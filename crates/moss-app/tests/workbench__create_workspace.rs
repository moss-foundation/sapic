pub mod shared;

use moss_app::{context::ctxkeys, dirs, models::operations::CreateWorkspaceInput};
use moss_applib::context::Context;
use moss_common::api::OperationError;
use moss_storage::storage::operations::{GetItem, ListByPrefix};
use moss_testutils::{fs_specific::FILENAME_SPECIAL_CHARS, random_name::random_workspace_name};
use moss_workspace::models::types::WorkspaceMode;
use std::{path::Path, sync::Arc};

use crate::shared::{set_up_test_app, workspace_key};

#[tokio::test]
async fn create_workspace_success() {
    let (app, ctx, cleanup, abs_path) = set_up_test_app().await;

    let workspace_name = random_workspace_name();
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

    assert!(expected_path.exists());

    let id = create_output.id;

    // Check active workspace
    let active_workspace = app.workspace().await;
    let (workspace_guard, _context) = active_workspace.as_ref().unwrap();
    let active_workspace_id = ctx
        .value::<ctxkeys::WorkspaceId>()
        .map(|id| id.to_string())
        .unwrap();
    assert_eq!(active_workspace_id, id);
    assert_eq!(workspace_guard.abs_path(), &expected_path);
    assert_eq!(workspace_guard.manifest().await.name, workspace_name);

    // Check known_workspaces
    let list_workspaces = app.list_workspaces(&ctx).await.unwrap();
    assert_eq!(list_workspaces.len(), 1);
    assert_eq!(list_workspaces[0].id, id);
    assert_eq!(list_workspaces[0].display_name, workspace_name);

    // Check database
    let item_store = app.__storage().item_store();
    let _ = GetItem::get(item_store.as_ref(), workspace_key(&id)).unwrap();

    cleanup().await;
}

#[tokio::test]
async fn create_workspace_empty_name() {
    let (app, ctx, cleanup, _abs_path) = set_up_test_app().await;

    let create_result = app
        .create_workspace(
            &ctx,
            &CreateWorkspaceInput {
                name: "".to_string(),
                mode: WorkspaceMode::default(),
                open_on_creation: false,
            },
        )
        .await;

    assert!(matches!(
        create_result,
        Err(OperationError::InvalidInput(_))
    ));

    // Ensure no workspace was created or activated
    let list_workspaces = app.list_workspaces(&ctx).await.unwrap();
    assert!(list_workspaces.is_empty());
    assert!(app.workspace().await.is_none());

    // Check database
    let item_store = app.__storage().item_store();
    let list_result = ListByPrefix::list_by_prefix(item_store.as_ref(), "workspace").unwrap();
    assert_eq!(list_result.len(), 0);

    cleanup().await;
}

#[tokio::test]
async fn create_workspace_same_name() {
    let (app, ctx, cleanup, workspaces_path) = set_up_test_app().await;

    let workspace_name = random_workspace_name();

    // Create first workspace
    let first_result = app
        .create_workspace(
            &ctx,
            &CreateWorkspaceInput {
                name: workspace_name.clone(),
                mode: WorkspaceMode::default(),
                open_on_creation: false,
            },
        )
        .await;
    let first_output = first_result.unwrap();

    let first_path: Arc<Path> = workspaces_path
        .join(dirs::WORKSPACES_DIR)
        .join(&first_output.id.to_string())
        .into();
    assert!(first_path.exists());

    // Check first workspace is in list
    let list_after_first = app.list_workspaces(&ctx).await.unwrap();
    assert_eq!(list_after_first.len(), 1);
    assert_eq!(list_after_first[0].id, first_output.id);
    assert_eq!(list_after_first[0].display_name, workspace_name);

    // Create second workspace with same name
    let second_result = app
        .create_workspace(
            &ctx,
            &CreateWorkspaceInput {
                name: workspace_name.clone(),
                mode: WorkspaceMode::default(),
                open_on_creation: true,
            },
        )
        .await;
    let second_output = second_result.unwrap();

    let second_path: Arc<Path> = workspaces_path
        .join(dirs::WORKSPACES_DIR)
        .join(&second_output.id.to_string())
        .into();
    assert!(second_path.exists());
    assert_ne!(first_output.id, second_output.id);

    // Check active workspace is the second one
    let active_workspace = app.workspace().await;
    let (workspace_guard, _context) = active_workspace.as_ref().unwrap();
    let active_workspace_id = ctx
        .value::<ctxkeys::WorkspaceId>()
        .map(|id| id.to_string())
        .unwrap();
    assert_eq!(active_workspace_id, second_output.id);
    assert_eq!(workspace_guard.abs_path(), &second_path);
    assert_eq!(workspace_guard.manifest().await.name, workspace_name);

    // Check both workspaces are in list
    let list_after_second = app.list_workspaces(&ctx).await.unwrap();
    assert_eq!(list_after_second.len(), 2);

    let listed_first = list_after_second
        .iter()
        .find(|w| w.id == first_output.id)
        .unwrap();
    assert_eq!(listed_first.display_name, workspace_name);

    let listed_second = list_after_second
        .iter()
        .find(|w| w.id == second_output.id)
        .unwrap();
    assert_eq!(listed_second.display_name, workspace_name);

    // Check only second workspace has entry in the databased since it's been opened

    let _global_storage = app.__storage();
    let item_store = app.__storage().item_store();
    let _ = GetItem::get(item_store.as_ref(), workspace_key(&second_output.id)).unwrap();
    assert!(GetItem::get(item_store.as_ref(), workspace_key(&first_output.id)).is_err());

    cleanup().await;
}

#[tokio::test]
async fn create_workspace_special_chars() {
    let (app, ctx, cleanup, workspaces_path) = set_up_test_app().await;

    let base_name = random_workspace_name();
    let mut created_count = 0;

    for special_char in FILENAME_SPECIAL_CHARS.iter() {
        let name = format!("{}{}", base_name, special_char);

        let create_result = app
            .create_workspace(
                &ctx,
                &CreateWorkspaceInput {
                    name: name.clone(),
                    mode: WorkspaceMode::default(),
                    open_on_creation: true,
                },
            )
            .await;
        let create_output = create_result.unwrap();
        created_count += 1;

        let expected_path: Arc<Path> = workspaces_path
            .join(dirs::WORKSPACES_DIR)
            .join(&create_output.id.to_string())
            .into();
        assert!(expected_path.exists());

        // Check active workspace
        let active_workspace = app.workspace().await;
        let (workspace_guard, _context) = active_workspace.as_ref().unwrap();
        let active_workspace_id = ctx
            .value::<ctxkeys::WorkspaceId>()
            .map(|id| id.to_string())
            .unwrap();
        assert_eq!(active_workspace_id, create_output.id);
        assert_eq!(workspace_guard.abs_path(), &expected_path);
        assert_eq!(workspace_guard.manifest().await.name, name);

        // Check workspace is in list
        let list_workspaces = app.list_workspaces(&ctx).await.unwrap();
        assert_eq!(list_workspaces.len(), created_count);

        let matching_workspace = list_workspaces
            .iter()
            .find(|w| w.id == create_output.id)
            .unwrap();
        assert_eq!(matching_workspace.display_name, name);
        // Check database
        let item_store = app.__storage().item_store();
        let _ = GetItem::get(item_store.as_ref(), workspace_key(&create_output.id)).unwrap();
    }

    cleanup().await;
}

#[tokio::test]
async fn create_workspace_not_open_on_creation() {
    let (app, ctx, cleanup, workspaces_path) = set_up_test_app().await;

    let workspace_name = random_workspace_name();
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

    let expected_path: Arc<Path> = workspaces_path
        .join(dirs::WORKSPACES_DIR)
        .join(&create_output.id.to_string())
        .into();
    assert!(expected_path.exists());

    // Check that no workspace is active
    assert!(app.workspace().await.as_ref().is_none());

    // Check workspace is in list
    let list_workspaces = app.list_workspaces(&ctx).await.unwrap();
    assert_eq!(list_workspaces.len(), 1);
    assert_eq!(list_workspaces[0].id, create_output.id);
    assert_eq!(list_workspaces[0].display_name, workspace_name);

    // Check that a database entry is not created for unopened workspace
    let item_store = app.__storage().item_store();
    assert!(GetItem::get(item_store.as_ref(), workspace_key(&create_output.id)).is_err());
    cleanup().await;
}
