// TODO: Remove once we get rid of layout service
#![cfg(feature = "integration-tests")]
pub mod shared;

use moss_workspace::models::{
    operations::UpdateLayoutInput,
    primitives::SidebarPosition,
    types::{PanelPartStateInfo, SidebarPartStateInfo},
};

use crate::shared::setup_test_workspace;

#[tokio::test]
async fn describe_layout_parts_state_empty() {
    let (ctx, _, workspace, cleanup, _) = setup_test_workspace().await;

    let describe_state_result = workspace.describe_workspace(&ctx).await;

    let describe_state_output = describe_state_result.unwrap();

    // With a fresh workspace, we expect default layout states to be present
    assert!(describe_state_output.layouts.editor.is_none()); // Editor is still None since no defaults

    // Sidebar, Panel, and Activitybar should have default values
    assert!(describe_state_output.layouts.sidebar.is_some());
    assert!(describe_state_output.layouts.panel.is_some());
    assert!(describe_state_output.layouts.activitybar.is_some());

    cleanup().await;
}

#[tokio::test]
async fn describe_layout_parts_state_sidebar_only() {
    let (ctx, _, workspace, cleanup, _) = setup_test_workspace().await;

    // Set up only the sidebar state
    let sidebar_state = SidebarPartStateInfo {
        size: 250.0,
        visible: true,
        position: SidebarPosition::Left,
    };

    let update_state_result = workspace
        .update_layout(
            &ctx,
            UpdateLayoutInput {
                sidebar: Some(sidebar_state),
                editor: None,
                panel: None,
                activitybar: None,
            },
        )
        .await;
    let _ = update_state_result.unwrap();

    // Check the describe_state operation
    let describe_state_output = workspace.describe_workspace(&ctx).await.unwrap();

    // Editor should be None
    assert!(describe_state_output.layouts.editor.is_none());

    // Panel and Activitybar should have default values
    assert!(describe_state_output.layouts.panel.is_some());
    assert!(describe_state_output.layouts.activitybar.is_some());

    // Sidebar should match the set value
    assert!(describe_state_output.layouts.sidebar.is_some());
    let retrieved_sidebar = describe_state_output.layouts.sidebar.unwrap();
    assert_eq!(retrieved_sidebar.size, 250.0);
    assert_eq!(retrieved_sidebar.visible, true);
    assert_eq!(retrieved_sidebar.position, SidebarPosition::Left);

    cleanup().await;
}

#[tokio::test]
async fn describe_layout_parts_state_panel_only() {
    let (ctx, _, workspace, cleanup, _) = setup_test_workspace().await;

    // Set up only the panel state
    let panel_state = PanelPartStateInfo {
        size: 200.0,
        visible: false,
    };

    let update_state_result = workspace
        .update_layout(
            &ctx,
            UpdateLayoutInput {
                editor: None,
                sidebar: None,
                panel: Some(panel_state),
                activitybar: None,
            },
        )
        .await;
    let _ = update_state_result.unwrap();

    // Check the describe_state operation
    let describe_state_output = workspace.describe_workspace(&ctx).await.unwrap();

    // Editor should be None
    assert!(describe_state_output.layouts.editor.is_none());

    // Sidebar and Activitybar should have default values
    assert!(describe_state_output.layouts.sidebar.is_some());
    assert!(describe_state_output.layouts.activitybar.is_some());

    // Panel should match the set value
    assert!(describe_state_output.layouts.panel.is_some());
    let retrieved_panel = describe_state_output.layouts.panel.unwrap();
    assert_eq!(retrieved_panel.size, 200.0);
    assert_eq!(retrieved_panel.visible, false);

    cleanup().await;
}

#[tokio::test]
async fn describe_layout_parts_state_editor_only() {
    let (ctx, _, workspace, cleanup, _) = setup_test_workspace().await;

    // Since EditorPartStateInfo is being removed, this test now just checks
    // that editor state is None when no editor state is set
    let describe_state_output = workspace.describe_workspace(&ctx).await.unwrap();

    // All states should be default values (not None)
    assert!(describe_state_output.layouts.sidebar.is_some());
    assert!(describe_state_output.layouts.panel.is_some());
    assert!(describe_state_output.layouts.activitybar.is_some());

    // Editor should be None since no editor state is set
    assert!(describe_state_output.layouts.editor.is_none());

    cleanup().await;
}

#[tokio::test]
async fn describe_layout_parts_state_all() {
    let (ctx, _, workspace, cleanup, _) = setup_test_workspace().await;

    // Set up sidebar and panel states (no editor state since it's being removed)
    let sidebar_state = SidebarPartStateInfo {
        size: 250.0,
        visible: true,
        position: SidebarPosition::Left,
    };
    let panel_state = PanelPartStateInfo {
        size: 200.0,
        visible: false,
    };

    // Update sidebar and panel states
    let update_sidebar_result = workspace
        .update_layout(
            &ctx,
            UpdateLayoutInput {
                editor: None,
                sidebar: Some(sidebar_state),
                panel: None,
                activitybar: None,
            },
        )
        .await;
    let _ = update_sidebar_result.unwrap();

    let update_panel_result = workspace
        .update_layout(
            &ctx,
            UpdateLayoutInput {
                editor: None,
                sidebar: None,
                panel: Some(panel_state),
                activitybar: None,
            },
        )
        .await;
    let _ = update_panel_result.unwrap();

    // Check the describe_state operation
    let describe_state_output = workspace.describe_workspace(&ctx).await.unwrap();

    // Editor should be None since no editor state is set
    assert!(describe_state_output.layouts.editor.is_none());

    // Check Sidebar
    assert!(describe_state_output.layouts.sidebar.is_some());
    let retrieved_sidebar = describe_state_output.layouts.sidebar.unwrap();
    assert_eq!(retrieved_sidebar.size, 250.0);
    assert_eq!(retrieved_sidebar.visible, true);

    // Check Panel
    assert!(describe_state_output.layouts.panel.is_some());
    let retrieved_panel = describe_state_output.layouts.panel.unwrap();
    assert_eq!(retrieved_panel.size, 200.0);
    assert_eq!(retrieved_panel.visible, false);

    // Check Activitybar (should have default values)
    assert!(describe_state_output.layouts.activitybar.is_some());

    cleanup().await;
}

#[tokio::test]
async fn describe_layout_parts_state_after_update() {
    let (ctx, _, workspace, cleanup, _) = setup_test_workspace().await;

    // First set sidebar and panel states (no editor state since it's being removed)
    let initial_sidebar_state = SidebarPartStateInfo {
        size: 250.0,
        visible: true,
        position: SidebarPosition::Left,
    };
    let initial_panel_state = PanelPartStateInfo {
        size: 200.0,
        visible: false,
    };

    let update_sidebar_result = workspace
        .update_layout(
            &ctx,
            UpdateLayoutInput {
                editor: None,
                sidebar: Some(initial_sidebar_state),
                panel: None,
                activitybar: None,
            },
        )
        .await;
    let _ = update_sidebar_result.unwrap();

    let update_panel_result = workspace
        .update_layout(
            &ctx,
            UpdateLayoutInput {
                editor: None,
                sidebar: None,
                panel: Some(initial_panel_state),
                activitybar: None,
            },
        )
        .await;
    let _ = update_panel_result.unwrap();

    // Now update only the sidebar
    let updated_sidebar_state = SidebarPartStateInfo {
        size: 300.0,
        visible: false,
        position: SidebarPosition::Left,
    };
    let update_sidebar_result = workspace
        .update_layout(
            &ctx,
            UpdateLayoutInput {
                editor: None,
                sidebar: Some(updated_sidebar_state),
                panel: None,
                activitybar: None,
            },
        )
        .await;
    let _ = update_sidebar_result.unwrap();

    // Check the describe_state operation after update
    let describe_state_output = workspace.describe_workspace(&ctx).await.unwrap();

    // Editor should be None since no editor state is set
    assert!(describe_state_output.layouts.editor.is_none());

    // Sidebar should be updated
    assert!(describe_state_output.layouts.sidebar.is_some());
    let retrieved_sidebar = describe_state_output.layouts.sidebar.unwrap();
    assert_eq!(retrieved_sidebar.size, 300.0); // Updated value
    assert_eq!(retrieved_sidebar.visible, false); // Updated value

    // Panel should not change
    assert!(describe_state_output.layouts.panel.is_some());
    let retrieved_panel = describe_state_output.layouts.panel.unwrap();
    assert_eq!(retrieved_panel.size, 200.0);
    assert_eq!(retrieved_panel.visible, false);

    cleanup().await;
}
