use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use moss_fs::adapters::disk::DiskFileSystem;
use moss_workspace::workspace::Workspace;
use moss_workspace::workspace_manager::WorkspaceManager;

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

pub fn random_workspace_name() -> String {
    format!("Test_{}_Workspace", random_string(10))
}

pub fn random_collection_name() -> String {
    format!("Test_{}_Collection", random_string(10))
}

pub fn random_workspaces_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("data").join(random_string(10))
}

pub fn random_workspace_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("data").join("workspaces").join(random_workspace_name())
}
pub fn random_string(length: usize) -> String {
    use rand::{distr::Alphanumeric, Rng};

    rand::rng()
        .sample_iter(Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}


pub async fn setup_test_workspace_manager() -> (PathBuf, WorkspaceManager) {
    let fs = Arc::new(DiskFileSystem::new());
    let workspaces_path: PathBuf = random_workspaces_path();
    std::fs::create_dir_all(workspaces_path.clone()).unwrap();
    let workspace_manager = WorkspaceManager::new(fs, workspaces_path.clone()).unwrap();
    (workspaces_path, workspace_manager)
}

pub async fn setup_test_workspace() -> (PathBuf, Workspace) {
    let fs = Arc::new(DiskFileSystem::new());
    let workspace_path: PathBuf = random_workspace_path();
    fs::create_dir_all(&workspace_path).unwrap();
    let workspace = Workspace::new(workspace_path.clone(), fs).unwrap();
    (workspace_path, workspace)
}