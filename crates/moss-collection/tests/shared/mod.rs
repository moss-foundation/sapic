use moss_collection::collection::Collection;
use moss_fs::utils::encode_directory_name;
use moss_fs::RealFileSystem;
use moss_testutils::random_name::random_collection_name;
use std::path::PathBuf;
use std::sync::Arc;

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
    let collection = Collection::new(collection_path.clone(), fs).unwrap();
    (collection_path, collection)
}

pub fn request_folder_name(request_name: &str) -> String {
    format!("{}.request", encode_directory_name(request_name))
}

pub fn request_relative_path(name: &str, relative: Option<&str>) -> PathBuf {
    if relative.is_some() {
        PathBuf::from("requests")
            .join(relative.unwrap())
            .join(request_folder_name(name))
    } else {
        PathBuf::from("requests").join(request_folder_name(name))
    }
}
