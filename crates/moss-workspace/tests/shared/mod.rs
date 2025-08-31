#![cfg(feature = "integration-tests")]

use image::{ImageBuffer, Rgb};
use moss_activity_broadcaster::ActivityBroadcaster;
use moss_applib::{
    AppHandle,
    context::{AnyContext, AsyncContext, MutableContext},
    mock::MockAppRuntime,
};
use moss_asp::AppSecretsProvider;
use moss_fs::RealFileSystem;
use moss_git_hosting_provider::{
    envvar_keys::GITLAB_CLIENT_SECRET, github::GitHubApiClient, gitlab::GitLabApiClient,
};
use moss_keyring::test::MockKeyringClient;
use moss_testutils::random_name::random_workspace_name;
use moss_user::{Account, AccountSession, models::primitives::AccountId, profile::ActiveProfile};
use moss_workspace::{
    Workspace,
    builder::{CreateWorkspaceParams, WorkspaceBuilder},
    models::{
        primitives::{EditorGridOrientation, PanelRenderer},
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
    sync::Arc,
    time::Duration,
};
use tauri::Manager;

pub type CleanupFn = Box<dyn FnOnce() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send>;

pub async fn setup_test_workspace() -> (
    AsyncContext,
    AppHandle<MockAppRuntime>,
    Workspace<MockAppRuntime>,
    CleanupFn,
) {
    dotenv::dotenv().ok();
    let fs = Arc::new(RealFileSystem::new());
    let mock_app = tauri::test::mock_app();
    let tao_app_handle = mock_app.handle().clone();
    {
        let http_client = HttpClientBuilder::new()
            .user_agent("SAPIC/1.0")
            .build()
            .expect("failed to build http client");

        let github_client = GitHubApiClient::new(http_client.clone());
        let gitlab_client = GitLabApiClient::new(http_client.clone());

        tao_app_handle.manage(http_client);
        tao_app_handle.manage(github_client);
        tao_app_handle.manage(gitlab_client);
    }
    let app_handle = AppHandle::new(tao_app_handle.clone());

    let mut ctx = MutableContext::background_with_timeout(Duration::from_secs(30));

    let abs_path: Arc<Path> = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join("workspaces")
        .join(random_workspace_name())
        .into();
    fs::create_dir_all(&abs_path).unwrap();

    let broadcaster = ActivityBroadcaster::new(tao_app_handle.clone());

    let keyring = Arc::new(MockKeyringClient::new());
    let secrets = AppSecretsProvider::new(
        dotenv::var(GITLAB_CLIENT_SECRET).unwrap_or_default(),
        dotenv::var(GITLAB_CLIENT_SECRET).unwrap_or_default(),
        keyring.clone(),
    )
    .await
    .unwrap();

    let account_id = AccountId::new();
    ctx.with_value("account_id", account_id.clone());
    let account_session = AccountSession::github(
        account_id.clone(),
        "github.com".to_string(),
        secrets,
        keyring.clone(),
        None,
    )
    .await
    .unwrap();
    let profiles = HashMap::from([(
        account_id.clone(),
        Account::new(
            account_id,
            random_workspace_name(),
            "github.com".to_string(),
            account_session,
        ),
    )]);
    let active_profile = ActiveProfile::new(profiles);

    let ctx = ctx.freeze();
    let workspace: Workspace<MockAppRuntime> =
        WorkspaceBuilder::<MockAppRuntime>::new(fs.clone(), broadcaster, active_profile.into())
            .create(
                &ctx,
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

    (ctx, app_handle, workspace, cleanup_fn)
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
