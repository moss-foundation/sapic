mod shared;

use crate::shared::{random_request_dir_name, set_up_test_collection};
use moss_collection::models::operations::{
    CreateRequestEntryInput, CreateRequestProtocolSpecificPayload,
};
use moss_collection::models::types::{HttpMethod, PathChangeKind};
use moss_common::api::{OperationError, OperationResult};
use moss_fs::utils::encode_name;
use moss_testutils::fs_specific::FOLDERNAME_SPECIAL_CHARS;
use moss_testutils::random_name::random_request_name;
use std::path::PathBuf;

#[tokio::test]
async fn create_request_entry_success() {
    let (collection_path, collection) = set_up_test_collection().await;
    let request_name = random_request_name();

    let create_result = collection
        .create_request_entry(CreateRequestEntryInput {
            destination: PathBuf::from("requests").join(request_name.to_string()),
            url: None,
            payload: None,
        })
        .await;

    let changed_paths = create_result.unwrap().changed_paths;
    // Current entry list:
    // requests
    // requests\\{request_name}.request
    // requests\\{request_name}.request\\get.sapic

    dbg!(&changed_paths);
    assert_eq!(changed_paths.len(), 3);
    assert!(changed_paths.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == PathBuf::from("requests") && kind == &PathChangeKind::Created
    }));
    assert!(changed_paths.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == PathBuf::from("requests").join(format!("{request_name}.request"))
            && kind == &PathChangeKind::Created
    }));
    assert!(changed_paths.iter().any(|(path, _id, kind)| {
        path.to_path_buf()
            == PathBuf::from("requests")
                .join(format!("{request_name}.request"))
                .join("get.sapic")
            && kind == &PathChangeKind::Created
    }));

    // Clean up
    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}

#[tokio::test]
async fn create_request_entry_with_payload() {
    let (collection_path, collection) = set_up_test_collection().await;
    let request_name = random_request_name();

    let create_result = collection
        .create_request_entry(CreateRequestEntryInput {
            destination: PathBuf::from("requests").join(request_name.to_string()),
            url: None,
            payload: Some(CreateRequestProtocolSpecificPayload::Http {
                method: HttpMethod::Post,
                query_params: vec![],
                path_params: vec![],
                headers: vec![],
                body: None,
            }),
        })
        .await;

    let changed_paths = create_result.unwrap().changed_paths;

    // Current entry list:
    // requests
    // requests\\{request_name}.request
    // requests\\{request_name}.request\\post.sapic

    assert_eq!(changed_paths.len(), 3);
    assert!(changed_paths.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == PathBuf::from("requests") && kind == &PathChangeKind::Created
    }));
    assert!(changed_paths.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == PathBuf::from("requests").join(format!("{request_name}.request"))
            && kind == &PathChangeKind::Created
    }));
    assert!(changed_paths.iter().any(|(path, _id, kind)| {
        path.to_path_buf()
            == PathBuf::from("requests")
                .join(format!("{request_name}.request"))
                .join("post.sapic")
            && kind == &PathChangeKind::Created
    }));

    // Clean up
    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}

#[tokio::test]
async fn create_request_entry_already_exists() {
    let (collection_path, collection) = set_up_test_collection().await;
    let request_name = random_request_name();

    let _ = collection
        .create_request_entry(CreateRequestEntryInput {
            destination: PathBuf::from("requests").join(request_name.to_string()),
            url: None,
            payload: None,
        })
        .await
        .unwrap();

    let create_result = collection
        .create_request_entry(CreateRequestEntryInput {
            destination: PathBuf::from("requests").join(request_name.to_string()),
            url: None,
            payload: None,
        })
        .await;

    assert!(matches!(
        create_result,
        OperationResult::Err(OperationError::AlreadyExists { .. })
    ));

    // Clean up
    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}

#[tokio::test]
async fn create_request_entry_multiple_same_level() {
    let (collection_path, collection) = set_up_test_collection().await;
    let request_name1 = "1";
    let request_name2 = "2";

    let _ = collection
        .create_request_entry(CreateRequestEntryInput {
            destination: PathBuf::from("requests").join(&request_name1),
            url: None,
            payload: None,
        })
        .await
        .unwrap();

    let create_result = collection
        .create_request_entry(CreateRequestEntryInput {
            destination: PathBuf::from("requests").join(&request_name2),
            url: None,
            payload: None,
        })
        .await;
    let changed_paths = create_result.unwrap().changed_paths;

    assert_eq!(changed_paths.len(), 2);
    assert!(changed_paths.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == PathBuf::from("requests").join(format!("{request_name2}.request"))
            && kind == &PathChangeKind::Created
    }));
    assert!(changed_paths.iter().any(|(path, _id, kind)| {
        path.to_path_buf()
            == PathBuf::from("requests")
                .join(format!("{request_name2}.request"))
                .join("get.sapic")
            && kind == &PathChangeKind::Created
    }));

    // Clean up
    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}

#[tokio::test]
async fn create_request_entry_nested() {
    let (collection_path, collection) = set_up_test_collection().await;
    let request_name = random_request_name();

    let create_result = collection
        .create_request_entry(CreateRequestEntryInput {
            destination: PathBuf::from("requests").join("group").join(&request_name),
            url: None,
            payload: None,
        })
        .await;
    let changed_paths = create_result.unwrap().changed_paths;

    assert_eq!(changed_paths.len(), 4);
    assert!(changed_paths.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == PathBuf::from("requests") && kind == &PathChangeKind::Created
    }));
    assert!(changed_paths.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == PathBuf::from("requests").join("group")
            && kind == &PathChangeKind::Created
    }));
    assert!(changed_paths.iter().any(|(path, _id, kind)| {
        path.to_path_buf()
            == PathBuf::from("requests")
                .join("group")
                .join(format!("{request_name}.request"))
            && kind == &PathChangeKind::Created
    }));
    assert!(changed_paths.iter().any(|(path, _id, kind)| {
        path.to_path_buf()
            == PathBuf::from("requests")
                .join("group")
                .join(format!("{request_name}.request"))
                .join("get.sapic")
    }));

    // Clean up
    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}

#[tokio::test]
async fn create_request_entry_multiple_nested() {
    // requests\1.request
    // requests\group\2.request

    let (collection_path, collection) = set_up_test_collection().await;
    let request_name1 = "1";
    let request_name2 = "2";

    let _ = collection
        .create_request_entry(CreateRequestEntryInput {
            destination: PathBuf::from("requests").join(request_name1),
            url: None,
            payload: None,
        })
        .await
        .unwrap();

    let create_result = collection
        .create_request_entry(CreateRequestEntryInput {
            destination: PathBuf::from("requests").join("group").join(request_name2),
            url: None,
            payload: None,
        })
        .await;
    let changed_paths = create_result.unwrap().changed_paths;

    assert_eq!(changed_paths.len(), 3);
    assert!(changed_paths.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == PathBuf::from("requests").join("group")
            && kind == &PathChangeKind::Created
    }));
    assert!(changed_paths.iter().any(|(path, _id, kind)| {
        path.to_path_buf()
            == PathBuf::from("requests")
                .join("group")
                .join(format!("{request_name2}.request"))
            && kind == &PathChangeKind::Created
    }));
    assert!(changed_paths.iter().any(|(path, _id, kind)| {
        path.to_path_buf()
            == PathBuf::from("requests")
                .join("group")
                .join(format!("{request_name2}.request"))
                .join("get.sapic")
            && kind == &PathChangeKind::Created
    }));

    // Clean up
    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}

#[tokio::test]
async fn create_request_entry_special_chars_in_name() {
    let (collection_path, collection) = set_up_test_collection().await;

    let request_name = random_request_name();
    let request_name_list = FOLDERNAME_SPECIAL_CHARS
        .into_iter()
        .map(|s| format!("{}{s}", request_name))
        .collect::<Vec<_>>();

    for name in request_name_list {
        let create_result = collection
            .create_request_entry(CreateRequestEntryInput {
                destination: PathBuf::from("requests").join(&name),
                url: None,
                payload: None,
            })
            .await;
        let changed_paths = create_result.unwrap().changed_paths;

        assert!(changed_paths.iter().any(|(path, _id, kind)| {
            path.to_path_buf()
                == PathBuf::from("requests").join(format!("{}.request", encode_name(&name)))
                && kind == &PathChangeKind::Created
        }));
        assert!(changed_paths.iter().any(|(path, _id, kind)| {
            path.to_path_buf()
                == PathBuf::from("requests")
                    .join(format!("{}.request", encode_name(&name)))
                    .join("get.sapic")
                && kind == &PathChangeKind::Created
        }));
    }
    // Clean up
    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}

#[tokio::test]
async fn create_request_entry_special_chars_in_path() {
    let (collection_path, collection) = set_up_test_collection().await;
    let request_dir_name = random_request_dir_name();
    let request_dir_list = FOLDERNAME_SPECIAL_CHARS
        .into_iter()
        .map(|s| format!("{}{s}", request_dir_name))
        .collect::<Vec<_>>();
    for name in request_dir_list {
        let create_result = collection
            .create_request_entry(CreateRequestEntryInput {
                destination: PathBuf::from("requests").join(&name).join("request"),
                url: None,
                payload: None,
            })
            .await;
        let changed_paths = create_result.unwrap().changed_paths;

        assert!(changed_paths.iter().any(|(path, _id, kind)| {
            path.to_path_buf()
                == PathBuf::from("requests")
                    .join(encode_name(&name))
                    .join("request.request")
                && kind == &PathChangeKind::Created
        }));
        assert!(changed_paths.iter().any(|(path, _id, kind)| {
            path.to_path_buf()
                == PathBuf::from("requests")
                    .join(encode_name(&name))
                    .join("request.request")
                    .join("get.sapic")
                && kind == &PathChangeKind::Created
        }));
    }
    // Clean up
    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}
