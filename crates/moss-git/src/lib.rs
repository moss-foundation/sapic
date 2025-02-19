use git2::build::RepoBuilder;
use git2::{RemoteCallbacks, Repository};
use std::fs::remove_dir_all;
use std::path::Path;

pub mod auth;
mod cred;
mod repo;

pub fn clone_flow(url: &str, path: &Path, callback: RemoteCallbacks) -> Result<Repository, String> {
    // remove_dir_all(path);

    let mut fo = git2::FetchOptions::new();
    fo.remote_callbacks(callback);

    let mut builder = RepoBuilder::new();
    builder.fetch_options(fo);

    match builder.clone(url, path) {
        Ok(repo) => Ok(repo),
        Err(e) => Err(format!("failed to clone: {}", e)),
    }
}

// GIT ui functionality
// Other VCS?
// Operations: Clone, Commit, (Conflict/Merge), (Branching), (PR)...
// Secrets: tauri plugin stronghold
