use moss_collection::models::operations::{CreateEntryInput, CreateEntryOutput};
use moss_collection::models::types::{Classification, PathChangeKind};
use moss_common::api::OperationError;
use moss_testutils::fs_specific::FOLDERNAME_SPECIAL_CHARS;
use moss_testutils::random_name::random_request_name;
use moss_text::sanitized::sanitize;
use serde_json::Value as JsonValue;
use serde_json::json;
use std::fs::read_to_string;
use std::path::Path;

use crate::shared::{create_test_collection, random_dir_name};

mod shared;

#[tokio::test]
async fn create_entry_dir_default_spec() {
    let (collection_path, collection) = create_test_collection().await;

    let dir_name = random_dir_name();

    let create_result = collection
        .create_entry(CreateEntryInput {
            destination: Path::new("requests").join(&dir_name),
            classification: Classification::Request,
            specification: None,
            protocol: None,
            order: None,
            is_dir: true,
        })
        .await;

    let CreateEntryOutput {
        physical_changes,
        virtual_changes,
    } = create_result.unwrap();

    // Physical
    // requests
    // requests\\folder.sapic
    // requests\\{dir_name}
    // requests\\{dir_name}\\folder.sapic

    // Virtual
    // requests
    // requests\\{dir_name}

    // dbg!(&physical_changes);
    // dbg!(&virtual_changes);

    assert_eq!(physical_changes.len(), 4);
    assert!(physical_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == Path::new("requests") && kind == &PathChangeKind::Created
    }));
    assert!(physical_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == Path::new("requests").join("folder.sapic")
            && kind == &PathChangeKind::Created
    }));
    assert!(virtual_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == Path::new("requests").join(&dir_name)
            && kind == &PathChangeKind::Created
    }));
    assert!(physical_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == Path::new("requests").join(&dir_name).join("folder.sapic")
            && kind == &PathChangeKind::Created
    }));

    assert_eq!(virtual_changes.len(), 2);
    assert!(virtual_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == Path::new("requests") && kind == &PathChangeKind::Created
    }));
    assert!(physical_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == Path::new("requests").join(&dir_name)
            && kind == &PathChangeKind::Created
    }));

    let specfile_path = collection_path
        .join("requests")
        .join(&dir_name)
        .join("folder.sapic");
    assert!(specfile_path.exists());

    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}
// TODO: Remake the test after implementing new specification mechanism
// #[tokio::test]
// async fn create_entry_dir_with_spec_content() {
//     let (collection_path, collection) = create_test_collection().await;
//
//     let dir_name = random_dir_name();
//
//     let create_result = collection
//         .create_entry(CreateEntryInput {
//             destination: Path::new("requests").join(&dir_name),
//             classification: Classification::Request,
//             specification: Some(json!(42)),
//             protocol: None,
//             order: None,
//             is_dir: true,
//         })
//         .await;
//
//     let CreateEntryOutput {
//         physical_changes,
//         virtual_changes,
//     } = create_result.unwrap();
//
//     // Physical
//     // requests
//     // requests\\folder.sapic
//     // requests\\{dir_name}
//     // requests\\{dir_name}\\folder.sapic
//
//     // Virtual
//     // requests
//     // requests\\{dir_name}
//
//     assert_eq!(physical_changes.len(), 4);
//     assert!(physical_changes.iter().any(|(path, _id, kind)| {
//         path.to_path_buf() == Path::new("requests") && kind == &PathChangeKind::Created
//     }));
//     assert!(physical_changes.iter().any(|(path, _id, kind)| {
//         path.to_path_buf() == Path::new("requests").join("folder.sapic")
//             && kind == &PathChangeKind::Created
//     }));
//     assert!(virtual_changes.iter().any(|(path, _id, kind)| {
//         path.to_path_buf() == Path::new("requests").join(&dir_name)
//             && kind == &PathChangeKind::Created
//     }));
//     assert!(physical_changes.iter().any(|(path, _id, kind)| {
//         path.to_path_buf() == Path::new("requests").join(&dir_name).join("folder.sapic")
//             && kind == &PathChangeKind::Created
//     }));
//
//     assert_eq!(virtual_changes.len(), 2);
//     assert!(virtual_changes.iter().any(|(path, _id, kind)| {
//         path.to_path_buf() == Path::new("requests") && kind == &PathChangeKind::Created
//     }));
//     assert!(physical_changes.iter().any(|(path, _id, kind)| {
//         path.to_path_buf() == Path::new("requests").join(&dir_name)
//             && kind == &PathChangeKind::Created
//     }));
//
//     let specfile_path = collection_path
//         .join("requests")
//         .join(&dir_name)
//         .join("folder.sapic");
//     assert!(specfile_path.exists());
//
//     let restored_content: JsonValue =
//         serde_json::from_str(&read_to_string(&specfile_path).unwrap()).unwrap();
//     assert_eq!(restored_content, json!(42));
//
//     tokio::fs::remove_dir_all(&collection_path).await.unwrap();
// }

#[tokio::test]
async fn create_entry_dir_already_exists() {
    let (collection_path, collection) = create_test_collection().await;
    let dir_name = random_dir_name();

    let input = CreateEntryInput {
        destination: Path::new("requests").join(&dir_name),
        classification: Classification::Request,
        specification: None,
        protocol: None,
        order: None,
        is_dir: true,
    };

    let _ = collection.create_entry(input.clone()).await.unwrap();

    let create_result = collection.create_entry(input.clone()).await;

    assert!(matches!(
        create_result,
        Err(OperationError::AlreadyExists(..))
    ));

    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}

#[tokio::test]
async fn create_entry_dir_implicitly_created() {
    let (collection_path, collection) = create_test_collection().await;
    let dir_name = random_dir_name();
    let request_name = random_request_name();

    // Create an entry at requests\\{dir_name}\\{request_name}
    // This should implicitly create a dir entry at requests\\{dir_name}

    let _ = collection
        .create_entry(CreateEntryInput {
            destination: Path::new("requests").join(&dir_name).join(&request_name),
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
            destination: Path::new("requests").join(&dir_name),
            classification: Classification::Request,
            specification: None,
            protocol: None,
            order: None,
            is_dir: true,
        })
        .await;

    assert!(matches!(
        create_result,
        Err(OperationError::AlreadyExists(..))
    ));

    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}

#[tokio::test]
async fn create_entry_dir_multiple_same_level() {
    let (collection_path, collection) = create_test_collection().await;
    let dir_name1 = "1";
    let dir_name2 = "2";

    let _ = collection
        .create_entry(CreateEntryInput {
            destination: Path::new("requests").join(&dir_name1),
            classification: Classification::Request,
            specification: None,
            protocol: None,
            order: None,
            is_dir: true,
        })
        .await
        .unwrap();

    let create_result = collection
        .create_entry(CreateEntryInput {
            destination: Path::new("requests").join(&dir_name2),
            classification: Classification::Request,
            specification: None,
            protocol: None,
            order: None,
            is_dir: true,
        })
        .await;

    // Physical
    // requests\\2
    // requests\\2\\folder.sapic

    // Virtual
    // requests\\2

    let CreateEntryOutput {
        physical_changes,
        virtual_changes,
    } = create_result.unwrap();

    assert_eq!(physical_changes.len(), 2);
    assert!(physical_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == Path::new("requests").join(&dir_name2)
            && kind == &PathChangeKind::Created
    }));
    assert!(physical_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == Path::new("requests").join(&dir_name2).join("folder.sapic")
            && kind == &PathChangeKind::Created
    }));

    assert_eq!(virtual_changes.len(), 1);
    assert!(virtual_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == Path::new("requests").join(&dir_name2)
            && kind == &PathChangeKind::Created
    }));

    assert!(
        collection_path
            .join("requests")
            .join("1")
            .join("folder.sapic")
            .exists()
    );
    assert!(
        collection_path
            .join("requests")
            .join("2")
            .join("folder.sapic")
            .exists()
    );
    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}

#[tokio::test]
async fn create_entry_dir_nested() {
    let (collection_path, collection) = create_test_collection().await;
    let outer_name = "outer";
    let inner_name = "inner";

    let create_result = collection
        .create_entry(CreateEntryInput {
            destination: Path::new("requests").join(&outer_name).join(&inner_name),
            classification: Classification::Request,
            specification: None,
            protocol: None,
            order: None,
            is_dir: true,
        })
        .await;

    // Physical
    // requests
    // requests\\folder.sapic
    // requests\\outer
    // requests\\outer\\folder.sapic
    // requests\\outer\\inner
    // requests\\outer\\inner\\folder.sapic

    // Virtual
    // requests
    // requests\\outer
    // requests\\outer\\inner

    let CreateEntryOutput {
        physical_changes,
        virtual_changes,
    } = create_result.unwrap();

    assert_eq!(physical_changes.len(), 6);
    assert!(physical_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == Path::new("requests") && kind == &PathChangeKind::Created
    }));
    assert!(physical_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == Path::new("requests").join("folder.sapic")
            && kind == &PathChangeKind::Created
    }));
    assert!(physical_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == Path::new("requests").join(&outer_name)
            && kind == &PathChangeKind::Created
    }));
    assert!(physical_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == Path::new("requests").join(&outer_name).join("folder.sapic")
            && kind == &PathChangeKind::Created
    }));
    assert!(physical_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == Path::new("requests").join(&outer_name).join(&inner_name)
            && kind == &PathChangeKind::Created
    }));
    assert!(physical_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf()
            == Path::new("requests")
                .join(&outer_name)
                .join(&inner_name)
                .join("folder.sapic")
            && kind == &PathChangeKind::Created
    }));

    assert_eq!(virtual_changes.len(), 3);
    assert!(virtual_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == Path::new("requests") && kind == &PathChangeKind::Created
    }));
    assert!(virtual_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == Path::new("requests").join(&outer_name)
            && kind == &PathChangeKind::Created
    }));
    assert!(virtual_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == Path::new("requests").join(&outer_name).join(&inner_name)
            && kind == &PathChangeKind::Created
    }));

    let specfile_path = collection_path
        .join("requests")
        .join(&outer_name)
        .join(&inner_name)
        .join("folder.sapic");
    assert!(specfile_path.exists());

    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}

#[tokio::test]
async fn create_entry_dir_multiple_different_level() {
    let (collection_path, collection) = create_test_collection().await;
    // requests\\1
    // requests\\group\\2

    let _ = collection
        .create_entry(CreateEntryInput {
            destination: Path::new("requests").join("1"),
            classification: Classification::Request,
            specification: None,
            protocol: None,
            order: None,
            is_dir: true,
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
            is_dir: true,
        })
        .await;

    // Physical
    // requests\\group
    // requests\\group\\folder.sapic
    // requests\\group\\2
    // requests\\group\\2\\folder.sapic

    // Virtual
    // requests\\group
    // requests\\group\\2

    let CreateEntryOutput {
        physical_changes,
        virtual_changes,
    } = create_result.unwrap();

    assert_eq!(physical_changes.len(), 4);
    assert!(physical_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == Path::new("requests").join("group")
            && kind == &PathChangeKind::Created
    }));
    assert!(physical_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == Path::new("requests").join("group").join("folder.sapic")
            && kind == &PathChangeKind::Created
    }));
    assert!(physical_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf() == Path::new("requests").join("group").join("2")
            && kind == &PathChangeKind::Created
    }));
    assert!(physical_changes.iter().any(|(path, _id, kind)| {
        path.to_path_buf()
            == Path::new("requests")
                .join("group")
                .join("2")
                .join("folder.sapic")
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
        .join("1")
        .join("folder.sapic");
    let specfile_path2 = collection_path
        .join("requests")
        .join("group")
        .join("2")
        .join("folder.sapic");
    assert!(specfile_path1.exists());
    assert!(specfile_path2.exists());

    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}

#[tokio::test]
async fn create_entry_dir_special_chars_in_name() {
    let (collection_path, collection) = create_test_collection().await;

    let dir_name = random_dir_name();
    let dir_name_list = FOLDERNAME_SPECIAL_CHARS
        .into_iter()
        .map(|s| format!("{}{s}", dir_name))
        .collect::<Vec<_>>();

    for name in dir_name_list {
        let create_result = collection
            .create_entry(CreateEntryInput {
                destination: Path::new("requests").join(&name),
                classification: Classification::Request,
                specification: None,
                protocol: None,
                order: None,
                is_dir: true,
            })
            .await;

        // Physical
        // requests\\{encoded_name}
        // requests\\{encoded_name}\\folder.sapic

        // Virtual
        // requests\\name

        let CreateEntryOutput {
            physical_changes,
            virtual_changes,
        } = create_result.unwrap();

        assert!(physical_changes.iter().any(|(path, _id, kind)| {
            path.to_path_buf() == Path::new("requests").join(sanitize(&name))
                && kind == &PathChangeKind::Created
        }));
        assert!(physical_changes.iter().any(|(path, _id, kind)| {
            path.to_path_buf()
                == Path::new("requests")
                    .join(sanitize(&name))
                    .join("folder.sapic")
                && kind == &PathChangeKind::Created
        }));

        assert!(virtual_changes.iter().any(|(path, _id, kind)| {
            path.to_path_buf() == Path::new("requests").join(&name)
                && kind == &PathChangeKind::Created
        }));
        let specfile_path = collection_path
            .join("requests")
            .join(sanitize(&name))
            .join("folder.sapic");

        assert!(specfile_path.exists());
    }
}

#[tokio::test]
async fn create_entry_dir_special_chars_in_path() {
    let (collection_path, collection) = create_test_collection().await;

    let dir_name = random_dir_name();
    let dir_name_list = FOLDERNAME_SPECIAL_CHARS
        .into_iter()
        .map(|s| format!("{}{s}", dir_name))
        .collect::<Vec<_>>();

    for name in dir_name_list {
        let create_result = collection
            .create_entry(CreateEntryInput {
                destination: Path::new("requests").join(&name).join("group"),
                classification: Classification::Request,
                specification: None,
                protocol: None,
                order: None,
                is_dir: true,
            })
            .await;

        // Physical
        // requests\\{encoded_name}
        // requests\\{encoded_name}\\folder.sapic
        // requests\\{encoded_name}\\group
        // requests\\{encoded_name}\\group\\folder.sapic

        // Virtual
        // requests\\name
        // requests\\name\\group

        let CreateEntryOutput {
            physical_changes,
            virtual_changes,
        } = create_result.unwrap();

        assert!(physical_changes.iter().any(|(path, _id, kind)| {
            path.to_path_buf() == Path::new("requests").join(sanitize(&name))
                && kind == &PathChangeKind::Created
        }));
        assert!(physical_changes.iter().any(|(path, _id, kind)| {
            path.to_path_buf()
                == Path::new("requests")
                    .join(sanitize(&name))
                    .join("folder.sapic")
                && kind == &PathChangeKind::Created
        }));
        assert!(physical_changes.iter().any(|(path, _id, kind)| {
            path.to_path_buf() == Path::new("requests").join(sanitize(&name)).join("group")
                && kind == &PathChangeKind::Created
        }));
        assert!(physical_changes.iter().any(|(path, _id, kind)| {
            path.to_path_buf()
                == Path::new("requests")
                    .join(sanitize(&name))
                    .join("group")
                    .join("folder.sapic")
                && kind == &PathChangeKind::Created
        }));

        assert!(virtual_changes.iter().any(|(path, _id, kind)| {
            path.to_path_buf() == Path::new("requests").join(&name)
                && kind == &PathChangeKind::Created
        }));
        assert!(virtual_changes.iter().any(|(path, _id, kind)| {
            path.to_path_buf() == Path::new("requests").join(&name).join("group")
                && kind == &PathChangeKind::Created
        }));

        let specfile_path = collection_path
            .join("requests")
            .join(sanitize(&name))
            .join("group")
            .join("folder.sapic");

        assert!(specfile_path.exists());
    }
}

#[tokio::test]
async fn create_entry_dir_same_name_as_another_entry() {
    // Create two entries with the same name, one normal and one dir
    // This will result in two identical virtual paths, so it should raise an error

    let (collection_path, collection) = create_test_collection().await;
    let destination = Path::new("requests").join("identical");

    let _ = collection
        .create_entry(CreateEntryInput {
            destination: destination.clone(),
            classification: Classification::Request,
            specification: None,
            protocol: None,
            order: None,
            is_dir: false,
        })
        .await;

    let create_result = collection
        .create_entry(CreateEntryInput {
            destination: destination.clone(),
            classification: Classification::Request,
            specification: None,
            protocol: None,
            order: None,
            is_dir: true,
        })
        .await;

    assert!(matches!(
        create_result,
        Err(OperationError::AlreadyExists(..))
    ));

    // Check that physical entry is not created incorrectly
    assert!(!collection_path.join("requests").join("identical").exists());

    tokio::fs::remove_dir_all(&collection_path).await.unwrap();
}
