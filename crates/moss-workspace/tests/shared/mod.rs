#![cfg(feature = "integration-tests")]

use image::{ImageBuffer, Rgb};
use moss_app_delegate::AppDelegate;
use moss_applib::{
    AppRuntime,
    context::{AsyncContext, MutableContext},
    mock::MockAppRuntime,
};
use moss_fs::RealFileSystem;
use moss_git_hosting_provider::{github::RealGitHubApiClient, gitlab::RealGitLabApiClient};
use moss_testutils::random_name::random_workspace_name;
use moss_user::profile::Profile;
use moss_workspace::{
    Workspace,
    builder::{CreateWorkspaceParams, WorkspaceBuilder},
    models::{
        events::StreamProjectsEvent,
        operations::StreamProjectsOutput,
        primitives::{EditorGridOrientation, PanelRenderer, ProjectId},
        types::{
            EditorGridLeafData, EditorGridNode, EditorGridState, EditorPanelState,
            EditorPartStateInfo,
        },
    },
};
use rand::Rng;
use reqwest::ClientBuilder as HttpClientBuilder;
use std::{
    collections::HashMap,
    fs,
    future::Future,
    path::{Path, PathBuf},
    pin::Pin,
    sync::{Arc, Mutex},
    time::Duration,
};
use tauri::{
    Manager,
    ipc::{Channel, InvokeResponseBody},
};

pub type CleanupFn = Box<dyn FnOnce() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send>;

pub async fn setup_test_workspace() -> (
    AsyncContext,
    AppDelegate<MockAppRuntime>,
    Workspace<MockAppRuntime>,
    CleanupFn,
) {
    dotenv::dotenv().ok();
    let abs_path: Arc<Path> = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join("workspaces")
        .join(random_workspace_name())
        .into();
    let tmp_path = abs_path.join("tmp");
    fs::create_dir_all(&abs_path).unwrap();
    fs::create_dir_all(&tmp_path).unwrap();

    let fs = Arc::new(RealFileSystem::new(&tmp_path));
    let mock_app = tauri::test::mock_app();
    let tao_app_handle = mock_app.handle().clone();
    {
        let http_client = HttpClientBuilder::new()
            .user_agent("SAPIC/1.0")
            .build()
            .expect("failed to build http client");

        let github_client = RealGitHubApiClient::new(http_client.clone());
        let gitlab_client = RealGitLabApiClient::new(http_client.clone());

        tao_app_handle.manage(http_client);
        tao_app_handle.manage(github_client);
        tao_app_handle.manage(gitlab_client);
    }
    let app_delegate = AppDelegate::new(tao_app_handle.clone());

    let ctx = MutableContext::background_with_timeout(Duration::from_secs(30));

    let abs_path: Arc<Path> = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join("workspaces")
        .join(random_workspace_name())
        .into();
    fs::create_dir_all(&abs_path).unwrap();

    let active_profile = Profile::new(
        moss_user::models::primitives::ProfileId::new(),
        HashMap::new(),
    );

    let ctx = ctx.freeze();
    let workspace: Workspace<MockAppRuntime> =
        WorkspaceBuilder::new(fs.clone(), active_profile.into())
            .create(
                &ctx,
                &app_delegate,
                CreateWorkspaceParams {
                    name: random_workspace_name(),
                    abs_path: abs_path.clone(),
                },
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

    (ctx, app_delegate, workspace, cleanup_fn)
}

pub fn _create_simple_editor_state() -> EditorPartStateInfo {
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

#[allow(unused)]
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

#[allow(unused)]
pub async fn test_stream_collections<R: AppRuntime>(
    ctx: &R::AsyncContext,
    workspace: &Workspace<R>,
) -> (
    HashMap<ProjectId, StreamProjectsEvent>,
    StreamProjectsOutput,
) {
    let received_events = Arc::new(Mutex::new(Vec::new()));
    let received_events_clone = received_events.clone();

    let channel = Channel::new(move |body: InvokeResponseBody| {
        if let InvokeResponseBody::Json(json_str) = body {
            if let Ok(event) = serde_json::from_str::<StreamProjectsEvent>(&json_str) {
                received_events_clone.lock().unwrap().push(event);
            }
        }
        Ok(())
    });

    let output = workspace
        .stream_projects(ctx, channel.clone())
        .await
        .unwrap();
    (
        received_events
            .lock()
            .unwrap()
            .iter()
            .map(|event| (event.id.clone(), event.clone()))
            .collect(),
        output,
    )
}
