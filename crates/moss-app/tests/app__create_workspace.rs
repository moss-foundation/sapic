#![cfg(feature = "integration-tests")]

pub mod shared;

use moss_app::{
    dirs,
    models::{operations::CreateWorkspaceInput, primitives::WorkspaceId},
    storage::segments::{SEGKEY_LAST_ACTIVE_WORKSPACE, segkey_last_opened_at},
};

use moss_storage::storage::operations::{GetItem, ListByPrefix};
use moss_testutils::{fs_specific::FILENAME_SPECIAL_CHARS, random_name::random_workspace_name};
use moss_workspace::models::primitives::WorkspaceMode;
use std::{path::Path, sync::Arc};

use crate::shared::set_up_test_app;

#[tokio::test]
async fn create_workspace_success() {
    let (app, ctx, cleanup) = set_up_test_app().await;

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
    let expected_path: Arc<Path> = app
        .app_dir()
        .join(dirs::WORKSPACES_DIR)
        .join(&create_output.id.to_string())
        .into();

    assert!(expected_path.exists());

    let id = create_output.id;

    // Check active workspace
    let maybe_active_workspace = app.workspace().await;
    assert!(maybe_active_workspace.is_some());
    let active_workspace_id = maybe_active_workspace.unwrap().id();
    assert_eq!(active_workspace_id, id);

    // Check known_workspaces
    let list_workspaces = app.list_workspaces(&ctx).await.unwrap();
    assert_eq!(list_workspaces.len(), 1);
    assert_eq!(list_workspaces[0].id, id);
    assert_eq!(list_workspaces[0].name, workspace_name);

    // Check database - verify last opened at timestamp is saved
    let item_store = app.db().item_store();
    let _ = GetItem::get(
        item_store.as_ref(),
        &ctx,
        segkey_last_opened_at(&id.clone().into()),
    )
    .await
    .unwrap();

    // Check that last active workspace is set in database
    let last_active_workspace = GetItem::get(
        item_store.as_ref(),
        &ctx,
        SEGKEY_LAST_ACTIVE_WORKSPACE.to_segkey_buf(),
    )
    .await
    .unwrap();
    let last_active_workspace_id: String = last_active_workspace.deserialize().unwrap();
    assert_eq!(last_active_workspace_id, id.to_string());

    cleanup().await;
}

#[tokio::test]
async fn create_workspace_empty_name() {
    let (app, ctx, cleanup) = set_up_test_app().await;

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

    assert!(create_result.is_err());

    // Ensure no workspace was created or activated
    let list_workspaces = app.list_workspaces(&ctx).await.unwrap();
    assert!(list_workspaces.is_empty());
    assert!(app.workspace().await.is_none());

    // Check database
    let item_store = app.db().item_store();
    let list_result = ListByPrefix::list_by_prefix(item_store.as_ref(), &ctx, "workspace")
        .await
        .unwrap();
    assert_eq!(list_result.len(), 0);

    cleanup().await;
}

#[tokio::test]
async fn create_workspace_same_name() {
    let (app, ctx, cleanup) = set_up_test_app().await;

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

    let first_path: Arc<Path> = app
        .app_dir()
        .join(dirs::WORKSPACES_DIR)
        .join(&first_output.id.to_string())
        .into();
    assert!(first_path.exists());

    // Check first workspace is in list
    let list_after_first = app.list_workspaces(&ctx).await.unwrap();
    assert_eq!(list_after_first.len(), 1);
    assert_eq!(list_after_first[0].id, first_output.id);
    assert_eq!(list_after_first[0].name, workspace_name);

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

    let second_path: Arc<Path> = app
        .app_dir()
        .join(dirs::WORKSPACES_DIR)
        .join(&second_output.id.to_string())
        .into();
    assert!(second_path.exists());
    assert_ne!(first_output.id, second_output.id);

    // Check active workspace is the second one
    let maybe_active_workspace = app.workspace().await;
    assert!(maybe_active_workspace.is_some());
    let active_workspace_id = maybe_active_workspace.unwrap().id();
    assert_eq!(active_workspace_id, second_output.id);

    // Check both workspaces are in list
    let list_after_second = app.list_workspaces(&ctx).await.unwrap();
    assert_eq!(list_after_second.len(), 2);

    let listed_first = list_after_second
        .iter()
        .find(|w| w.id == first_output.id)
        .unwrap();
    assert_eq!(listed_first.name, workspace_name);

    let listed_second = list_after_second
        .iter()
        .find(|w| w.id == second_output.id)
        .unwrap();
    assert_eq!(listed_second.name, workspace_name);

    // Check only second workspace has entry in the database since it's been opened

    let item_store = app.db().item_store();
    let first_id: WorkspaceId = first_output.id;
    let second_id: WorkspaceId = second_output.id;
    let _ = GetItem::get(
        item_store.as_ref(),
        &ctx,
        segkey_last_opened_at(&second_id.clone().into()),
    )
    .await
    .unwrap();

    assert!(
        GetItem::get(
            item_store.as_ref(),
            &ctx,
            segkey_last_opened_at(&first_id.clone().into())
        )
        .await
        .is_err()
    );

    // Check that last active workspace is set in database (second workspace)
    let last_active_workspace = GetItem::get(
        item_store.as_ref(),
        &ctx,
        SEGKEY_LAST_ACTIVE_WORKSPACE.to_segkey_buf(),
    )
    .await
    .unwrap();
    let last_active_workspace_id: WorkspaceId = last_active_workspace.deserialize().unwrap();
    assert_eq!(last_active_workspace_id, second_id);

    cleanup().await;
}

#[tokio::test]
async fn create_workspace_special_chars() {
    let (app, ctx, cleanup) = set_up_test_app().await;

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

        let expected_path: Arc<Path> = app
            .app_dir()
            .join(dirs::WORKSPACES_DIR)
            .join(&create_output.id.to_string())
            .into();
        assert!(expected_path.exists());

        // Check active workspace
        let maybe_active_workspace = app.workspace().await;
        assert!(maybe_active_workspace.is_some());
        let active_workspace_id = maybe_active_workspace.unwrap().id();
        assert_eq!(active_workspace_id, create_output.id);

        // Check workspace is in list
        let list_workspaces = app.list_workspaces(&ctx).await.unwrap();
        assert_eq!(list_workspaces.len(), created_count);

        let matching_workspace = list_workspaces
            .iter()
            .find(|w| w.id == create_output.id)
            .unwrap();
        assert_eq!(matching_workspace.name, name);

        let id: WorkspaceId = create_output.id;
        // Check database - verify last opened at timestamp is saved
        let item_store = app.db().item_store();
        let _ = GetItem::get(
            item_store.as_ref(),
            &ctx,
            segkey_last_opened_at(&id.clone().into()),
        )
        .await
        .unwrap();

        // Check that last active workspace is set in database
        let last_active_workspace = GetItem::get(
            item_store.as_ref(),
            &ctx,
            SEGKEY_LAST_ACTIVE_WORKSPACE.to_segkey_buf(),
        )
        .await
        .unwrap();
        let last_active_workspace_id: WorkspaceId = last_active_workspace.deserialize().unwrap();
        assert_eq!(last_active_workspace_id, id);
    }

    cleanup().await;
}

#[tokio::test]
async fn create_workspace_not_open_on_creation() {
    let (app, ctx, cleanup) = set_up_test_app().await;

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

    let expected_path: Arc<Path> = app
        .app_dir()
        .join(dirs::WORKSPACES_DIR)
        .join(&create_output.id.to_string())
        .into();
    assert!(expected_path.exists());

    // Check that no workspace is active
    assert!(app.workspace().await.is_none());

    // Check workspace is in list
    let list_workspaces = app.list_workspaces(&ctx).await.unwrap();
    assert_eq!(list_workspaces.len(), 1);
    assert_eq!(list_workspaces[0].id, create_output.id);
    assert_eq!(list_workspaces[0].name, workspace_name);

    // Check that a database entry is not created for unopened workspace
    let item_store = app.db().item_store();
    let id: WorkspaceId = create_output.id;
    assert!(
        GetItem::get(
            item_store.as_ref(),
            &ctx,
            segkey_last_opened_at(&id.clone().into())
        )
        .await
        .is_err()
    );

    // Check that last active workspace is not set in database
    assert!(
        GetItem::get(
            item_store.as_ref(),
            &ctx,
            SEGKEY_LAST_ACTIVE_WORKSPACE.to_segkey_buf(),
        )
        .await
        .is_err()
    );
    cleanup().await;
}
