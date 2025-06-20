use image::{ImageBuffer, Rgb};
use moss_activity_indicator::ActivityIndicator;
use moss_applib::context::test::MockContext;
use moss_fs::{FileSystem, RealFileSystem};
use moss_storage::primitives::segkey::SegKeyBuf;
use moss_testutils::random_name::random_workspace_name;
use moss_workspace::{
    Workspace,
    models::{
        primitives::{EditorGridOrientation, PanelRenderer},
        types::{
            EditorGridLeafData, EditorGridNode, EditorGridState, EditorPanelState,
            EditorPartStateInfo,
        },
    },
    storage::segments::COLLECTION_SEGKEY,
    workspace::CreateParams,
};
use rand::Rng;
use std::{
    collections::HashMap,
    fs,
    future::Future,
    path::{Path, PathBuf},
    pin::Pin,
    sync::Arc,
};
use tauri::test::MockRuntime;
use uuid::Uuid;

pub type CleanupFn = Box<dyn FnOnce() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send>;

pub async fn setup_test_workspace() -> (MockContext, Arc<Path>, Workspace<MockRuntime>, CleanupFn) {
    let fs = Arc::new(RealFileSystem::new());
    let mock_app = tauri::test::mock_app();
    let app_handle = mock_app.handle().clone();

    <dyn FileSystem>::set_global(fs, &app_handle);

    let ctx = MockContext::new(app_handle.clone());

    let workspace_path: Arc<Path> = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join("workspaces")
        .join(Uuid::new_v4().to_string())
        .into();
    fs::create_dir_all(&workspace_path).unwrap();

    let activity_indicator = ActivityIndicator::new(app_handle.clone());
    let workspace = Workspace::create(
        &ctx,
        &workspace_path,
        // fs,
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

    (ctx, workspace_path, workspace, cleanup_fn)
}

pub fn create_simple_editor_state() -> EditorPartStateInfo {
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

    EditorPartStateInfo {
        grid,
        panels,
        active_group: Some("group1".to_string()),
    }
}

pub fn collection_key(id: Uuid) -> SegKeyBuf {
    COLLECTION_SEGKEY.join(id.to_string())
}

pub fn generate_random_icon(output_path: &Path) {
    // Create an empty RGB image buffer
    let mut img = ImageBuffer::new(128, 128);
    let mut rng = rand::rng();

    // Fill each pixel with random values [0..255]
    for pixel in img.pixels_mut() {
        let r: u8 = rng.random();
        let g: u8 = rng.random();
        let b: u8 = rng.random();
        *pixel = Rgb([r, g, b]);
    }

    img.save(output_path).unwrap();
}
