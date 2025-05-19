use moss_collection::collection::Collection;
use moss_common::sanitized::sanitized_name::SanitizedName;
use moss_fs::RealFileSystem;
use moss_fs::utils::encode_path;
use moss_testutils::random_name::{random_collection_name, random_string};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;

pub fn random_request_dir_name() -> String {
    format!("Test_{}_Dir", random_string(10))
}

pub fn random_collection_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join(random_collection_name())
}

pub async fn set_up_test_collection() -> (PathBuf, Collection) {
    let fs = Arc::new(RealFileSystem::new());
    let collection_path = random_collection_path();

    std::fs::create_dir_all(collection_path.clone()).unwrap();

    // Create collection/requests to prevent indexation error
    // std::fs::create_dir_all(collection_path.join("requests")).unwrap();

    let next_entry_id = Arc::new(AtomicUsize::new(0));
    let collection = Collection::new(collection_path.clone(), fs, next_entry_id).unwrap();

    (collection_path, collection)
}
/// Generate the encoded request folder name
pub fn request_folder_name(request_name: &str) -> String {
    let sanitized_name = SanitizedName::new(request_name);
    format!("{}.request", &sanitized_name)
}

/// Generate the encoded relative path of request from the collection folder
pub fn request_relative_path(name: &str, relative: Option<&Path>) -> PathBuf {
    if let Some(relative) = relative {
        PathBuf::from("requests")
            .join(encode_path(relative, None).unwrap())
            .join(request_folder_name(name))
    } else {
        PathBuf::from("requests").join(request_folder_name(name))
    }
}

/// Generate the encoded relative path of request group from the collection folder
pub fn request_group_relative_path(path: &Path) -> PathBuf {
    PathBuf::from("requests").join(encode_path(path, None).unwrap())
}
