mod shared;

use moss_workspace::models::operations::UpdateStateInput;
use moss_workspace::models::types::{PanelPartState, SidebarPartState};
use shared::create_simple_editor_state;

use crate::shared::setup_test_workspace;

#[tokio::test]
async fn update_state_editor_part() {
    let (workspace_path, workspace) = setup_test_workspace().await;

    let editor_state = create_simple_editor_state();

    let result = workspace
        .update_state(UpdateStateInput::UpdateEditorPartState(
            editor_state.clone(),
        ))
        .await;

    assert!(result.is_ok());

    // Verify the state was stored correctly
    let state = workspace.describe_state().await.unwrap();
    assert!(state.editor.is_some());
    assert_eq!(state.editor.unwrap(), editor_state);

    // Clean up
    {
        tokio::fs::remove_dir_all(workspace_path).await.unwrap();
    }
}

#[tokio::test]
async fn update_state_sidebar_part() {
    let (workspace_path, workspace) = setup_test_workspace().await;

    let sidebar_state = SidebarPartState {
        preferred_size: 250,
        is_visible: true,
    };

    let result = workspace
        .update_state(UpdateStateInput::UpdateSidebarPartState(
            sidebar_state.clone(),
        ))
        .await;

    assert!(result.is_ok());

    // Verify the state was stored correctly
    let state = workspace.describe_state().await.unwrap();
    assert!(state.sidebar.is_some());
    assert_eq!(state.sidebar.unwrap(), sidebar_state);

    // Clean up
    {
        tokio::fs::remove_dir_all(workspace_path).await.unwrap();
    }
}

#[tokio::test]
async fn update_state_panel_part() {
    let (workspace_path, workspace) = setup_test_workspace().await;

    let panel_state = PanelPartState {
        preferred_size: 200,
        is_visible: false,
    };

    let result = workspace
        .update_state(UpdateStateInput::UpdatePanelPartState(panel_state.clone()))
        .await;

    assert!(result.is_ok());

    // Verify the state was stored correctly
    let state = workspace.describe_state().await.unwrap();
    assert!(state.panel.is_some());
    assert_eq!(state.panel.unwrap(), panel_state);

    // Clean up
    {
        tokio::fs::remove_dir_all(workspace_path).await.unwrap();
    }
}

#[tokio::test]
async fn update_state_multiple_updates() {
    let (workspace_path, workspace) = setup_test_workspace().await;

    // Initial states
    let editor_state = create_simple_editor_state();
    let sidebar_state = SidebarPartState {
        preferred_size: 250,
        is_visible: true,
    };
    let panel_state = PanelPartState {
        preferred_size: 200,
        is_visible: false,
    };

    // Update all states
    workspace
        .update_state(UpdateStateInput::UpdateEditorPartState(
            editor_state.clone(),
        ))
        .await
        .unwrap();
    workspace
        .update_state(UpdateStateInput::UpdateSidebarPartState(
            sidebar_state.clone(),
        ))
        .await
        .unwrap();
    workspace
        .update_state(UpdateStateInput::UpdatePanelPartState(panel_state.clone()))
        .await
        .unwrap();

    // Verify all states were stored correctly
    let state = workspace.describe_state().await.unwrap();
    assert_eq!(state.editor.unwrap(), editor_state);
    assert_eq!(state.sidebar.unwrap(), sidebar_state);
    assert_eq!(state.panel.unwrap(), panel_state);

    // Update individual states
    let updated_sidebar_state = SidebarPartState {
        preferred_size: 300,
        is_visible: false,
    };

    workspace
        .update_state(UpdateStateInput::UpdateSidebarPartState(
            updated_sidebar_state.clone(),
        ))
        .await
        .unwrap();

    // Verify only sidebar state was updated
    let state = workspace.describe_state().await.unwrap();
    assert_eq!(state.editor.unwrap(), editor_state);
    assert_eq!(state.sidebar.unwrap(), updated_sidebar_state);
    assert_eq!(state.panel.unwrap(), panel_state);

    // Clean up
    {
        tokio::fs::remove_dir_all(workspace_path).await.unwrap();
    }
}

#[tokio::test]
async fn update_state_overwrite_existing() {
    let (workspace_path, workspace) = setup_test_workspace().await;

    // Set initial state
    let initial_sidebar_state = SidebarPartState {
        preferred_size: 250,
        is_visible: true,
    };

    workspace
        .update_state(UpdateStateInput::UpdateSidebarPartState(
            initial_sidebar_state,
        ))
        .await
        .unwrap();

    // Update with new state
    let updated_sidebar_state = SidebarPartState {
        preferred_size: 300,
        is_visible: false,
    };

    workspace
        .update_state(UpdateStateInput::UpdateSidebarPartState(
            updated_sidebar_state.clone(),
        ))
        .await
        .unwrap();

    // Verify state was overwritten
    let state = workspace.describe_state().await.unwrap();
    assert_eq!(state.sidebar.unwrap(), updated_sidebar_state);

    // Clean up
    {
        tokio::fs::remove_dir_all(workspace_path).await.unwrap();
    }
}
