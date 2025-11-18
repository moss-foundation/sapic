#![cfg(feature = "integration-tests")]

use moss_app_delegate::AppDelegate;
use moss_applib::mock::MockAppRuntime;
use moss_fs::RealFileSystem;
use moss_project::{
    ProjectBuilder,
    builder::ProjectCreateParams,
    models::{
        operations::CreateResourceInput,
        primitives::{ResourceClass, ResourceId},
        types::{CreateDirResourceParams, CreateItemResourceParams},
    },
    project::Project,
};
use moss_testutils::random_name::{random_project_name, random_string};
use nanoid::nanoid;
use sapic_core::context::{AsyncContext, MutableContext};
use std::{
    path::{Path, PathBuf},
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

pub async fn create_test_project() -> (
    AsyncContext,
    AppDelegate<MockAppRuntime>,
    Arc<Path>,
    Project<MockAppRuntime>,
    impl FnOnce(),
) {
    let mock_app = tauri::test::mock_app();
    let ctx = MutableContext::background_with_timeout(Duration::from_secs(30)).freeze();
    let test_dir_path = random_test_dir_path();
    let temp_path = test_dir_path.join("tmp");
    let project_path = test_dir_path.join(nanoid!(10));

    std::fs::create_dir_all(&temp_path).unwrap();
    std::fs::create_dir_all(&project_path).unwrap();
    let fs = Arc::new(RealFileSystem::new(&temp_path));

    let app_delegate = {
        let delegate = AppDelegate::new(mock_app.handle().clone());
        delegate
    };

    let project = ProjectBuilder::new(fs)
        .await
        .create(
            &ctx,
            ProjectCreateParams {
                name: Some(random_project_name()),
                external_abs_path: None,
                internal_abs_path: project_path.clone().into(),
                git_params: None,
                icon_path: None,
            },
        )
        .await
        .unwrap();

    let cleanup = || {
        std::fs::remove_dir_all(test_dir_path).unwrap();
    };
    (ctx, app_delegate, project_path.into(), project, cleanup)
}

#[allow(dead_code)]
pub async fn create_test_endpoint_dir_entry(
    ctx: &AsyncContext,
    project: &mut Project<MockAppRuntime>,
    name: &str,
) -> ResourceId {
    project
        .create_resource(
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
    project: &mut Project<MockAppRuntime>,
    name: &str,
) -> ResourceId {
    project
        .create_resource(
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
    project: &mut Project<MockAppRuntime>,
    name: &str,
) -> ResourceId {
    project
        .create_resource(
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
    project: &mut Project<MockAppRuntime>,
    name: &str,
) -> ResourceId {
    project
        .create_resource(
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
