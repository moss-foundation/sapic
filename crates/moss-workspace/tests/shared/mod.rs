use moss_activity_indicator::ActivityIndicator;
use moss_fs::RealFileSystem;
use moss_testutils::random_name::random_workspace_name;
use moss_workspace::models::types::{
    EditorGridLeafData, EditorGridNode, EditorGridOrientation, EditorGridState, EditorPanelState,
    EditorPartState, PanelRenderer,
};
use moss_workspace::{CreateParams, Workspace};
use std::collections::HashMap;
use std::fs;
use std::future::Future;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::Arc;
use tauri::test::MockRuntime;
use uuid::Uuid;

pub type CleanupFn = Box<dyn FnOnce() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send>;

pub async fn setup_test_workspace() -> (Arc<Path>, Workspace<MockRuntime>, CleanupFn) {
    let mock_app = tauri::test::mock_app();
    let app_handle = mock_app.handle().clone();

    let workspace_path: Arc<Path> = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join("workspaces")
        .join(Uuid::new_v4().to_string())
        .into();
    fs::create_dir_all(&workspace_path).unwrap();

    let fs = Arc::new(RealFileSystem::new());
    let activity_indicator = ActivityIndicator::new(app_handle.clone());
    let workspace = Workspace::create(
        app_handle.clone(),
        &workspace_path,
        fs,
        activity_indicator,
        CreateParams {
            name: Some(random_workspace_name()),
        },
    )
    .await
    .unwrap();

    let path = workspace_path.to_path_buf();
    let cleanup_fn = Box::new(move || {
        let path = path;
        Box::pin(async move {
            if let Err(e) = tokio::fs::remove_dir_all(&path).await {
                eprintln!("Failed to clean up test workspace directory: {}", e);
            }
        }) as Pin<Box<dyn Future<Output = ()> + Send>>
    });

    (workspace_path, workspace, cleanup_fn)
}

#[allow(dead_code)]
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
