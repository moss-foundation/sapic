#![cfg(feature = "integration-tests")]

use image::{ImageBuffer, Rgb};
use moss_activity_indicator::ActivityIndicator;
use moss_applib::{
    context::{AsyncContext, MutableContext},
    mock::MockAppRuntime,
};
use moss_fs::{FileSystem, RealFileSystem};
use moss_git_hosting_provider::{
    common::ssh_auth_agent::SSHAuthAgentImpl,
    github::{auth::GitHubAuthAgentImpl, client::GitHubClient},
    gitlab::{auth::GitLabAuthAgentImpl, client::GitLabClient},
};
use moss_keyring::KeyringClientImpl;
use moss_storage::primitives::segkey::SegKeyBuf;
use moss_testutils::random_name::random_workspace_name;
use moss_workspace::{
    Workspace,
    builder::{CreateWorkspaceParams, WorkspaceBuilder},
    models::{
        primitives::{CollectionId, EditorGridOrientation, PanelRenderer},
        types::{
            EditorGridLeafData, EditorGridNode, EditorGridState, EditorPanelState,
            EditorPartStateInfo,
        },
    },
    storage::segments::SEGKEY_COLLECTION,
};
use rand::Rng;
use std::{
    collections::HashMap,
    fs,
    future::Future,
    path::{Path, PathBuf},
    pin::Pin,
    sync::Arc,
    time::Duration,
};

pub type CleanupFn = Box<dyn FnOnce() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send>;

pub async fn setup_test_workspace() -> (AsyncContext, Workspace<MockAppRuntime>, CleanupFn) {
    let fs = Arc::new(RealFileSystem::new());
    let mock_app = tauri::test::mock_app();
    let app_handle = mock_app.handle().clone();

    <dyn FileSystem>::set_global(fs.clone(), &app_handle);

    let ctx = MutableContext::background_with_timeout(Duration::from_secs(30)).freeze();

    let abs_path: Arc<Path> = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join("workspaces")
        .join(random_workspace_name())
        .into();
    fs::create_dir_all(&abs_path).unwrap();

    let activity_indicator = ActivityIndicator::new(app_handle.clone());

    let keyring_client = Arc::new(KeyringClientImpl::new());
    let reqwest_client = reqwest::ClientBuilder::new()
        .user_agent("SAPIC")
        .build()
        .expect("failed to build reqwest client");

    let github_client = {
        let github_auth_agent =
            GitHubAuthAgentImpl::new(keyring_client.clone(), "".to_string(), "".to_string());
        Arc::new(GitHubClient::new(
            reqwest_client.clone(),
            github_auth_agent,
            None as Option<SSHAuthAgentImpl>,
        ))
    };
    let gitlab_client = {
        let gitlab_auth_agent =
            GitLabAuthAgentImpl::new(keyring_client.clone(), "".to_string(), "".to_string());
        Arc::new(GitLabClient::new(
            reqwest_client.clone(),
            gitlab_auth_agent,
            None as Option<SSHAuthAgentImpl>,
        ))
    };

    let workspace: Workspace<MockAppRuntime> = WorkspaceBuilder::new(fs.clone())
        .create(
            &ctx,
            activity_indicator,
            CreateWorkspaceParams {
                name: random_workspace_name(),
                abs_path: abs_path.clone(),
            },
            github_client,
            gitlab_client,
        )
        .await
        .unwrap();

    let cleanup_fn = Box::new({
        let abs_path_clone = abs_path.clone();
        move || {
            Box::pin(async move {
                if let Err(e) = tokio::fs::remove_dir_all(&abs_path_clone).await {
                    eprintln!("Failed to clean up test workspace directory: {}", e);
                }
            }) as Pin<Box<dyn Future<Output = ()> + Send>>
        }
    });

    (ctx, workspace, cleanup_fn)
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

pub fn collection_key(id: &CollectionId) -> SegKeyBuf {
    SEGKEY_COLLECTION.join(id)
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
