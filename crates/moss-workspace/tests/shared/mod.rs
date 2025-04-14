use moss_fs::RealFileSystem;
use moss_testutils::random_name::{random_string, random_workspace_name};
use moss_workbench::activity_indicator::ActivityIndicator;
use moss_workspace::models::types::{
    EditorGridLeafData, EditorGridNode, EditorGridOrientation, EditorGridState, EditorPanelState,
    EditorPartState, PanelRenderer,
};
use moss_workspace::workspace::Workspace;
use moss_workspace::workspace_manager::WorkspaceManager;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::test::MockRuntime;

pub fn random_workspaces_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join(random_string(10))
}

pub fn random_workspace_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join("workspaces")
        .join(random_workspace_name())
}

pub async fn setup_test_workspace_manager() -> (PathBuf, WorkspaceManager<MockRuntime>) {
    let mock_app = tauri::test::mock_app();
    let app_handle = mock_app.handle().clone();

    let fs = Arc::new(RealFileSystem::new());
    let workspaces_path: PathBuf = random_workspaces_path();
    tokio::fs::create_dir_all(workspaces_path.clone())
        .await
        .unwrap();

    let workspace_manager = WorkspaceManager::new(app_handle, fs, workspaces_path.clone()).unwrap();

    (workspaces_path, workspace_manager)
}

pub async fn setup_test_workspace() -> (PathBuf, Workspace) {
    let mock_app = tauri::test::mock_app();
    let app_handle = mock_app.handle().clone();

    let fs = Arc::new(RealFileSystem::new());
    let workspace_path: PathBuf = random_workspace_path();
    fs::create_dir_all(&workspace_path).unwrap();
    let activity_indicator = ActivityIndicator::new(app_handle.clone());
    let workspace = Workspace::new(workspace_path.clone(), fs, activity_indicator).unwrap();
    (workspace_path, workspace)
}

pub fn create_simple_editor_state() -> EditorPartState {
    // Create a simple grid with one leaf
    let leaf_data = EditorGridLeafData {
        views: vec!["panel1".to_string()],
        active_view: "panel1".to_string(),
        id: "group1".to_string(),
    };

    let grid_node = EditorGridNode::Leaf {
        data: leaf_data,
        size: 100.0,
    };

    // Create grid state
    let grid = EditorGridState {
        root: grid_node,
        width: 800.0,
        height: 600.0,
        orientation: EditorGridOrientation::Horizontal,
    };

    // Create some panels
    let mut panels = HashMap::new();

    panels.insert(
        "panel1".to_string(),
        EditorPanelState {
            id: "panel1".to_string(),
            content_component: Some("TestComponent".to_string()),
            tab_component: None,
            title: Some("Test Panel".to_string()),
            renderer: Some(PanelRenderer::OnlyWhenVisible),
            params: Some(HashMap::new()),
            minimum_width: None,
            minimum_height: None,
            maximum_width: None,
            maximum_height: None,
        },
    );

    panels.insert(
        "panel2".to_string(),
        EditorPanelState {
            id: "panel2".to_string(),
            content_component: Some("AnotherComponent".to_string()),
            tab_component: None,
            title: Some("Another Panel".to_string()),
            renderer: None,
            params: None,
            minimum_width: Some(200.0),
            minimum_height: Some(150.0),
            maximum_width: None,
            maximum_height: None,
        },
    );

    EditorPartState {
        grid,
        panels,
        active_group: Some("group1".to_string()),
    }
}
