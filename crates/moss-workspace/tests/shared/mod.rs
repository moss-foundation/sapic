#![allow(deprecated)] // TODO: remove once we get rid of old context types
#![cfg(feature = "integration-tests")]

use image::{ImageBuffer, Rgb};
use moss_app_delegate::AppDelegate;
use moss_applib::{AppRuntime, mock::MockAppRuntime};
use moss_fs::RealFileSystem;
use moss_git_hosting_provider::{github::AppGitHubApiClient, gitlab::AppGitLabApiClient};
use moss_storage2::{AppStorage, AppStorageOptions, Storage};
use moss_testutils::random_name::{random_string, random_workspace_name};
use moss_user::profile::Profile;
use moss_workspace::{
    Workspace,
    builder::{CreateWorkspaceParams, WorkspaceBuilder},
    models::{
        events::StreamProjectsEvent,
        operations::{CreateProjectInput, DeleteProjectInput, StreamProjectsOutput},
        primitives::{EditorGridOrientation, PanelRenderer, ProjectId, WorkspaceId},
        types::{
            CreateProjectParams, EditorGridLeafData, EditorGridNode, EditorGridState,
            EditorPanelState, EditorPartStateInfo,
        },
    },
};
use rand::Rng;
use reqwest::ClientBuilder as HttpClientBuilder;
use sapic_core::context::{AsyncContext, MutableContext};
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
    dotenvy::dotenv().ok();

    let abs_path: Arc<Path> = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join(random_string(10))
        .into();
    let tmp_path = abs_path.join("tmp");
    let globals_path = abs_path.join("globals");
    let workspaces_path = abs_path.join("workspaces");
    let workspace_id = WorkspaceId::new();
    let test_workspace_path = workspaces_path.join(workspace_id.as_str());

    fs::create_dir_all(&abs_path).unwrap();
    fs::create_dir_all(&tmp_path).unwrap();
    fs::create_dir_all(&globals_path).unwrap();
    fs::create_dir_all(&test_workspace_path).unwrap();

    let fs = Arc::new(RealFileSystem::new(&tmp_path));
    let mock_app = tauri::test::mock_app();
    let tao_app_handle = mock_app.handle().clone();
    {
        let http_client = HttpClientBuilder::new()
            .user_agent("SAPIC/1.0")
            .build()
            .expect("failed to build http client");

        let github_client = AppGitHubApiClient::new(http_client.clone());
        let gitlab_client = AppGitLabApiClient::new(http_client.clone());

        tao_app_handle.manage(http_client);
        tao_app_handle.manage(github_client);
        tao_app_handle.manage(gitlab_client);
    }

    let app_storage = AppStorage::new(
        &globals_path,
        workspaces_path,
        AppStorageOptions {
            in_memory: Some(false),
            busy_timeout: Some(Duration::from_secs(5)),
        }
        .into(),
    )
    .await
    .unwrap();

    let app_delegate = {
        let delegate = AppDelegate::new(tao_app_handle.clone());
        <dyn Storage>::set_global(&delegate, app_storage.clone());
        delegate
    };

    let ctx = MutableContext::background_with_timeout(Duration::from_secs(30));

    let active_profile = Profile::new(
        moss_user::models::primitives::ProfileId::new(),
        HashMap::new(),
    );

    let ctx = ctx.freeze();
    let workspace: Workspace<MockAppRuntime> =
        WorkspaceBuilder::new(fs.clone(), active_profile.into(), workspace_id.clone())
            .create(
                &ctx,
                &app_delegate,
                CreateWorkspaceParams {
                    name: random_workspace_name(),
                    abs_path: test_workspace_path.clone().into(),
                },
            )
            .await
            .unwrap();

    let cleanup_fn = Box::new({
        let abs_path_clone = abs_path.clone();
        let app_storage_clone = app_storage.clone();
        move || {
            Box::pin(async move {
                app_storage_clone.close().await.unwrap();
                // Looks like some delay is necessary to release SQLite file handle
                tokio::time::sleep(Duration::from_millis(100)).await;
                if let Err(e) = tokio::fs::remove_dir_all(&abs_path_clone).await {
                    eprintln!("Failed to clean up test workspace directory: {}", e);
                }
            }) as Pin<Box<dyn Future<Output = ()> + Send>>
        }
    });

    // Add workspace storage
    <dyn Storage>::global(&app_delegate)
        .add_workspace(workspace.id().inner())
        .await
        .unwrap();

    (ctx, app_delegate, workspace, cleanup_fn)
}

// Suppress false warning
#[allow(unused)]
// Create an external project that can be tested for import
pub async fn setup_external_project(
    ctx: &<MockAppRuntime as AppRuntime>::AsyncContext,
    app_delegate: &AppDelegate<MockAppRuntime>,
    workspace: &Workspace<MockAppRuntime>,
) -> (String, PathBuf) {
    let project_name = random_workspace_name();
    let external_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join("external_projects")
        .join(&project_name);
    tokio::fs::create_dir_all(&external_path).await.unwrap();

    let id = workspace
        .create_project(
            ctx,
            app_delegate,
            &CreateProjectInput {
                inner: CreateProjectParams {
                    name: project_name.clone(),
                    order: 0,
                    external_path: Some(external_path.clone()),
                    git_params: None,
                    icon_path: None,
                },
            },
        )
        .await
        .unwrap()
        .id;

    // Delete the external project
    // The external folder should remain intact, possible to be imported again
    workspace
        .delete_project(ctx, &DeleteProjectInput { id })
        .await
        .unwrap();

    (project_name, external_path)
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
pub async fn test_stream_projects<R: AppRuntime>(
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
