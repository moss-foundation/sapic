use crate::shared::{random_request_dir_name, request_folder_name, set_up_test_collection};
use moss_collection::collection;
use moss_collection::models::operations::{CreateEntryInput, CreateEntryOutput};
use moss_collection::models::types::{Classification, PathChangeKind};
use moss_common::api::OperationError;
use moss_fs::utils::encode_name;
use moss_testutils::fs_specific::FOLDERNAME_SPECIAL_CHARS;
use moss_testutils::random_name::random_request_name;
use serde_json::Value as JsonValue;
use serde_json::json;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

mod shared;

// TODO: Test writing the spec file content correctly

#[tokio::test]
async fn create_entry_request_default_spec() {
    let (collection_path, collection) = set_up_test_collection().await;
    let request_name = random_request_name();

    let create_result = collection
        .create_entry(CreateEntryInput {
            destination: PathBuf::from("requests").join(&request_name),
            classification: Classification::Request,
            specification: None,
            protocol: None,
            order: None,
            is_dir: false,
        })
        .await;

    let CreateEntryOutput {
        physical_changes,
        virtual_changes,
    } = create_result.unwrap();

    // Physical
    // requests
    // requests\\{request_name}.request
    // requests.\\{request_name}.request\\get.sapic

    // Virtual
    // requests
    // requests\\{request_name}

    assert_eq!(physical_changes.len(), 3);
    assert!(physical_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == Path::new("requests") && kind == &PathChangeKind::Created
    }));
    assert!(physical_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == Path::new("requests").join(request_folder_name(&request_name))
            && kind == &PathChangeKind::Created
    }));
    assert!(physical_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf()
            == Path::new("requests")
                .join(request_folder_name(&request_name))
                .join("get.sapic")
            && kind == &PathChangeKind::Created
    }));

    assert_eq!(virtual_changes.len(), 2);
    assert!(virtual_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == Path::new("requests") && kind == &PathChangeKind::Created
    }));
    assert!(virtual_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == Path::new("requests").join(&request_name)
            && kind == &PathChangeKind::Created
    }));

    let specfile_path = collection_path
        .join("requests")
        .join(request_folder_name(&request_name))
        .join("get.sapic");
    assert!(specfile_path.exists());
    tokio::fs::remove_dir_all(collection_path).await.unwrap();
}

#[tokio::test]
async fn create_entry_request_with_spec_content() {
    let (collection_path, collection) = set_up_test_collection().await;
    let request_name = random_request_name();

    let create_result = collection
        .create_entry(CreateEntryInput {
            destination: PathBuf::from("requests").join(&request_name),
            classification: Classification::Request,
            specification: Some(json!(42)),
            protocol: Some("post".to_string()),
            order: None,
            is_dir: false,
        })
        .await;

    let CreateEntryOutput {
        physical_changes,
        virtual_changes,
    } = create_result.unwrap();

    // Physical
    // requests
    // requests\\{request_name}.request
    // requests.\\{request_name}.request\\post.sapic

    // Virtual
    // requests
    // requests\\{request_name}

    assert_eq!(physical_changes.len(), 3);
    assert!(physical_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == Path::new("requests") && kind == &PathChangeKind::Created
    }));
    assert!(physical_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == Path::new("requests").join(request_folder_name(&request_name))
            && kind == &PathChangeKind::Created
    }));
    assert!(physical_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf()
            == Path::new("requests")
                .join(request_folder_name(&request_name))
                .join("post.sapic")
            && kind == &PathChangeKind::Created
    }));

    assert_eq!(virtual_changes.len(), 2);
    assert!(virtual_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == Path::new("requests") && kind == &PathChangeKind::Created
    }));
    assert!(virtual_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == Path::new("requests").join(&request_name)
            && kind == &PathChangeKind::Created
    }));

    let specfile_path = collection_path
        .join("requests")
        .join(request_folder_name(&request_name))
        .join("post.sapic");
    assert!(specfile_path.exists());

    let restored_content: JsonValue =
        serde_json::from_str(&read_to_string(&specfile_path).unwrap()).unwrap();
    assert_eq!(restored_content, json!(42));

    tokio::fs::remove_dir_all(collection_path).await.unwrap();
}

#[tokio::test]
async fn create_entry_request_already_exists() {
    let (collection_path, collection) = set_up_test_collection().await;
    let request_name = random_request_name();

    let input = CreateEntryInput {
        destination: PathBuf::from("requests").join(&request_name),
        classification: Classification::Request,
        specification: None,
        protocol: None,
        order: None,
        is_dir: false,
    };
    let _ = collection.create_entry(input.clone()).await;
    let create_result = collection.create_entry(input.clone()).await;
    assert!(matches!(
        create_result,
        Err(OperationError::AlreadyExists(..))
    ));

    tokio::fs::remove_dir_all(collection_path).await.unwrap();
}

#[tokio::test]
async fn create_entry_request_multiple_same_level() {
    let (collection_path, collection) = set_up_test_collection().await;
    let request_name1 = "1";
    let request_name2 = "2";

    let _ = collection
        .create_entry(CreateEntryInput {
            destination: PathBuf::from("requests").join(&request_name1),
            classification: Classification::Request,
            specification: None,
            protocol: None,
            order: None,
            is_dir: false,
        })
        .await
        .unwrap();

    let create_result = collection
        .create_entry(CreateEntryInput {
            destination: PathBuf::from("requests").join(&request_name2),
            classification: Classification::Request,
            specification: None,
            protocol: None,
            order: None,
            is_dir: false,
        })
        .await;

    // Physical
    // requests\\2.request
    // requests\\2.request\\get.sapic

    // Virtual
    // requests\\2

    let CreateEntryOutput {
        physical_changes,
        virtual_changes,
    } = create_result.unwrap();

    assert_eq!(physical_changes.len(), 2);
    assert!(physical_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == Path::new("requests").join("2.request")
            && kind == &PathChangeKind::Created
    }));
    assert!(physical_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == Path::new("requests").join("2.request").join("get.sapic")
            && kind == &PathChangeKind::Created
    }));

    assert_eq!(virtual_changes.len(), 1);
    assert!(virtual_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == Path::new("requests").join("2") && kind == &PathChangeKind::Created
    }));
    assert!(
        collection_path
            .join("requests")
            .join("1.request")
            .join("get.sapic")
            .exists()
    );
    assert!(
        collection_path
            .join("requests")
            .join("2.request")
            .join("get.sapic")
            .exists()
    );
    tokio::fs::remove_dir_all(collection_path).await.unwrap();
}

#[tokio::test]
async fn create_entry_request_nested() {
    let (collection_path, collection) = set_up_test_collection().await;
    let request_name = random_request_name();

    let create_result = collection
        .create_entry(CreateEntryInput {
            destination: Path::new("requests").join("folder").join(&request_name),
            classification: Classification::Request,
            specification: None,
            protocol: None,
            order: None,
            is_dir: false,
        })
        .await;

    // Physical
    // requests
    // requests\\folder
    // requests\\folder\\{request_name}.request
    // requests\\folder\\{request_name}.request\\get.sapic

    // Virtual
    // requests
    // requests\\folder
    // requests\\folder\\{request_name}

    let CreateEntryOutput {
        physical_changes,
        virtual_changes,
    } = create_result.unwrap();

    assert_eq!(physical_changes.len(), 4);
    assert!(physical_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == Path::new("requests") && kind == &PathChangeKind::Created
    }));
    assert!(physical_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == Path::new("requests").join("folder")
            && kind == &PathChangeKind::Created
    }));
    assert!(physical_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf()
            == Path::new("requests")
                .join("folder")
                .join(request_folder_name(&request_name))
            && kind == &PathChangeKind::Created
    }));
    assert!(physical_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf()
            == Path::new("requests")
                .join("folder")
                .join(request_folder_name(&request_name))
                .join("get.sapic")
            && kind == &PathChangeKind::Created
    }));

    assert_eq!(virtual_changes.len(), 3);
    assert!(virtual_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == Path::new("requests") && kind == &PathChangeKind::Created
    }));
    assert!(virtual_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == Path::new("requests").join("folder")
            && kind == &PathChangeKind::Created
    }));
    assert!(virtual_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == Path::new("requests").join("folder").join(&request_name)
            && kind == &PathChangeKind::Created
    }));

    let specfile_path = collection_path
        .join("requests")
        .join("folder")
        .join(request_folder_name(&request_name))
        .join("get.sapic");
    assert!(specfile_path.exists());

    tokio::fs::remove_dir_all(collection_path).await.unwrap();
}

#[tokio::test]
async fn create_entry_request_multiple_different_level() {
    let (collection_path, collection) = set_up_test_collection().await;
    // requests\1.request
    // requests\group\2.request

    let _ = collection
        .create_entry(CreateEntryInput {
            destination: Path::new("requests").join("1"),
            classification: Classification::Request,
            specification: None,
            protocol: None,
            order: None,
            is_dir: false,
        })
        .await
        .unwrap();

    let create_result = collection
        .create_entry(CreateEntryInput {
            destination: Path::new("requests").join("group").join("2"),
            classification: Classification::Request,
            specification: None,
            protocol: None,
            order: None,
            is_dir: false,
        })
        .await;

    // Physical
    // requests\\group
    // requests\\group\\2.request
    // requests\\group\\2.request\\get.sapic

    // Virtual
    // requests\\group
    // requests\\group\\2

    let CreateEntryOutput {
        physical_changes,
        virtual_changes,
    } = create_result.unwrap();

    assert_eq!(physical_changes.len(), 3);
    assert!(physical_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == Path::new("requests").join("group")
            && kind == &PathChangeKind::Created
    }));
    assert!(physical_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == Path::new("requests").join("group").join("2.request")
            && kind == &PathChangeKind::Created
    }));
    assert!(physical_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf()
            == Path::new("requests")
                .join("group")
                .join("2.request")
                .join("get.sapic")
            && kind == &PathChangeKind::Created
    }));

    assert_eq!(virtual_changes.len(), 2);
    assert!(virtual_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == Path::new("requests").join("group")
            && kind == &PathChangeKind::Created
    }));
    assert!(virtual_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == Path::new("requests").join("group").join("2")
            && kind == &PathChangeKind::Created
    }));

    let specfile_path1 = collection_path
        .join("requests")
        .join("1.request")
        .join("get.sapic");
    let specfile_path2 = collection_path
        .join("requests")
        .join("group")
        .join("2.request")
        .join("get.sapic");
    assert!(specfile_path1.exists());
    assert!(specfile_path2.exists());

    tokio::fs::remove_dir_all(collection_path).await.unwrap();
}

#[tokio::test]
async fn create_entry_request_special_chars_in_name() {
    let (collection_path, collection) = set_up_test_collection().await;

    let request_name = random_request_name();
    let request_name_list = FOLDERNAME_SPECIAL_CHARS
        .into_iter()
        .map(|s| format!("{}{s}", request_name))
        .collect::<Vec<_>>();

    for name in request_name_list {
        let create_result = collection
            .create_entry(CreateEntryInput {
                destination: Path::new("requests").join(&name),
                classification: Classification::Request,
                specification: None,
                protocol: None,
                order: None,
                is_dir: false,
            })
            .await;

        // Physical
        // requests\\{encoded_name}.request
        // requests\\{encoded_name}.request\\get.sapic

        // Virtual
        // requests\\{name}

        let CreateEntryOutput {
            physical_changes,
            virtual_changes,
        } = create_result.unwrap();

        assert!(physical_changes.iter().any(|(path, _id, kind)| {
            path.to_path_buf() == Path::new("requests").join(request_folder_name(&name))
                && kind == &PathChangeKind::Created
        }));
        assert!(physical_changes.iter().any(|(path, _id, kind)| {
            path.to_path_buf()
                == Path::new("requests")
                    .join(request_folder_name(&name))
                    .join("get.sapic")
                && kind == &PathChangeKind::Created
        }));
        assert!(virtual_changes.iter().any(|(path, _id, kind)| {
            path.to_path_buf() == Path::new("requests").join(&name)
                && kind == &PathChangeKind::Created
        }));

        let specfile_path = collection_path
            .join("requests")
            .join(request_folder_name(&name))
            .join("get.sapic");
        assert!(specfile_path.exists());
    }

    tokio::fs::remove_dir_all(collection_path).await.unwrap();
}

#[tokio::test]
async fn create_entry_request_special_chars_in_path() {
    let (collection_path, collection) = set_up_test_collection().await;

    let request_name = "1";
    let request_dir_name = random_request_dir_name();
    let request_dir_name_list = FOLDERNAME_SPECIAL_CHARS
        .into_iter()
        .map(|s| format!("{}{s}", request_dir_name))
        .collect::<Vec<_>>();

    for dir_name in request_dir_name_list {
        let create_result = collection
            .create_entry(CreateEntryInput {
                destination: Path::new("requests").join(&dir_name).join(&request_name),
                classification: Classification::Request,
                specification: None,
                protocol: None,
                order: None,
                is_dir: false,
            })
            .await;

        // Physical
        // requests\\{encoded_dirname}
        // requests\\{encoded_dirname}\\1.request
        // requests\\{encoded_dirname}\\1.request\\get.sapic

        // Virtual
        // requests\\{dirname}
        // requests\\{dirname}\\1

        let CreateEntryOutput {
            physical_changes,
            virtual_changes,
        } = create_result.unwrap();

        assert!(physical_changes.iter().any(|(path, _id, kind)| {
            path.to_path_buf() == Path::new("requests").join(encode_name(&dir_name))
                && kind == &PathChangeKind::Created
        }));
        assert!(physical_changes.iter().any(|(path, _id, kind)| {
            path.to_path_buf()
                == Path::new("requests")
                    .join(encode_name(&dir_name))
                    .join(request_folder_name(&request_name))
                && kind == &PathChangeKind::Created
        }));
        assert!(physical_changes.iter().any(|(path, _id, kind)| {
            path.to_path_buf()
                == Path::new("requests")
                    .join(encode_name(&dir_name))
                    .join(request_folder_name(&request_name))
                    .join("get.sapic")
                && kind == &PathChangeKind::Created
        }));

        assert!(virtual_changes.iter().any(|(path, _id, kind)| {
            path.to_path_buf() == Path::new("requests").join(&dir_name)
                && kind == &PathChangeKind::Created
        }));
        assert!(virtual_changes.iter().any(|(path, _id, kind)| {
            path.to_path_buf() == Path::new("requests").join(&dir_name).join(&request_name)
                && kind == &PathChangeKind::Created
        }));

        let specfile_path = collection_path
            .join("requests")
            .join(encode_name(&dir_name))
            .join(request_folder_name(&request_name))
            .join("get.sapic");
        assert!(specfile_path.exists());
    }

    tokio::fs::remove_dir_all(collection_path).await.unwrap();
}
