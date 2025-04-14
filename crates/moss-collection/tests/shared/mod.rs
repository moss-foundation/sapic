use moss_collection::collection::Collection;
use moss_collection::indexer::IndexerHandle;
use moss_fs::utils::{encode_directory_name, encode_path};
use moss_fs::RealFileSystem;
use moss_testutils::random_name::random_collection_name;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::mpsc;

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

    let (job_sender, _job_receiver) = mpsc::unbounded_channel();
    let indexer_handle = IndexerHandle::new(job_sender);
    let collection = Collection::new(collection_path.clone(), fs, indexer_handle).unwrap();

    // Normally, the indexation process will require the tauri application to be running
    // We will bypass the indexation process for test purposes by calling `list_requests()` at first
    // Since the requests folder will not be created yet, this allows us to bypass the indexation process
    collection.list_requests().await.unwrap();
    (collection_path, collection)
}

pub fn request_folder_name(request_name: &str) -> String {
    format!("{}.request", encode_directory_name(request_name))
}

/// Generate the encoded relative path of request from the collection folder
pub fn request_relative_path(name: &str, relative: Option<&Path>) -> PathBuf {
    if let Some(relative) = relative {
        PathBuf::from("requests")
            .join(encode_path(None, relative).unwrap())
            .join(request_folder_name(name))
    } else {
        PathBuf::from("requests").join(request_folder_name(name))
    }
}

/// Generate the encoded relative path of request group from the collection folder
pub fn request_group_relative_path(path: &Path) -> PathBuf {
    PathBuf::from("requests")
        .join(encode_path(None, path).unwrap())
}
