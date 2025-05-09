mod shared;

use std::path::PathBuf;

use moss_collection::models::operations::{CreateRequestDirEntryInput, UpdateRequestDirEntryInput};
use moss_testutils::random_name::random_request_name;

use crate::shared::set_up_test_collection;

#[tokio::test]
async fn rename_request_dir_entry() {
    let (_collection_path, collection) = set_up_test_collection().await;
    let request_name = random_request_name();

    let destination = PathBuf::from("requests")
        .join("test")
        .join(request_name.to_string());

    let create_request_dir_entry_result = collection
        .create_request_dir_entry(CreateRequestDirEntryInput {
            destination: destination.clone(),
        })
        .await
        .unwrap();

    dbg!(&create_request_dir_entry_result);

    let id = create_request_dir_entry_result
        .changed_paths
        .into_iter()
        .find(|&v| v.0 == PathBuf::from("requests").into())
        .unwrap()
        .1;

    let update_request_dir_entry_result = collection
        .update_request_dir_entry(UpdateRequestDirEntryInput {
            id,
            name: Some("new_name".to_string()),
        })
        .await
        .unwrap();

    dbg!(&update_request_dir_entry_result);
}
