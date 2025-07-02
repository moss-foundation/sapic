use moss_collection::{
    CollectionBuilder,
    builder::CreateParams,
    collection::Collection,
    services::{storage_service::StorageService, worktree_service::WorktreeService},
};
use moss_common::nanoid::new_nanoid;
use moss_fs::RealFileSystem;
use moss_testutils::random_name::{random_collection_name, random_string};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

#[allow(dead_code)]
pub fn random_dir_name() -> String {
    format!("Test_{}_Dir", random_string(10))
}

fn random_collection_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join(new_nanoid())
}

pub async fn create_test_collection() -> (Arc<Path>, Collection) {
    let fs = Arc::new(RealFileSystem::new());
    let internal_abs_path = random_collection_path();

    std::fs::create_dir_all(internal_abs_path.clone()).unwrap();

    let abs_path: Arc<Path> = internal_abs_path.clone().into();
    let storage = Arc::new(StorageService::new(&abs_path).unwrap());
    let worktree = WorktreeService::new(abs_path.clone(), fs.clone(), storage.clone());
    let collection = CollectionBuilder::new(fs)
        .with_service::<StorageService>(storage)
        .with_service(worktree)
        .create(CreateParams {
            name: Some(random_collection_name()),
            external_abs_path: None,
            repository: None,
            internal_abs_path: abs_path.clone(),
            icon_path: None,
        })
        .await
        .unwrap();

    (abs_path, collection)
}
