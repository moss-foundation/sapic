use moss_collection::{
    collection::{Collection, CreateParams},
    models::primitives::WorktreeDiff,
};
use moss_fs::RealFileSystem;
use moss_testutils::random_name::{random_collection_name, random_string};
use moss_text::sanitized::sanitized_name::SanitizedName;
use std::{
    path::{Path, PathBuf},
    sync::{Arc, atomic::AtomicUsize},
};
use uuid::Uuid;

#[allow(dead_code)]
pub fn random_dir_name() -> String {
    format!("Test_{}_Dir", random_string(10))
}

fn random_collection_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join(Uuid::new_v4().to_string())
}

pub async fn setup_test_collection() -> (PathBuf, Collection) {
    let fs = Arc::new(RealFileSystem::new());
    let internal_abs_path = random_collection_path();

    std::fs::create_dir_all(internal_abs_path.clone()).unwrap();

    let next_entry_id = Arc::new(AtomicUsize::new(0));
    let collection = Collection::create(
        fs,
        CreateParams {
            name: Some(random_collection_name()),
            external_abs_path: None,
            internal_abs_path: &internal_abs_path,
        },
    )
    .await
    .unwrap();

    (internal_abs_path, collection)
}

// #[allow(dead_code)]
// /// Find the entry id by path
// pub fn find_id_by_path(changes_diff_set: &WorktreeDiff, path: &Path) -> Option<EntryId> {
//     changes_diff_set
//         .iter()
//         .find(|change| entry_path.as_ref() == path)
//         .map(|item| item.1.clone())
// }
