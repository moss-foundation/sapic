use moss_collection::collection::Collection;
use moss_collection::indexer::{self, IndexerHandle};
use moss_fs::utils::{encode_name, encode_path};
use moss_fs::RealFileSystem;
use moss_testutils::random_name::random_collection_name;
use moss_workbench::activity_indicator::ActivityIndicator;
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

    let mock_app = tauri::test::mock_app();
    let app_handle = mock_app.handle().clone();

    let (job_sender, job_receiver) = mpsc::unbounded_channel();
    let activity_indicator = ActivityIndicator::new(app_handle);
    let indexer_handle = IndexerHandle::new(job_sender);

    {
        tauri::async_runtime::spawn({
            let fs_clone = Arc::clone(&fs);
            let activity_indicator_clone = activity_indicator.clone();

            async move {
                indexer::run(activity_indicator_clone, fs_clone, job_receiver).await;
            }
        });
    }

    let collection = Collection::new(collection_path.clone(), fs, indexer_handle).unwrap();

    // Normally, the indexation process will require the tauri application to be running
    // We will bypass the indexation process for test purposes by calling `list_requests()` at first
    // Since the requests folder will not be created yet, this allows us to bypass the indexation process
    collection.list_requests().await.unwrap();

    (collection_path, collection)
}

pub fn request_folder_name(request_name: &str) -> String {
    format!("{}.request", encode_name(request_name))
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
