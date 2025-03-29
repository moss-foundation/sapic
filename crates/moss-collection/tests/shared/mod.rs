use std::path::PathBuf;
use std::sync::Arc;
use moss_collection::collection::Collection;
use moss_fs::RealFileSystem;
use moss_fs::utils::encode_directory_name;

pub const SPECIAL_CHARS: [&str; 10] =   [
    // Test with various special characters
    ".",  // dot
    "/",  // path separator
    "\\", // backslash
    ":",  // colon
    "*",  // wildcard
    "?",  // question mark
    "\"", // quotes
    "<",  // angle brackets
    ">",  // angle brackets
    "|",  // pipe
];
pub fn random_request_name() -> String { format!("Test_{}_Request", random_string(10))}

pub fn random_collection_name() -> String {
    format!("Test_{}_Collection", random_string(10))
}

pub fn random_collection_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("data").join(random_collection_name())
}

pub fn random_string(length: usize) -> String {
    use rand::{distr::Alphanumeric, Rng};

    rand::rng()
        .sample_iter(Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

pub async fn set_up_test_collection() -> (PathBuf, Collection) {
    let fs = Arc::new(RealFileSystem::new());
    let collection_path = random_collection_path();
    std::fs::create_dir_all(collection_path.clone()).unwrap();
    let collection = Collection::new(collection_path.clone(), fs).unwrap();
    (collection_path, collection)
}


pub fn request_folder_name(request_name: &str) -> String {
    format!("{}.request", encode_directory_name(request_name))
}

pub fn request_relative_path(name: &str, relative: Option<&str>) -> PathBuf{
    if relative.is_some() {
        PathBuf::from("requests").join(relative.unwrap()).join(request_folder_name(name))
    } else {
        PathBuf::from("requests").join(request_folder_name(name))
    }
}