pub mod shared;

use moss_workspace::models::{
    operations::UpdateStateInput,
    types::{PanelPartState, SidebarPartState},
};
use shared::create_simple_editor_state;

use crate::shared::setup_test_workspace;

#[tokio::test]
async fn describe_layout_parts_state_empty() {
    let (_workspace_path, workspace, cleanup) = setup_test_workspace().await;

    let describe_state_result = workspace.describe_state().await;
    assert!(describe_state_result.is_ok());

    let describe_state_output = describe_state_result.unwrap();

    // With a fresh workspace, we expect no layout states to be present
    assert!(describe_state_output.editor.is_none());
    assert!(describe_state_output.sidebar.is_none());
    assert!(describe_state_output.panel.is_none());

    // Clean up
    cleanup().await;
}

#[tokio::test]
async fn describe_layout_parts_state_sidebar_only() {
    let (_workspace_path, workspace, cleanup) = setup_test_workspace().await;

    // Set up only the sidebar state
    let sidebar_state = SidebarPartState {
        preferred_size: 250,
        is_visible: true,
    };

    let update_state_result = workspace
        .update_state(UpdateStateInput::UpdateSidebarPartState(sidebar_state))
        .await;
    assert!(update_state_result.is_ok());

    // Check the describe_state operation
    let describe_state_output = workspace.describe_state().await.unwrap();

    // Editor and Panel should be None
    assert!(describe_state_output.editor.is_none());
    assert!(describe_state_output.panel.is_none());

    // Sidebar should match the set value
    assert!(describe_state_output.sidebar.is_some());
    let retrieved_sidebar = describe_state_output.sidebar.unwrap();
    assert_eq!(retrieved_sidebar.preferred_size, 250);
    assert_eq!(retrieved_sidebar.is_visible, true);

    // Cleanup
    cleanup().await;
}

#[tokio::test]
async fn describe_layout_parts_state_panel_only() {
    let (_workspace_path, workspace, cleanup) = setup_test_workspace().await;

    // Set up only the panel state
    let panel_state = PanelPartState {
        preferred_size: 200,
        is_visible: false,
    };

    let update_state_result = workspace
        .update_state(UpdateStateInput::UpdatePanelPartState(panel_state))
        .await;
    assert!(update_state_result.is_ok());

    // Check the describe_state operation
    let describe_state_output = workspace.describe_state().await.unwrap();

    // Editor and Sidebar should be None
    assert!(describe_state_output.editor.is_none());
    assert!(describe_state_output.sidebar.is_none());

    // Panel should match the set value
    assert!(describe_state_output.panel.is_some());
    let retrieved_panel = describe_state_output.panel.unwrap();
    assert_eq!(retrieved_panel.preferred_size, 200);
    assert_eq!(retrieved_panel.is_visible, false);

    // Cleanup
    cleanup().await;
}

#[tokio::test]
async fn describe_layout_parts_state_editor_only() {
    let (_workspace_path, workspace, cleanup) = setup_test_workspace().await;

    // Set up only the editor state
    let editor_state = create_simple_editor_state();

    let update_state_result = workspace
        .update_state(UpdateStateInput::UpdateEditorPartState(editor_state))
        .await;
    assert!(update_state_result.is_ok());

    // Check the describe_state operation
    let describe_state_output = workspace.describe_state().await.unwrap();

    // Sidebar and Panel should be None
    assert!(describe_state_output.sidebar.is_none());
    assert!(describe_state_output.panel.is_none());

    // Editor should be set
    assert!(describe_state_output.editor.is_some());
    let retrieved_editor = describe_state_output.editor.unwrap();

    // Check editor values
    assert!(!retrieved_editor.panels.is_empty());
    assert!(retrieved_editor.panels.contains_key("panel1"));
    assert_eq!(retrieved_editor.active_group, Some("group1".to_string()));

    // Cleanup
    cleanup().await;
}

#[tokio::test]
async fn describe_layout_parts_state_all() {
    let (_workspace_path, workspace, cleanup) = setup_test_workspace().await;

    // Set up all states
    let editor_state = create_simple_editor_state();
    let sidebar_state = SidebarPartState {
        preferred_size: 250,
        is_visible: true,
    };
    let panel_state = PanelPartState {
        preferred_size: 200,
        is_visible: false,
    };

    // Update each state individually
    let update_editor_result = workspace
        .update_state(UpdateStateInput::UpdateEditorPartState(editor_state))
        .await;
    assert!(update_editor_result.is_ok());

    let update_sidebar_result = workspace
        .update_state(UpdateStateInput::UpdateSidebarPartState(sidebar_state))
        .await;
    assert!(update_sidebar_result.is_ok());

    let update_panel_result = workspace
        .update_state(UpdateStateInput::UpdatePanelPartState(panel_state))
        .await;
    assert!(update_panel_result.is_ok());

    // Check the describe_state operation
    let describe_state_output = workspace.describe_state().await.unwrap();

    // All states should be set

    // Check Editor
    assert!(describe_state_output.editor.is_some());
    let retrieved_editor = describe_state_output.editor.unwrap();
    assert!(!retrieved_editor.panels.is_empty());
    assert!(retrieved_editor.panels.contains_key("panel1"));
    assert_eq!(retrieved_editor.active_group, Some("group1".to_string()));

    // Check Sidebar
    assert!(describe_state_output.sidebar.is_some());
    let retrieved_sidebar = describe_state_output.sidebar.unwrap();
    assert_eq!(retrieved_sidebar.preferred_size, 250);
    assert_eq!(retrieved_sidebar.is_visible, true);

    // Check Panel
    assert!(describe_state_output.panel.is_some());
    let retrieved_panel = describe_state_output.panel.unwrap();
    assert_eq!(retrieved_panel.preferred_size, 200);
    assert_eq!(retrieved_panel.is_visible, false);

    // Cleanup
    cleanup().await;
}

#[tokio::test]
async fn describe_layout_parts_state_after_update() {
    let (_workspace_path, workspace, cleanup) = setup_test_workspace().await;

    // First set all states
    let initial_editor_state = create_simple_editor_state();
    let initial_sidebar_state = SidebarPartState {
        preferred_size: 250,
        is_visible: true,
    };
    let initial_panel_state = PanelPartState {
        preferred_size: 200,
        is_visible: false,
    };

    let update_editor_result = workspace
        .update_state(UpdateStateInput::UpdateEditorPartState(
            initial_editor_state,
        ))
        .await;
    assert!(update_editor_result.is_ok());

    let update_sidebar_result = workspace
        .update_state(UpdateStateInput::UpdateSidebarPartState(
            initial_sidebar_state,
        ))
        .await;
    assert!(update_sidebar_result.is_ok());

    let update_panel_result = workspace
        .update_state(UpdateStateInput::UpdatePanelPartState(initial_panel_state))
        .await;
    assert!(update_panel_result.is_ok());

    // Now update only the sidebar
    let updated_sidebar_state = SidebarPartState {
        preferred_size: 300,
        is_visible: false,
    };
    let update_sidebar_result = workspace
        .update_state(UpdateStateInput::UpdateSidebarPartState(
            updated_sidebar_state,
        ))
        .await;
    assert!(update_sidebar_result.is_ok());

    // Check the describe_state operation after update
    let describe_state_output = workspace.describe_state().await.unwrap();

    // Editor should not change
    assert!(describe_state_output.editor.is_some());
    let retrieved_editor = describe_state_output.editor.unwrap();
    assert!(!retrieved_editor.panels.is_empty());
    assert!(retrieved_editor.panels.contains_key("panel1"));

    // Sidebar should be updated
    assert!(describe_state_output.sidebar.is_some());
    let retrieved_sidebar = describe_state_output.sidebar.unwrap();
    assert_eq!(retrieved_sidebar.preferred_size, 300); // Updated value
    assert_eq!(retrieved_sidebar.is_visible, false); // Updated value

    // Panel should not change
    assert!(describe_state_output.panel.is_some());
    let retrieved_panel = describe_state_output.panel.unwrap();
    assert_eq!(retrieved_panel.preferred_size, 200);
    assert_eq!(retrieved_panel.is_visible, false);

    // Clean up
    cleanup().await;
}
