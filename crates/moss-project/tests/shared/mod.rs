#![allow(deprecated)] // TODO: remove once we get rid of old context types
#![cfg(feature = "integration-tests")]

use moss_app_delegate::AppDelegate;
use moss_applib::mock::MockAppRuntime;
use moss_fs::RealFileSystem;
use moss_project::{
    ProjectBuilder,
    builder::ProjectCreateParams,
    models::{
        operations::CreateResourceInput,
        primitives::{ProjectId, ResourceClass, ResourceId},
        types::{CreateDirResourceParams, CreateItemResourceParams},
    },
    project::Project,
};
use moss_storage2::SubstoreManager;
use moss_testutils::random_name::{random_project_name, random_string};
use nanoid::nanoid;
use sapic_core::context::{AsyncContext, MutableContext};
use sapic_runtime::{
    app::kv_storage::{AppStorage, AppStorageOptions},
    globals::GlobalKvStorage,
};
use std::{
    path::{Path, PathBuf},
    pin::Pin,
    sync::Arc,
    time::Duration,
};

pub const RESOURCES_ROOT_DIR: &str = "";

#[allow(dead_code)]
pub fn random_dir_name() -> String {
    format!("Test_{}_Dir", random_string(10))
}

pub fn random_entry_name() -> String {
    format!("Test_{}_Entry", random_string(10))
}

fn random_test_dir_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join(nanoid!(10))
}
pub type CleanupFn = Box<dyn FnOnce() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send>;

pub async fn create_test_project() -> (
    AsyncContext,
    AppDelegate<MockAppRuntime>,
    Arc<Path>,
    Project,
    CleanupFn,
) {
    let mock_app = tauri::test::mock_app();
    let ctx = MutableContext::background_with_timeout(Duration::from_secs(30)).freeze();
    let test_dir_path = random_test_dir_path();
    let temp_path = test_dir_path.join("tmp");
    let globals_path = test_dir_path.join("globals");
    let workspaces_path = test_dir_path.join("workspaces");

    let workspace_id = nanoid!(10);
    let project_id = ProjectId::new();
    let project_path = workspaces_path
        .join(&workspace_id)
        .join("projects")
        .join(project_id.as_str());

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

    std::fs::create_dir_all(&globals_path).unwrap();
    std::fs::create_dir_all(&temp_path).unwrap();
    std::fs::create_dir_all(&project_path).unwrap();
    let fs = Arc::new(RealFileSystem::new(&temp_path));

    let app_delegate = {
        let delegate = AppDelegate::new(mock_app.handle().clone());
        GlobalKvStorage::set(&delegate, app_storage.clone());
        delegate
    };

    let storage = GlobalKvStorage::get(&app_delegate);
    let project = ProjectBuilder::new(fs, storage, project_id.clone())
        .await
        .create(ProjectCreateParams {
            name: Some(random_project_name()),
            external_abs_path: None,
            internal_abs_path: project_path.clone().into(),
            git_params: None,
            icon_path: None,
        })
        .await
        .unwrap();

    // let cleanup = || {
    //     std::fs::remove_dir_all(test_dir_path).unwrap();
    // };
    let cleanup_fn = Box::new({
        let test_dir_path_clone = test_dir_path.clone();
        let app_storage_clone = app_storage.clone();
        move || {
            Box::pin(async move {
                app_storage_clone.close().await.unwrap();
                // Looks like some delay is necessary to release SQLite file handle
                tokio::time::sleep(Duration::from_millis(100)).await;
                if let Err(e) = tokio::fs::remove_dir_all(&test_dir_path_clone).await {
                    eprintln!("Failed to clean up test workspace directory: {}", e);
                }
            }) as Pin<Box<dyn Future<Output = ()> + Send>>
        }
    });

    // Add workspace and project storage
    app_storage
        .add_workspace(workspace_id.clone().into())
        .await
        .unwrap();
    app_storage
        .add_project(workspace_id.into(), project_id.inner())
        .await
        .unwrap();
    (ctx, app_delegate, project_path.into(), project, cleanup_fn)
}

#[allow(dead_code)]
pub async fn create_test_endpoint_dir_entry(
    ctx: &AsyncContext,
    project: &mut Project,
    name: &str,
) -> ResourceId {
    project
        .create_resource::<MockAppRuntime>(
            &ctx,
            CreateResourceInput::Dir(CreateDirResourceParams {
                class: ResourceClass::Endpoint,
                path: PathBuf::from(""),
                name: name.to_string(),
                order: 0,
            }),
        )
        .await
        .unwrap()
        .id
}

#[allow(dead_code)]
pub async fn create_test_component_dir_entry(
    ctx: &AsyncContext,
    project: &mut Project,
    name: &str,
) -> ResourceId {
    project
        .create_resource::<MockAppRuntime>(
            &ctx,
            CreateResourceInput::Dir(CreateDirResourceParams {
                class: ResourceClass::Component,
                path: PathBuf::from(""),
                name: name.to_string(),
                order: 0,
            }),
        )
        .await
        .unwrap()
        .id
}

#[allow(dead_code)]
pub async fn create_test_component_item_entry(
    ctx: &AsyncContext,
    project: &mut Project,
    name: &str,
) -> ResourceId {
    project
        .create_resource::<MockAppRuntime>(
            &ctx,
            CreateResourceInput::Item(CreateItemResourceParams {
                class: ResourceClass::Component,
                path: PathBuf::from(""),
                name: name.to_string(),
                order: 0,
                protocol: None,
                query_params: vec![],
                path_params: vec![],
                headers: vec![],
                body: None,
            }),
        )
        .await
        .unwrap()
        .id
}

#[allow(dead_code)]
pub async fn create_test_schema_dir_entry(
    ctx: &AsyncContext,
    project: &mut Project,
    name: &str,
) -> ResourceId {
    project
        .create_resource::<MockAppRuntime>(
            &ctx,
            CreateResourceInput::Dir(CreateDirResourceParams {
                class: ResourceClass::Schema,
                path: PathBuf::from(""),
                name: name.to_string(),
                order: 0,
            }),
        )
        .await
        .unwrap()
        .id
}
