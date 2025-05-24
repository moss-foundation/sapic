mod shared;

use moss_storage::workspace_storage::entities::state_store_entities::{
    EditorPartStateEntity, PanelPartStateEntity, SidebarPartStateEntity,
};
use moss_workspace::models::operations::UpdateStateInput;
use moss_workspace::models::types::{PanelPartState, SidebarPartState};
use shared::create_simple_editor_state;

use crate::shared::{ITEMS_KEY, setup_test_workspace};

const PART_EDITOR_KEY: &'static str = "part:editor";
const PART_PANEL_KEY: &'static str = "part:panel";
const PART_SIDEBAR_KEY: &'static str = "part:sidebar";

#[tokio::test]
async fn update_state_editor_part() {
    let (_workspace_path, workspace, cleanup) = setup_test_workspace().await;

    let editor_state = create_simple_editor_state();

    let update_state_result = workspace
        .update_state(UpdateStateInput::UpdateEditorPartState(
            editor_state.clone(),
        ))
        .await;

    assert!(update_state_result.is_ok());

    // Verify the state was stored correctly
    let describe_state_output = workspace.describe_state().await.unwrap();
    assert!(describe_state_output.editor.is_some());
    assert_eq!(describe_state_output.editor.unwrap(), editor_state);

    // Verify the database is updated
    let dumped = workspace._storage().dump().unwrap();
    let items_dump = dumped[ITEMS_KEY].clone();
    let value = items_dump[PART_EDITOR_KEY].clone();
    let entity: EditorPartStateEntity = serde_json::from_value(value).unwrap();

    assert_eq!(entity, editor_state.into());

    // Clean up
    cleanup().await;
}

#[tokio::test]
async fn update_state_sidebar_part() {
    let (_workspace_path, workspace, cleanup) = setup_test_workspace().await;

    let sidebar_state = SidebarPartState {
        preferred_size: 250,
        is_visible: true,
    };

    let update_state_result = workspace
        .update_state(UpdateStateInput::UpdateSidebarPartState(
            sidebar_state.clone(),
        ))
        .await;

    assert!(update_state_result.is_ok());

    // Verify the state was stored correctly
    let describe_state_output = workspace.describe_state().await.unwrap();
    assert!(describe_state_output.sidebar.is_some());
    assert_eq!(describe_state_output.sidebar.unwrap(), sidebar_state);

    // Verify the database is updated
    let dumped = workspace._storage().dump().unwrap();
    let items_dump = dumped[ITEMS_KEY].clone();
    let value = items_dump[PART_SIDEBAR_KEY].clone();
    let entity: SidebarPartStateEntity = serde_json::from_value(value).unwrap();

    assert_eq!(entity, sidebar_state.into());

    // Clean up
    cleanup().await;
}

#[tokio::test]
async fn update_state_panel_part() {
    let (_workspace_path, workspace, cleanup) = setup_test_workspace().await;

    let panel_state = PanelPartState {
        preferred_size: 200,
        is_visible: false,
    };

    let update_state_result = workspace
        .update_state(UpdateStateInput::UpdatePanelPartState(panel_state.clone()))
        .await;

    assert!(update_state_result.is_ok());

    // Verify the state was stored correctly
    let describe_state_output = workspace.describe_state().await.unwrap();
    assert!(describe_state_output.panel.is_some());
    assert_eq!(describe_state_output.panel.unwrap(), panel_state);

    // Verify the database is updated
    let dumped = workspace._storage().dump().unwrap();
    let items_dump = dumped[ITEMS_KEY].clone();
    let value = items_dump[PART_PANEL_KEY].clone();
    let entity: PanelPartStateEntity = serde_json::from_value(value).unwrap();

    assert_eq!(entity, panel_state.into());

    // Clean up
    cleanup().await;
}

#[tokio::test]
async fn update_state_multiple_updates() {
    let (_workspace_path, workspace, cleanup) = setup_test_workspace().await;

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
    let update_editor_result = workspace
        .update_state(UpdateStateInput::UpdateEditorPartState(
            editor_state.clone(),
        ))
        .await;
    assert!(update_editor_result.is_ok());

    let update_sidebar_result = workspace
        .update_state(UpdateStateInput::UpdateSidebarPartState(
            sidebar_state.clone(),
        ))
        .await;
    assert!(update_sidebar_result.is_ok());

    let update_panel_result = workspace
        .update_state(UpdateStateInput::UpdatePanelPartState(panel_state.clone()))
        .await;
    assert!(update_panel_result.is_ok());

    // Verify all states were stored correctly
    let describe_state_output = workspace.describe_state().await.unwrap();
    assert_eq!(describe_state_output.editor.unwrap(), editor_state);
    assert_eq!(describe_state_output.sidebar.unwrap(), sidebar_state);
    assert_eq!(describe_state_output.panel.unwrap(), panel_state);

    let dumped = workspace._storage().dump().unwrap();
    let items_dump = dumped[ITEMS_KEY].clone();

    let editor_value = items_dump[PART_EDITOR_KEY].clone();
    let editor_entity: EditorPartStateEntity = serde_json::from_value(editor_value).unwrap();
    assert_eq!(editor_entity, editor_state.clone().into());

    let sidebar_value = items_dump[PART_SIDEBAR_KEY].clone();
    let sidebar_entity: SidebarPartStateEntity = serde_json::from_value(sidebar_value).unwrap();
    assert_eq!(sidebar_entity, sidebar_state.clone().into());

    let panel_value = items_dump[PART_PANEL_KEY].clone();
    let panel_entity: PanelPartStateEntity = serde_json::from_value(panel_value).unwrap();
    assert_eq!(panel_entity, panel_state.clone().into());

    // Update individual states
    let updated_sidebar_state = SidebarPartState {
        preferred_size: 300,
        is_visible: false,
    };

    let update_sidebar_result = workspace
        .update_state(UpdateStateInput::UpdateSidebarPartState(
            updated_sidebar_state.clone(),
        ))
        .await;
    assert!(update_sidebar_result.is_ok());

    // Verify only sidebar state was updated
    let describe_state_output = workspace.describe_state().await.unwrap();
    assert_eq!(describe_state_output.editor.unwrap(), editor_state);
    assert_eq!(
        describe_state_output.sidebar.unwrap(),
        updated_sidebar_state
    );
    assert_eq!(describe_state_output.panel.unwrap(), panel_state);

    let dumped = workspace._storage().dump().unwrap();
    let items_dump = dumped[ITEMS_KEY].clone();

    let editor_value = items_dump[PART_EDITOR_KEY].clone();
    let editor_entity: EditorPartStateEntity = serde_json::from_value(editor_value).unwrap();
    assert_eq!(editor_entity, editor_state.clone().into());

    let sidebar_value = items_dump[PART_SIDEBAR_KEY].clone();
    let sidebar_entity: SidebarPartStateEntity = serde_json::from_value(sidebar_value).unwrap();
    assert_eq!(sidebar_entity, updated_sidebar_state.clone().into());

    let panel_value = items_dump[PART_PANEL_KEY].clone();
    let panel_entity: PanelPartStateEntity = serde_json::from_value(panel_value).unwrap();
    assert_eq!(panel_entity, panel_state.clone().into());

    // Clean up
    cleanup().await;
}

#[tokio::test]
async fn update_state_overwrite_existing() {
    let (_workspace_path, workspace, cleanup) = setup_test_workspace().await;

    // Set initial state
    let initial_sidebar_state = SidebarPartState {
        preferred_size: 250,
        is_visible: true,
    };

    let update_sidebar_result = workspace
        .update_state(UpdateStateInput::UpdateSidebarPartState(
            initial_sidebar_state,
        ))
        .await;
    assert!(update_sidebar_result.is_ok());

    // Update with new state
    let updated_sidebar_state = SidebarPartState {
        preferred_size: 300,
        is_visible: false,
    };

    let update_sidebar_result = workspace
        .update_state(UpdateStateInput::UpdateSidebarPartState(
            updated_sidebar_state.clone(),
        ))
        .await;
    assert!(update_sidebar_result.is_ok());

    // Verify state was overwritten
    let describe_state_output = workspace.describe_state().await.unwrap();
    assert_eq!(
        describe_state_output.sidebar.unwrap(),
        updated_sidebar_state
    );

    // Verify database was updated
    let dumped = workspace._storage().dump().unwrap();
    let items_dump = dumped[ITEMS_KEY].clone();
    let sidebar_value = items_dump[PART_SIDEBAR_KEY].clone();
    let sidebar_entity: SidebarPartStateEntity = serde_json::from_value(sidebar_value).unwrap();
    assert_eq!(sidebar_entity, updated_sidebar_state.clone().into());

    // Clean up
    cleanup().await;
}
