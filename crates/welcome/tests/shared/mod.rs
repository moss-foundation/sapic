#![cfg(feature = "integration-tests")]
use moss_app_delegate::AppDelegate;
use moss_applib::mock::MockAppRuntime;
use moss_fs::RealFileSystem;
use moss_testutils::random_name::random_string;
use sapic_core::context::ArcContext;
use sapic_platform::{
    environment::app_environment_service_fs::AppEnvironmentServiceFs,
    workspace::{
        workspace_edit_backend::WorkspaceFsEditBackend, workspace_service_fs::WorkspaceServiceFs,
    },
};
use sapic_runtime::app::kv_storage::AppStorage;
use sapic_system::{
    environment::app_environment_service::AppEnvironmentService,
    workspace::{
        workspace_edit_service::WorkspaceEditService, workspace_service::WorkspaceService,
    },
};
use std::{path::PathBuf, pin::Pin, sync::Arc, time::Duration};
use welcome::{
    WelcomeWindow, environment_ops::WelcomeWindowEnvironmentOps,
    workspace_ops::WelcomeWindowWorkspaceOps,
};

pub type CleanupFn = Box<dyn FnOnce() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send>;

pub struct WelcomeWindowServices {
    pub workspace_service: Arc<WorkspaceService>,
}

pub fn random_test_dir_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join(random_string(10))
}
pub async fn set_up_test_welcome_window() -> (
    WelcomeWindow<MockAppRuntime>,
    AppDelegate<MockAppRuntime>,
    WelcomeWindowServices,
    ArcContext,
    CleanupFn,
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

    let workspace_edit_backend = WorkspaceFsEditBackend::new(fs.clone(), delegate.workspaces_dir());
    let workspace_edit_service = Arc::new(WorkspaceEditService::new(workspace_edit_backend));
    let workspace_service = Arc::new(WorkspaceService::new(
        WorkspaceServiceFs::new(fs.clone(), delegate.workspaces_dir()),
        storage.clone(),
    ));

    let app_environment_service = AppEnvironmentService::new(AppEnvironmentServiceFs::new(
        &delegate.workspaces_dir(),
        fs.clone(),
    ));

    let environment_ops = WelcomeWindowEnvironmentOps::new(Arc::new(app_environment_service));

    let welcome_window = WelcomeWindow::new(
        &delegate,
        WelcomeWindowWorkspaceOps::new(workspace_service.clone(), workspace_edit_service),
        environment_ops,
    )
    .await
    .unwrap();

    let services = WelcomeWindowServices { workspace_service };

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

    (welcome_window, delegate, services, ctx, cleanup_fn)
}
