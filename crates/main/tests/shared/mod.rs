#![cfg(feature = "integration-tests")]

use main::{MainWindow, workspace::RuntimeWorkspace, workspace_ops::MainWindowWorkspaceOps};
use moss_app_delegate::AppDelegate;
use moss_applib::mock::MockAppRuntime;
use moss_fs::{FileSystem, RealFileSystem};
use moss_keyring::KeyringClientImpl;
use moss_storage2::SubstoreManager;
use moss_testutils::random_name::random_string;
use moss_workspace::models::events::StreamProjectsEvent;
use reqwest::ClientBuilder as HttpClientBuilder;
use sapic_base::workspace::types::primitives::WorkspaceId;
use sapic_core::context::ArcContext;
use sapic_ipc::contracts::main::project::StreamProjectsOutput;
use sapic_platform::{
    github::{AppGitHubApiClient, auth::AppGitHubAuthAdapter},
    gitlab::{AppGitLabApiClient, auth::AppGitLabAuthAdapter},
    project::project_service_fs::ProjectServiceFs,
    server::HttpServerApiClient,
    workspace::{
        workspace_edit_backend::WorkspaceFsEditBackend, workspace_service_fs::WorkspaceServiceFs,
    },
};
use sapic_runtime::{app::kv_storage::AppStorage, user::AppUser};
use sapic_system::{
    ports::{github_api::GitHubAuthAdapter, gitlab_api::GitLabAuthAdapter},
    project::project_service::ProjectService,
    workspace::{
        workspace_edit_service::WorkspaceEditService, workspace_service::WorkspaceService,
    },
};
use sapic_window::OldSapicWindowBuilder;
use std::{
    path::PathBuf,
    pin::Pin,
    sync::{Arc, Mutex},
    time::Duration,
};
use tauri::ipc::{Channel, InvokeResponseBody};

pub type CleanupFn = Box<dyn FnOnce() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send>;
pub fn random_test_dir_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join(random_string(10))
}
pub struct MainWindowServices {}

pub async fn set_up_test_main_window() -> (
    MainWindow<MockAppRuntime>,
    AppDelegate<MockAppRuntime>,
    MainWindowServices,
    ArcContext,
    CleanupFn,
    PathBuf,
) {
    let ctx = ArcContext::background_with_timeout(Duration::from_secs(30));

    let tauri_app = tauri::test::mock_app();
    let tao_app_handle = tauri_app.handle().to_owned();

    let test_dir_path = random_test_dir_path();
    let resource_path = test_dir_path.join("resources");
    let user_path = test_dir_path.join("user");

    let delegate = AppDelegate::<MockAppRuntime>::new(tao_app_handle.clone());
    delegate.set_resource_dir(resource_path.clone());
    delegate.set_user_dir(user_path.clone());

    let required_folders = vec![
        delegate.tmp_dir().to_path_buf(),
        delegate.user_dir().to_path_buf(),
        delegate.resource_dir().to_path_buf(),
        delegate.workspaces_dir().to_path_buf(),
        delegate.globals_dir().to_path_buf(),
    ];

    for folder in required_folders {
        tokio::fs::create_dir_all(folder).await.unwrap();
    }

    let fs = Arc::new(RealFileSystem::new(&delegate.tmp_dir()));

    let storage = AppStorage::new(&delegate.globals_dir(), delegate.workspaces_dir(), None)
        .await
        .expect("failed to create storage");

    let http_client = HttpClientBuilder::new()
        .user_agent("SAPIC/1.0")
        .build()
        .expect("failed to build http client");

    let server_api_client: Arc<HttpServerApiClient> =
        HttpServerApiClient::new("Test endpoint".to_string(), http_client.clone()).into();

    let github_api_client = Arc::new(AppGitHubApiClient::new(http_client.clone()));
    let gitlab_api_client = Arc::new(AppGitLabApiClient::new(http_client.clone()));

    let auth_gateway_url: Arc<String> = server_api_client.base_url().to_string().into();

    let github_auth_adapter: Arc<dyn GitHubAuthAdapter> = Arc::new(AppGitHubAuthAdapter::new(
        server_api_client.clone(),
        auth_gateway_url.clone(),
        8080,
    ));
    let gitlab_auth_adapter: Arc<dyn GitLabAuthAdapter> = Arc::new(AppGitLabAuthAdapter::new(
        server_api_client.clone(),
        auth_gateway_url,
        8081,
    ));

    let keyring = Arc::new(KeyringClientImpl::new());

    let user = AppUser::new(
        &ctx,
        delegate.user_dir(),
        fs.clone(),
        server_api_client.clone(),
        github_api_client.clone(),
        gitlab_api_client.clone(),
        github_auth_adapter.clone(),
        gitlab_auth_adapter.clone(),
        keyring.clone(),
    )
    .await
    .unwrap();

    // Main Window requires a workspace. We will create it first

    let workspace_id = WorkspaceId::new();
    let workspace_path = delegate.workspaces_dir().join(workspace_id.to_string());
    let workspace_projects_path = workspace_path.join("projects");
    tokio::fs::create_dir_all(&workspace_path).await.unwrap();
    tokio::fs::create_dir_all(&workspace_projects_path)
        .await
        .unwrap();

    storage.add_workspace(workspace_id.inner()).await.unwrap();

    let workspace_service = Arc::new(WorkspaceService::new(
        WorkspaceServiceFs::new(fs.clone(), delegate.workspaces_dir()),
        storage.clone(),
    ));

    let workspace_edit_service = {
        let workspace_edit_backend =
            WorkspaceFsEditBackend::new(fs.clone(), delegate.workspaces_dir());
        Arc::new(WorkspaceEditService::new(workspace_edit_backend))
    };

    let workspace = {
        let projects_path = workspace_path.join("projects");
        let project_service = ProjectService::new(
            workspace_id.clone(),
            ProjectServiceFs::new(fs.clone(), projects_path.clone()),
            fs.clone(),
            storage.clone(),
        );

        Arc::new(RuntimeWorkspace::new(
            workspace_id.clone(),
            workspace_path.clone(),
            fs.clone(),
            storage.clone(),
            workspace_edit_service.clone(),
            user.clone(),
            github_api_client.clone(),
            gitlab_api_client.clone(),
            project_service,
        ))
    };

    let old_sapic_window = OldSapicWindowBuilder::new(
        fs,
        storage.clone(),
        keyring.clone(),
        server_api_client.clone(),
        github_api_client.clone(),
        gitlab_api_client.clone(),
        workspace_id.clone(),
    )
    .build(&ctx, &delegate)
    .await
    .unwrap();

    let workspace_ops = MainWindowWorkspaceOps::new(workspace_service.clone());

    let main_window = MainWindow::new(
        &delegate,
        0,
        old_sapic_window,
        workspace.clone(),
        workspace_ops,
    )
    .await
    .unwrap();

    let main_window_services = MainWindowServices {};

    let storage_clone = storage.clone();
    let cleanup_fn = Box::new({
        let path = test_dir_path.clone();
        let storage_clone = storage_clone.clone();
        move || {
            Box::pin(async move {
                storage_clone.close().await.unwrap();
                if let Err(e) = tokio::fs::remove_dir_all(&path).await {
                    eprintln!("Failed to clean up test directory: {}", e);
                }
            }) as Pin<Box<dyn Future<Output = ()> + Send>>
        }
    });

    (
        main_window,
        delegate,
        main_window_services,
        ctx,
        cleanup_fn,
        test_dir_path,
    )
}

pub async fn test_stream_projects(
    window: &MainWindow<MockAppRuntime>,
    ctx: &ArcContext,
) -> (StreamProjectsOutput, Vec<StreamProjectsEvent>) {
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

    let output = window.stream_projects(ctx, channel).await.unwrap();

    (output, received_events.lock().unwrap().clone())
}
