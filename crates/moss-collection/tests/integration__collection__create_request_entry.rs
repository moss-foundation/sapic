mod shared;

use std::path::PathBuf;

use moss_collection::models::operations::CreateRequestEntryInput;
use moss_testutils::random_name::random_request_name;

use crate::shared::{request_folder_name, request_relative_path, set_up_test_collection};

#[tokio::test]
async fn create_request_success() {
    let (collection_path, collection) = set_up_test_collection().await;
    let request_name = random_request_name();

    let create_request_result = collection
        .create_request_entry(CreateRequestEntryInput {
            destination: PathBuf::from("requests").join(request_name.to_string()),
            url: None,
            payload: None,
        })
        .await
        .unwrap();
}
