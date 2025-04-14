mod shared;

use moss_workspace::models::operations::SetLayoutPartsStateInput;
use moss_workspace::models::types::{PanelPartState, SidebarPartState};
use shared::create_simple_editor_state;

use crate::shared::setup_test_workspace;

#[tokio::test]
async fn set_layout_parts_state_editor() {
    let (workspace_path, workspace) = setup_test_workspace().await;

    let editor_state = create_simple_editor_state();
    let set_layout_parts_state_result = workspace
        .set_layout_parts_state(SetLayoutPartsStateInput {
            editor: Some(editor_state),
            sidebar: None,
            panel: None,
        })
        .await;

    assert!(set_layout_parts_state_result.is_ok());

    // Verify state was set by retrieving it
    let describe_layout_parts_state_output = workspace.describe_layout_parts_state().await.unwrap();

    // Editor should be set to our values
    assert!(describe_layout_parts_state_output.editor.is_some());
    let retrieved_editor = describe_layout_parts_state_output.editor.unwrap();

    // Verify that editor state is populated
    assert!(!retrieved_editor.panels.is_empty());
    assert!(retrieved_editor.panels.contains_key("panel1"));
    assert_eq!(retrieved_editor.active_group, Some("group1".to_string()));

    // Other values should be None since we didn't set them
    assert!(describe_layout_parts_state_output.sidebar.is_none());
    assert!(describe_layout_parts_state_output.panel.is_none());

    // Cleanup
    {
        tokio::fs::remove_dir_all(workspace_path).await.unwrap();
    }
}

#[tokio::test]
async fn set_layout_parts_state_sidebar() {
    let (workspace_path, workspace) = setup_test_workspace().await;

    let sidebar_state = SidebarPartState {
        preferred_size: 250,
        is_visible: true,
    };

    let set_layout_parts_state_result = workspace
        .set_layout_parts_state(SetLayoutPartsStateInput {
            editor: None,
            sidebar: Some(sidebar_state),
            panel: None,
        })
        .await;

    assert!(set_layout_parts_state_result.is_ok());

    // Verify state was set by retrieving it
    let describe_layout_parts_state_output = workspace.describe_layout_parts_state().await.unwrap();

    // Editor and panel should be None
    assert!(describe_layout_parts_state_output.editor.is_none());
    assert!(describe_layout_parts_state_output.panel.is_none());

    // Sidebar should be set to our values
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
async fn set_layout_parts_state_panel() {
    let (workspace_path, workspace) = setup_test_workspace().await;

    let panel_state = PanelPartState {
        preferred_size: 200,
        is_visible: false,
    };
    let set_layout_parts_state_result = workspace
        .set_layout_parts_state(SetLayoutPartsStateInput {
            editor: None,
            sidebar: None,
            panel: Some(panel_state),
        })
        .await;

    assert!(set_layout_parts_state_result.is_ok());

    // Verify state was set by retrieving it
    let describe_layout_parts_state_output = workspace.describe_layout_parts_state().await.unwrap();

    // Editor and sidebar should be None
    assert!(describe_layout_parts_state_output.editor.is_none());
    assert!(describe_layout_parts_state_output.sidebar.is_none());

    // Panel should be set to our values
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
async fn set_layout_parts_state_all() {
    let (workspace_path, workspace) = setup_test_workspace().await;
    let editor_state = create_simple_editor_state();
    let sidebar_state = SidebarPartState {
        preferred_size: 250,
        is_visible: true,
    };

    let panel_state = PanelPartState {
        preferred_size: 200,
        is_visible: false,
    };

    // Set all the states
    let set_layout_parts_state_result = workspace
        .set_layout_parts_state(SetLayoutPartsStateInput {
            editor: Some(editor_state),
            sidebar: Some(sidebar_state),
            panel: Some(panel_state),
        })
        .await;

    assert!(set_layout_parts_state_result.is_ok());

    // Verify all states were set by retrieving them
    let describe_layout_parts_state_output = workspace.describe_layout_parts_state().await.unwrap();

    // Editor state should match
    assert!(describe_layout_parts_state_output.editor.is_some());
    let retrieved_editor = describe_layout_parts_state_output.editor.unwrap();
    assert!(!retrieved_editor.panels.is_empty());
    assert!(retrieved_editor.panels.contains_key("panel1"));
    assert_eq!(retrieved_editor.active_group, Some("group1".to_string()));

    // Sidebar state should match
    assert!(describe_layout_parts_state_output.sidebar.is_some());
    let retrieved_sidebar = describe_layout_parts_state_output.sidebar.unwrap();
    assert_eq!(retrieved_sidebar.preferred_size, 250);
    assert_eq!(retrieved_sidebar.is_visible, true);

    // Panel state should match
    assert!(describe_layout_parts_state_output.panel.is_some());
    let retrieved_panel = describe_layout_parts_state_output.panel.unwrap();
    assert_eq!(retrieved_panel.preferred_size, 200);
    assert_eq!(retrieved_panel.is_visible, false);

    {
        tokio::fs::remove_dir_all(workspace_path).await.unwrap();
    }
}

#[tokio::test]
async fn set_layout_parts_state_update() {
    let (workspace_path, workspace) = setup_test_workspace().await;

    workspace
        .set_layout_parts_state(SetLayoutPartsStateInput {
            editor: Some(create_simple_editor_state()),
            sidebar: Some(SidebarPartState {
                preferred_size: 200,
                is_visible: true,
            }),
            panel: Some(PanelPartState {
                preferred_size: 150,
                is_visible: false,
            }),
        })
        .await
        .unwrap();

    // Now update just one state (sidebar)
    let set_layout_parts_state_result = workspace
        .set_layout_parts_state(SetLayoutPartsStateInput {
            editor: None,
            sidebar: Some(SidebarPartState {
                preferred_size: 300,
                is_visible: false,
            }),
            panel: None,
        })
        .await;

    assert!(set_layout_parts_state_result.is_ok());

    // Verify that only sidebar state was updated
    let describe_layout_parts_state_output = workspace.describe_layout_parts_state().await.unwrap();

    // Editor state should remain unchanged
    assert!(describe_layout_parts_state_output.editor.is_some());
    let retrieved_editor = describe_layout_parts_state_output.editor.unwrap();
    assert!(retrieved_editor.panels.contains_key("panel1"));
    assert_eq!(retrieved_editor.active_group, Some("group1".to_string()));

    // Sidebar state should be updated
    assert!(describe_layout_parts_state_output.sidebar.is_some());
    let retrieved_sidebar = describe_layout_parts_state_output.sidebar.unwrap();
    assert_eq!(retrieved_sidebar.preferred_size, 300);
    assert_eq!(retrieved_sidebar.is_visible, false);

    // Panel state should remain unchanged
    assert!(describe_layout_parts_state_output.panel.is_some());
    let retrieved_panel = describe_layout_parts_state_output.panel.unwrap();
    assert_eq!(retrieved_panel.preferred_size, 150);
    assert_eq!(retrieved_panel.is_visible, false);

    // Cleanup
    {
        tokio::fs::remove_dir_all(workspace_path).await.unwrap();
    }
}

#[tokio::test]
async fn set_layout_parts_state_empty() {
    let (workspace_path, workspace) = setup_test_workspace().await;
    let set_layout_parts_state_result = workspace
        .set_layout_parts_state(SetLayoutPartsStateInput {
            editor: None,
            sidebar: None,
            panel: None,
        })
        .await;

    assert!(set_layout_parts_state_result.is_ok());

    // Verify all states are still None
    let describe_layout_parts_state_output = workspace.describe_layout_parts_state().await.unwrap();
    assert!(describe_layout_parts_state_output.editor.is_none());
    assert!(describe_layout_parts_state_output.sidebar.is_none());
    assert!(describe_layout_parts_state_output.panel.is_none());

    // Cleanup
    {
        tokio::fs::remove_dir_all(workspace_path).await.unwrap();
    }
}
