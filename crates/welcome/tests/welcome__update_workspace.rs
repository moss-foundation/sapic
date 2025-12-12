#![cfg(feature = "integration-tests")]

use moss_testutils::random_name::random_workspace_name;
use sapic_ipc::contracts::welcome::workspace::{CreateWorkspaceInput, UpdateWorkspaceInput};

use crate::shared::set_up_test_welcome_window;

pub mod shared;

#[tokio::test]
async fn rename_workspace_success() {
    let (welcome_window, _delegate, services, ctx, cleanup) = set_up_test_welcome_window().await;

    let workspace_name = random_workspace_name();
    let new_workspace_name = random_workspace_name();

    let id = welcome_window
        .create_workspace(
            &ctx,
            &CreateWorkspaceInput {
                name: workspace_name.clone(),
            },
        )
        .await
        .unwrap()
        .id;

    welcome_window
        .update_workspace(
            &ctx,
            &UpdateWorkspaceInput {
                id: id.clone(),
                name: Some(new_workspace_name.clone()),
            },
        )
        .await
        .unwrap();

    // Verify the workspace was renamed
    let list_workspaces = services.workspace_service.workspaces(&ctx).await.unwrap();
    assert_eq!(list_workspaces.len(), 1);
    assert_eq!(list_workspaces[0].id, id);
    assert_eq!(list_workspaces[0].name, new_workspace_name);

    cleanup().await;
}

#[tokio::test]
async fn rename_workspace_empty_name() {
    let (welcome_window, _delegate, _services, ctx, cleanup) = set_up_test_welcome_window().await;

    let workspace_name = random_workspace_name();
    let new_workspace_name = "".to_string();

    let id = welcome_window
        .create_workspace(
            &ctx,
            &CreateWorkspaceInput {
                name: workspace_name.clone(),
            },
        )
        .await
        .unwrap()
        .id;

    let update_result = welcome_window
        .update_workspace(
            &ctx,
            &UpdateWorkspaceInput {
                id: id.clone(),
                name: Some(new_workspace_name.clone()),
            },
        )
        .await;

    assert!(update_result.is_err());

    cleanup().await;
}

#[tokio::test]
async fn rename_workspace_same_name() {
    let (welcome_window, _delegate, services, ctx, cleanup) = set_up_test_welcome_window().await;

    let workspace_name = random_workspace_name();
    let new_workspace_name = workspace_name.clone();

    let id = welcome_window
        .create_workspace(
            &ctx,
            &CreateWorkspaceInput {
                name: workspace_name.clone(),
            },
        )
        .await
        .unwrap()
        .id;

    welcome_window
        .update_workspace(
            &ctx,
            &UpdateWorkspaceInput {
                id: id.clone(),
                name: Some(new_workspace_name.clone()),
            },
        )
        .await
        .unwrap();

    // Verify the workspace has the same name
    let list_workspaces = services.workspace_service.workspaces(&ctx).await.unwrap();
    assert_eq!(list_workspaces.len(), 1);
    assert_eq!(list_workspaces[0].id, id);
    assert_eq!(list_workspaces[0].name, new_workspace_name);

    cleanup().await;
}
