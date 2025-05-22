use moss_collection::collection::Collection;
use moss_collection::models::primitives::{ChangesDiffSet, EntryId};
use moss_fs::RealFileSystem;
use moss_testutils::random_name::{random_collection_name, random_string};
use moss_text::sanitized::sanitized_name::SanitizedName;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;

pub fn random_dir_name() -> String {
    format!("Test_{}_Dir", random_string(10))
}

fn random_collection_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join(random_collection_name())
}

pub async fn set_up_test_collection() -> (PathBuf, Collection) {
    let fs = Arc::new(RealFileSystem::new());
    let collection_path = random_collection_path();

    std::fs::create_dir_all(collection_path.clone()).unwrap();

    // Create collection/requests to prevent indexation error
    // std::fs::create_dir_all(collection_path.join("requests")).unwrap();

    let next_entry_id = Arc::new(AtomicUsize::new(0));
    let collection = Collection::new(collection_path.clone(), fs, next_entry_id).unwrap();

    (collection_path, collection)
}
/// Generate the encoded request folder name
pub fn request_folder_name(request_name: &str) -> String {
    let sanitized_name = SanitizedName::new(request_name);
    format!("{}.request", &sanitized_name)
}

/// Find the entry id by path
pub fn find_id_by_path(changes_diff_set: &ChangesDiffSet, path: &Path) -> Option<EntryId> {
    changes_diff_set
        .iter()
        .find(|(entry_path, _id, _kind)| entry_path.as_ref() == path)
        .map(|item| item.1.clone())
}
