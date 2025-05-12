mod shared;

use std::path::PathBuf;

use moss_collection::models::operations::{CreateRequestDirEntryInput, CreateRequestEntryInput};
use moss_testutils::random_name::random_request_name;

use crate::shared::{request_folder_name, request_relative_path, set_up_test_collection};

#[tokio::test]
async fn rename_request_dir_entry() {
    let (collection_path, collection) = set_up_test_collection().await;
    let request_name = random_request_name();

    let create_request_dir_entry_result = collection
        .create_request_dir_entry(CreateRequestDirEntryInput {
            destination: PathBuf::from("requests").join(request_name.to_string()),
        })
        .await
        .unwrap();

    // let new_name = random_request_name();
    // let update_request_dir_entry_result = collection
    //     .update_request_dir_entry(UpdateRequestDirEntryInput {
    //         id: create_request_result.key,
    //         name: Some(new_name),
    //     })
    //     .await
    //     .unwrap();
}
