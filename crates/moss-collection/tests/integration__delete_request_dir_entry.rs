mod shared;

use std::path::PathBuf;

use moss_collection::models::operations::{CreateRequestEntryInput, DeleteRequestDirEntryInput};
use moss_testutils::random_name::random_request_name;

use crate::shared::set_up_test_collection;

#[tokio::test]
async fn delete_request_dir_entry() {
    let (_collection_path, collection) = set_up_test_collection().await;
    let request_name = random_request_name();

    let destination = PathBuf::from("requests")
        .join("test")
        .join(request_name.to_string());

    let create_request_result = collection
        .create_request_entry(CreateRequestEntryInput {
            destination: destination.clone(),
            url: None,
            payload: None,
        })
        .await
        .unwrap();

    dbg!(&create_request_result);

    let id = create_request_result
        .changed_paths
        .into_iter()
        .find(|&v| v.0 == destination.parent().unwrap().into())
        .unwrap()
        .1;

    let delete_request_dir_entry_result = collection
        .delete_request_dir_entry(DeleteRequestDirEntryInput { id })
        .await
        .unwrap();

    dbg!(&delete_request_dir_entry_result);
}
