#![cfg(feature = "integration-tests")]

use moss_app_delegate::AppDelegate;
use moss_applib::{
    context::{AsyncContext, MutableContext},
    mock::MockAppRuntime,
};
use moss_fs::RealFileSystem;
use moss_project::{
    ProjectBuilder,
    builder::ProjectCreateParams,
    models::{
        operations::CreateEntryInput,
        primitives::{EntryClass, EntryId},
        types::{CreateDirEntryParams, CreateItemEntryParams},
    },
    project::Project,
};
use moss_testutils::random_name::{random_collection_name, random_string};
use nanoid::nanoid;
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

fn random_collection_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join(nanoid!(10))
}

pub async fn create_test_collection() -> (
    AsyncContext,
    AppDelegate<MockAppRuntime>,
    Arc<Path>,
    Project<MockAppRuntime>,
) {
    let mock_app = tauri::test::mock_app();
    let ctx = MutableContext::background_with_timeout(Duration::from_secs(30)).freeze();
    let internal_abs_path = random_collection_path();
    let temp_path = internal_abs_path.join("tmp");

    std::fs::create_dir_all(&internal_abs_path).unwrap();
    std::fs::create_dir_all(&temp_path).unwrap();
    let fs = Arc::new(RealFileSystem::new(&temp_path));

    let abs_path: Arc<Path> = internal_abs_path.clone().into();

    let app_delegate = {
        let delegate = AppDelegate::new(mock_app.handle().clone());
        delegate.set_app_dir(internal_abs_path);
        delegate
    };

    let collection = ProjectBuilder::new(fs)
        .await
        .create(
            &ctx,
            ProjectCreateParams {
                name: Some(random_collection_name()),
                external_abs_path: None,
                internal_abs_path: abs_path.clone(),
                git_params: None,
                icon_path: None,
            },
        )
        .await
        .unwrap();

    (ctx, app_delegate, abs_path, collection)
}

#[allow(dead_code)]
pub async fn create_test_endpoint_dir_entry(
    ctx: &AsyncContext,
    collection: &mut Project<MockAppRuntime>,
    name: &str,
) -> EntryId {
    collection
        .create_entry(
            &ctx,
            CreateEntryInput::Dir(CreateDirEntryParams {
                class: EntryClass::Endpoint,
                path: PathBuf::from(""),
                name: name.to_string(),
                order: 0,
                headers: vec![],
            }),
        )
        .await
        .unwrap()
        .id
}

#[allow(dead_code)]
pub async fn create_test_component_dir_entry(
    ctx: &AsyncContext,
    collection: &mut Project<MockAppRuntime>,
    name: &str,
) -> EntryId {
    collection
        .create_entry(
            &ctx,
            CreateEntryInput::Dir(CreateDirEntryParams {
                class: EntryClass::Component,
                path: PathBuf::from(""),
                name: name.to_string(),
                order: 0,
                headers: vec![],
            }),
        )
        .await
        .unwrap()
        .id
}

#[allow(dead_code)]
pub async fn create_test_component_item_entry(
    ctx: &AsyncContext,
    collection: &mut Project<MockAppRuntime>,
    name: &str,
) -> EntryId {
    collection
        .create_entry(
            &ctx,
            CreateEntryInput::Item(CreateItemEntryParams {
                class: EntryClass::Component,
                path: PathBuf::from(""),
                name: name.to_string(),
                order: 0,
                protocol: None,
                query_params: vec![],
                path_params: vec![],
                headers: vec![],
            }),
        )
        .await
        .unwrap()
        .id
}

#[allow(dead_code)]
pub async fn create_test_schema_dir_entry(
    ctx: &AsyncContext,
    collection: &mut Project<MockAppRuntime>,
    name: &str,
) -> EntryId {
    collection
        .create_entry(
            &ctx,
            CreateEntryInput::Dir(CreateDirEntryParams {
                class: EntryClass::Schema,
                path: PathBuf::from(""),
                name: name.to_string(),
                order: 0,
                headers: vec![],
            }),
        )
        .await
        .unwrap()
        .id
}
