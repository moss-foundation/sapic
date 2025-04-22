mod shared;

use moss_workspace::models::operations::SetLayoutPartsStateInput;
use moss_workspace::models::types::{PanelPartState, SidebarPartState};
use shared::create_simple_editor_state;

use crate::shared::setup_test_workspace;

#[tokio::test]
async fn describe_layout_parts_state_empty() {
    let (workspace_path, workspace) = setup_test_workspace().await;

    let result = workspace.describe_layout_parts_state().await;
    assert!(result.is_ok());

    let describe_layout_parts_state_output = result.unwrap();

    // With a fresh workspace, we expect no layout states to be present
    assert!(describe_layout_parts_state_output.editor.is_none());
    assert!(describe_layout_parts_state_output.sidebar.is_none());
    assert!(describe_layout_parts_state_output.panel.is_none());

    // Clean up
    {
        tokio::fs::remove_dir_all(workspace_path).await.unwrap();
    }
}

#[tokio::test]
async fn describe_layout_parts_state_sidebar_only() {
    let (workspace_path, workspace) = setup_test_workspace().await;

    // Set up only the sidebar state
    let sidebar_state = SidebarPartState {
        preferred_size: 250,
        is_visible: true,
    };

    workspace
        .set_layout_parts_state(SetLayoutPartsStateInput {
            editor: None,
            sidebar: Some(sidebar_state),
            panel: None,
        })
        .await
        .unwrap();

    // Check the describe_layout_parts_state operation
    let describe_layout_parts_state_output = workspace.describe_layout_parts_state().await.unwrap();

    // Editor and Panel should be None
    assert!(describe_layout_parts_state_output.editor.is_none());
    assert!(describe_layout_parts_state_output.panel.is_none());

    // Sidebar should match the set value
    assert!(describe_layout_parts_state_output.sidebar.is_some());
    let retrieved_sidebar = describe_layout_parts_state_output.sidebar.unwrap();
    assert_eq!(retrieved_sidebar.preferred_size, 250);
    assert_eq!(retrieved_sidebar.is_visible, true);

    // Cleanup
    {
        tokio::fs::remove_dir_all(workspace_path).await.unwrap();
    }
}

#[tokio::test]
async fn describe_layout_parts_state_panel_only() {
    let (workspace_path, workspace) = setup_test_workspace().await;

    // Set up only the panel state
    let panel_state = PanelPartState {
        preferred_size: 200,
        is_visible: false,
    };

    workspace
        .set_layout_parts_state(SetLayoutPartsStateInput {
            editor: None,
            sidebar: None,
            panel: Some(panel_state),
        })
        .await
        .unwrap();

    // Check the describe_layout_parts_state operation
    let describe_layout_parts_state_output = workspace.describe_layout_parts_state().await.unwrap();

    // Editor and Sidebar should be None
    assert!(describe_layout_parts_state_output.editor.is_none());
    assert!(describe_layout_parts_state_output.sidebar.is_none());

    // Panel should match the set value
    assert!(describe_layout_parts_state_output.panel.is_some());
    let retrieved_panel = describe_layout_parts_state_output.panel.unwrap();
    assert_eq!(retrieved_panel.preferred_size, 200);
    assert_eq!(retrieved_panel.is_visible, false);

    // Cleanup
    {
        tokio::fs::remove_dir_all(workspace_path).await.unwrap();
    }
}

#[tokio::test]
async fn describe_layout_parts_state_editor_only() {
    let (workspace_path, workspace) = setup_test_workspace().await;

    // Set up only the editor state
    let editor_state = create_simple_editor_state();

    workspace
        .set_layout_parts_state(SetLayoutPartsStateInput {
            editor: Some(editor_state),
            sidebar: None,
            panel: None,
        })
        .await
        .unwrap();

    // Check the describe_layout_parts_state operation
    let describe_layout_parts_state_output = workspace.describe_layout_parts_state().await.unwrap();

    // Sidebar and Panel should be None
    assert!(describe_layout_parts_state_output.sidebar.is_none());
    assert!(describe_layout_parts_state_output.panel.is_none());

    // Editor should be set
    assert!(describe_layout_parts_state_output.editor.is_some());
    let retrieved_editor = describe_layout_parts_state_output.editor.unwrap();

    // Check editor values
    assert!(!retrieved_editor.panels.is_empty());
    assert!(retrieved_editor.panels.contains_key("panel1"));
    assert_eq!(retrieved_editor.active_group, Some("group1".to_string()));

    // Cleanup
    {
        tokio::fs::remove_dir_all(workspace_path).await.unwrap();
    }
}

#[tokio::test]
async fn describe_layout_parts_state_all() {
    let (workspace_path, workspace) = setup_test_workspace().await;

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

    workspace
        .set_layout_parts_state(SetLayoutPartsStateInput {
            editor: Some(editor_state),
            sidebar: Some(sidebar_state),
            panel: Some(panel_state),
        })
        .await
        .unwrap();

    // Check the describe_layout_parts_state operation
    let describe_layout_parts_state_output = workspace.describe_layout_parts_state().await.unwrap();

    // All states should be set

    // Check Editor
    assert!(describe_layout_parts_state_output.editor.is_some());
    let retrieved_editor = describe_layout_parts_state_output.editor.unwrap();
    assert!(!retrieved_editor.panels.is_empty());
    assert!(retrieved_editor.panels.contains_key("panel1"));
    assert_eq!(retrieved_editor.active_group, Some("group1".to_string()));

    // Check Sidebar
    assert!(describe_layout_parts_state_output.sidebar.is_some());
    let retrieved_sidebar = describe_layout_parts_state_output.sidebar.unwrap();
    assert_eq!(retrieved_sidebar.preferred_size, 250);
    assert_eq!(retrieved_sidebar.is_visible, true);

    // Check Panel
    assert!(describe_layout_parts_state_output.panel.is_some());
    let retrieved_panel = describe_layout_parts_state_output.panel.unwrap();
    assert_eq!(retrieved_panel.preferred_size, 200);
    assert_eq!(retrieved_panel.is_visible, false);

    // Cleanup
    {
        tokio::fs::remove_dir_all(workspace_path).await.unwrap();
    }
}

#[tokio::test]
async fn describe_layout_parts_state_after_update() {
    let (workspace_path, workspace) = setup_test_workspace().await;

    // First set all states
    workspace
        .set_layout_parts_state(SetLayoutPartsStateInput {
            editor: Some(create_simple_editor_state()),
            sidebar: Some(SidebarPartState {
                preferred_size: 250,
                is_visible: true,
            }),
            panel: Some(PanelPartState {
                preferred_size: 200,
                is_visible: false,
            }),
        })
        .await
        .unwrap();

    // Now update only the sidebar
    workspace
        .set_layout_parts_state(SetLayoutPartsStateInput {
            editor: None,
            sidebar: Some(SidebarPartState {
                preferred_size: 300,
                is_visible: false,
            }),
            panel: None,
        })
        .await
        .unwrap();

    // Check the describe_layout_parts_state operation after update
    let describe_layout_parts_state_output = workspace.describe_layout_parts_state().await.unwrap();

    // Editor should not change
    assert!(describe_layout_parts_state_output.editor.is_some());
    let retrieved_editor = describe_layout_parts_state_output.editor.unwrap();
    assert!(!retrieved_editor.panels.is_empty());
    assert!(retrieved_editor.panels.contains_key("panel1"));

    // Sidebar should be updated
    assert!(describe_layout_parts_state_output.sidebar.is_some());
    let retrieved_sidebar = describe_layout_parts_state_output.sidebar.unwrap();
    assert_eq!(retrieved_sidebar.preferred_size, 300); // Updated value
    assert_eq!(retrieved_sidebar.is_visible, false); // Updated value

    // Panel should not change
    assert!(describe_layout_parts_state_output.panel.is_some());
    let retrieved_panel = describe_layout_parts_state_output.panel.unwrap();
    assert_eq!(retrieved_panel.preferred_size, 200);
    assert_eq!(retrieved_panel.is_visible, false);

    // Cleanup
    {
        tokio::fs::remove_dir_all(workspace_path).await.unwrap();
    }
}
