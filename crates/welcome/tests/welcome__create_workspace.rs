#![cfg(feature = "integration-tests")]

use moss_testutils::random_name::random_workspace_name;
use sapic_ipc::contracts::welcome::workspace::CreateWorkspaceInput;

use crate::shared::set_up_test_welcome_window;

mod shared;

#[tokio::test]
async fn create_workspace_success() {
    let (welcome_window, delegate, services, ctx, cleanup) = set_up_test_welcome_window().await;

    let workspace_name = random_workspace_name();

    let create_result = welcome_window
        .create_workspace(
            &ctx,
            &CreateWorkspaceInput {
                name: workspace_name.clone(),
            },
        )
        .await
        .unwrap();

    let id = create_result.id;

    let expected_path = delegate.workspaces_dir().join(id.as_str());

    // Check known workspaces
    let list_workspaces = services.workspace_service.workspaces(&ctx).await.unwrap();

    assert_eq!(list_workspaces.len(), 1);
    assert_eq!(list_workspaces[0].name.as_str(), workspace_name);
    assert_eq!(list_workspaces[0].id, id);
    assert_eq!(list_workspaces[0].abs_path, expected_path.into());
    assert!(list_workspaces[0].last_opened_at.is_none());

    cleanup().await;
}

#[tokio::test]
async fn create_workspace_empty_name() {
    let (welcome_window, _delegate, services, ctx, cleanup) = set_up_test_welcome_window().await;

    let workspace_name = "";
    let create_result = welcome_window
        .create_workspace(
            &ctx,
            &CreateWorkspaceInput {
                name: workspace_name.to_string(),
            },
        )
        .await;

    assert!(create_result.is_err());

    // Check no workspace was created
    let list_workspaces = services.workspace_service.workspaces(&ctx).await.unwrap();
    assert_eq!(list_workspaces.len(), 0);

    cleanup().await;
}

#[tokio::test]
async fn create_workspace_same_name() {
    let (welcome_window, _delegate, services, ctx, cleanup) = set_up_test_welcome_window().await;

    let workspace_name = random_workspace_name();

    // Try creating workspaces with the same name twice
    // This should work since both would have different ids
    let first_id = welcome_window
        .create_workspace(
            &ctx,
            &CreateWorkspaceInput {
                name: workspace_name.clone(),
            },
        )
        .await
        .unwrap()
        .id;

    let second_id = welcome_window
        .create_workspace(
            &ctx,
            &CreateWorkspaceInput {
                name: workspace_name.clone(),
            },
        )
        .await
        .unwrap()
        .id;

    assert_ne!(first_id, second_id);

    let list_workspaces = services.workspace_service.workspaces(&ctx).await.unwrap();
    assert_eq!(list_workspaces.len(), 2);

    cleanup().await;
}
