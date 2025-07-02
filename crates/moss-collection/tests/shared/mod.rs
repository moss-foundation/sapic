use moss_collection::collection::{Collection, CreateParams};
use moss_fs::RealFileSystem;
use moss_testutils::random_name::{random_collection_name, random_string};
use std::{path::PathBuf, sync::Arc};
use uuid::Uuid;

#[allow(dead_code)]
pub fn random_dir_name() -> String {
    format!("Test_{}_Dir", random_string(10))
}

fn random_collection_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join(Uuid::new_v4().to_string())
}

pub async fn create_test_collection() -> (PathBuf, Collection) {
    let fs = Arc::new(RealFileSystem::new());
    let internal_abs_path = random_collection_path();

    std::fs::create_dir_all(internal_abs_path.clone()).unwrap();

    let collection = Collection::create(
        fs,
        CreateParams {
            name: Some(random_collection_name()),
            external_abs_path: None,
            repository: None,
            internal_abs_path: &internal_abs_path,
            icon_path: None,
        },
    )
    .await
    .unwrap();

    // Base directories (requests, endpoints, components, schemas, environments)
    // are now created automatically by Collection::create

    (internal_abs_path, collection)
}

// Removed unused helper functions that depend on types not available in this crate
