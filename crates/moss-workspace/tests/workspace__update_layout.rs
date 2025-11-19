// TODO: Remove this when we get rid of layout service
#![cfg(feature = "integration-tests")]
pub mod shared;

use moss_storage::storage::operations::GetItem;
use moss_workspace::models::{
    operations::UpdateLayoutInput,
    primitives::SidebarPosition,
    types::{PanelPartStateInfo, SidebarPartStateInfo},
};

use crate::shared::setup_test_workspace;
#[tokio::test]
async fn update_state_sidebar_part() {
    let (ctx, _, workspace, cleanup) = setup_test_workspace().await;

    let sidebar_state = SidebarPartStateInfo {
        size: 250.0,
        visible: true,
        position: SidebarPosition::Left,
    };

    let update_state_result = workspace
        .update_layout(
            &ctx,
            UpdateLayoutInput {
                editor: None,
                sidebar: Some(sidebar_state.clone()),
                panel: None,
                activitybar: None,
            },
        )
        .await;

    let _ = update_state_result.unwrap();

    // Verify the state was stored correctly via describe_state
    let describe_state_output = workspace.describe_workspace(&ctx).await.unwrap();
    assert!(describe_state_output.layouts.sidebar.is_some());
    assert_eq!(
        describe_state_output.layouts.sidebar.as_ref().unwrap(),
        &sidebar_state
    );

    // Verify the database is updated with individual keys
    let item_store = workspace.db().item_store();

    // Check position
    let position_value = GetItem::get(
        item_store.as_ref(),
        &ctx,
        SEGKEY_LAYOUT_SIDEBAR.join("position"),
    )
    .await
    .unwrap();
    let stored_position: SidebarPosition = position_value.deserialize().unwrap();
    assert_eq!(stored_position, SidebarPosition::Left);

    // Check size
    let size_value = GetItem::get(
        item_store.as_ref(),
        &ctx,
        SEGKEY_LAYOUT_SIDEBAR.join("size"),
    )
    .await
    .unwrap();
    let stored_size: f64 = size_value.deserialize().unwrap();
    assert_eq!(stored_size, 250.0);

    // Check visible
    let visible_value = GetItem::get(
        item_store.as_ref(),
        &ctx,
        SEGKEY_LAYOUT_SIDEBAR.join("visible"),
    )
    .await
    .unwrap();
    let stored_visible: bool = visible_value.deserialize().unwrap();
    assert_eq!(stored_visible, true);

    cleanup().await;
}

#[tokio::test]
async fn update_state_panel_part() {
    let (ctx, _, workspace, cleanup) = setup_test_workspace().await;

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
                panel: Some(panel_state.clone()),
                activitybar: None,
            },
        )
        .await;

    let _ = update_state_result.unwrap();

    // Verify the state was stored correctly via describe_state
    let describe_state_output = workspace.describe_workspace(&ctx).await.unwrap();
    assert!(describe_state_output.layouts.panel.is_some());
    assert_eq!(describe_state_output.layouts.panel.unwrap(), panel_state);

    // Verify the database is updated with individual keys
    let item_store = workspace.db().item_store();

    // Check size
    let size_value = GetItem::get(item_store.as_ref(), &ctx, SEGKEY_LAYOUT_PANEL.join("size"))
        .await
        .unwrap();
    let stored_size: f64 = size_value.deserialize().unwrap();
    assert_eq!(stored_size, 200.0);

    // Check visible
    let visible_value = GetItem::get(
        item_store.as_ref(),
        &ctx,
        SEGKEY_LAYOUT_PANEL.join("visible"),
    )
    .await
    .unwrap();
    let stored_visible: bool = visible_value.deserialize().unwrap();
    assert_eq!(stored_visible, false);

    cleanup().await;
}

#[tokio::test]
async fn update_state_multiple_updates() {
    let (ctx, _, workspace, cleanup) = setup_test_workspace().await;

    // Initial states
    let sidebar_state = SidebarPartStateInfo {
        size: 250.0,
        visible: true,
        position: SidebarPosition::Left,
    };
    let panel_state = PanelPartStateInfo {
        size: 200.0,
        visible: false,
    };

    // Update states
    let update_sidebar_result = workspace
        .update_layout(
            &ctx,
            UpdateLayoutInput {
                editor: None,
                sidebar: Some(sidebar_state.clone()),
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
                panel: Some(panel_state.clone()),
                activitybar: None,
            },
        )
        .await;
    let _ = update_panel_result.unwrap();

    // Verify all states were stored correctly via describe_state
    let describe_state_output = workspace.describe_workspace(&ctx).await.unwrap();
    assert_eq!(
        describe_state_output.layouts.sidebar.unwrap(),
        sidebar_state
    );
    assert_eq!(describe_state_output.layouts.panel.unwrap(), panel_state);

    // Verify the database is updated with individual keys
    let item_store = workspace.db().item_store();

    // Check sidebar values
    let sidebar_position: SidebarPosition = GetItem::get(
        item_store.as_ref(),
        &ctx,
        SEGKEY_LAYOUT_SIDEBAR.join("position"),
    )
    .await
    .unwrap()
    .deserialize()
    .unwrap();
    assert_eq!(sidebar_position, SidebarPosition::Left);

    let sidebar_size: f64 = GetItem::get(
        item_store.as_ref(),
        &ctx,
        SEGKEY_LAYOUT_SIDEBAR.join("size"),
    )
    .await
    .unwrap()
    .deserialize()
    .unwrap();
    assert_eq!(sidebar_size, 250.0);

    let sidebar_visible: bool = GetItem::get(
        item_store.as_ref(),
        &ctx,
        SEGKEY_LAYOUT_SIDEBAR.join("visible"),
    )
    .await
    .unwrap()
    .deserialize()
    .unwrap();
    assert_eq!(sidebar_visible, true);

    // Check panel values
    let panel_size: f64 = GetItem::get(item_store.as_ref(), &ctx, SEGKEY_LAYOUT_PANEL.join("size"))
        .await
        .unwrap()
        .deserialize()
        .unwrap();
    assert_eq!(panel_size, 200.0);

    let panel_visible: bool = GetItem::get(
        item_store.as_ref(),
        &ctx,
        SEGKEY_LAYOUT_PANEL.join("visible"),
    )
    .await
    .unwrap()
    .deserialize()
    .unwrap();
    assert_eq!(panel_visible, false);

    // Update individual states
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
                sidebar: Some(updated_sidebar_state.clone()),
                panel: None,
                activitybar: None,
            },
        )
        .await;
    let _ = update_sidebar_result.unwrap();

    // Verify only sidebar state was updated via describe_state
    let describe_state_output = workspace.describe_workspace(&ctx).await.unwrap();
    assert_eq!(
        describe_state_output.layouts.sidebar.as_ref().unwrap(),
        &updated_sidebar_state
    );
    assert_eq!(describe_state_output.layouts.panel.unwrap(), panel_state);

    // Verify the database reflects the updated sidebar values
    let updated_sidebar_size: f64 = GetItem::get(
        item_store.as_ref(),
        &ctx,
        SEGKEY_LAYOUT_SIDEBAR.join("size"),
    )
    .await
    .unwrap()
    .deserialize()
    .unwrap();
    assert_eq!(updated_sidebar_size, 300.0);

    let updated_sidebar_visible: bool = GetItem::get(
        item_store.as_ref(),
        &ctx,
        SEGKEY_LAYOUT_SIDEBAR.join("visible"),
    )
    .await
    .unwrap()
    .deserialize()
    .unwrap();
    assert_eq!(updated_sidebar_visible, false);

    // Panel values should remain unchanged
    let panel_size_after: f64 =
        GetItem::get(item_store.as_ref(), &ctx, SEGKEY_LAYOUT_PANEL.join("size"))
            .await
            .unwrap()
            .deserialize()
            .unwrap();
    assert_eq!(panel_size_after, 200.0);

    let panel_visible_after: bool = GetItem::get(
        item_store.as_ref(),
        &ctx,
        SEGKEY_LAYOUT_PANEL.join("visible"),
    )
    .await
    .unwrap()
    .deserialize()
    .unwrap();
    assert_eq!(panel_visible_after, false);

    cleanup().await;
}

#[tokio::test]
async fn update_state_overwrite_existing() {
    let (ctx, _, workspace, cleanup) = setup_test_workspace().await;

    // Set initial state
    let initial_sidebar_state = SidebarPartStateInfo {
        size: 250.0,
        visible: true,
        position: SidebarPosition::Left,
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

    // Verify initial state in database
    let item_store = workspace.db().item_store();
    let initial_size: f64 = GetItem::get(
        item_store.as_ref(),
        &ctx,
        SEGKEY_LAYOUT_SIDEBAR.join("size"),
    )
    .await
    .unwrap()
    .deserialize()
    .unwrap();
    assert_eq!(initial_size, 250.0);

    let initial_visible: bool = GetItem::get(
        item_store.as_ref(),
        &ctx,
        SEGKEY_LAYOUT_SIDEBAR.join("visible"),
    )
    .await
    .unwrap()
    .deserialize()
    .unwrap();
    assert_eq!(initial_visible, true);

    // Update with new state
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
                sidebar: Some(updated_sidebar_state.clone()),
                panel: None,
                activitybar: None,
            },
        )
        .await;
    let _ = update_sidebar_result.unwrap();

    // Verify state was overwritten via describe_state
    let describe_state_output = workspace.describe_workspace(&ctx).await.unwrap();
    assert_eq!(
        describe_state_output.layouts.sidebar.as_ref().unwrap(),
        &updated_sidebar_state
    );

    // Verify database was updated with new values
    let updated_size: f64 = GetItem::get(
        item_store.as_ref(),
        &ctx,
        SEGKEY_LAYOUT_SIDEBAR.join("size"),
    )
    .await
    .unwrap()
    .deserialize()
    .unwrap();
    assert_eq!(updated_size, 300.0);

    let updated_visible: bool = GetItem::get(
        item_store.as_ref(),
        &ctx,
        SEGKEY_LAYOUT_SIDEBAR.join("visible"),
    )
    .await
    .unwrap()
    .deserialize()
    .unwrap();
    assert_eq!(updated_visible, false);

    cleanup().await;
}
