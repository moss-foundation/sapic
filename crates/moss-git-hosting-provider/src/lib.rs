pub mod github;
pub mod gitlab;
mod utils;

use async_trait::async_trait;

#[derive(Debug, Clone)]
pub enum GitProviderKind {
    GitHub,
    GitLab,
}

#[async_trait]
pub trait GitAuthAdapter {
    type PkceToken;
    type PatToken;

    async fn auth_with_pkce(&self) -> joinerror::Result<Self::PkceToken>;
    async fn auth_with_pat(&self) -> joinerror::Result<Self::PatToken>;
}

#[cfg(any(test, feature = "integration-tests"))]
pub mod envvar_keys {
    /// Environment variable keys
    pub const GITHUB_CLIENT_ID: &'static str = "GITHUB_CLIENT_ID";
    pub const GITHUB_CLIENT_SECRET: &'static str = "GITHUB_CLIENT_SECRET";
    pub const GITHUB_ACCESS_TOKEN: &'static str = "GITHUB_ACCESS_TOKEN";
    pub const GITLAB_CLIENT_ID: &'static str = "GITLAB_CLIENT_ID";
    pub const GITLAB_CLIENT_SECRET: &'static str = "GITLAB_CLIENT_SECRET";
    pub const GITLAB_REFRESH_TOKEN: &'static str = "GITLAB_REFRESH_TOKEN";
}

// FIXME:
// #[cfg(test)]
// pub mod tests {
//     use git2::RemoteCallbacks;
//     use git2::{IndexAddOption, RemoteCallbacks, Signature, Status};
//     use nanoid::nanoid;
//     use std::{path::PathBuf, sync::Arc};

//     // use crate::{GitAuthAgent, repo::RepoHandle};

//     // All the tests here will be local operations, no need for authentication
//     struct TestGitAuthAgent;

//     impl GitAuthAgent for TestGitAuthAgent {
//         fn generate_callback<'a>(
//             &'a self,
//             _cb: &mut RemoteCallbacks<'a>,
//         ) -> Result<(), anyhow::Error> {
//             Ok(())
//         }
//     }

//     fn create_test_repo() -> (RepoHandle, PathBuf) {
//         let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
//             .join("tests")
//             .join("data")
//             .join(nanoid!(10));
//         std::fs::create_dir_all(&path).unwrap();

//         let auth_agent = Arc::new(TestGitAuthAgent {});

//         (RepoHandle::init(&path, auth_agent).unwrap(), path)
//     }

//     fn status_label(status: Status) -> Vec<&'static str> {
//         let mut v = Vec::new();
//         if status.is_empty() {
//             v.push("clean");
//             return v;
//         }
//         if status.contains(Status::CURRENT) {
//             v.push("CURRENT");
//         }
//         if status.contains(Status::INDEX_NEW) {
//             v.push("INDEX_NEW (staged new)");
//         }
//         if status.contains(Status::INDEX_MODIFIED) {
//             v.push("INDEX_MODIFIED (staged modified)");
//         }
//         if status.contains(Status::INDEX_DELETED) {
//             v.push("INDEX_DELETED (staged deleted)");
//         }
//         if status.contains(Status::INDEX_RENAMED) {
//             v.push("INDEX_RENAMED");
//         }
//         if status.contains(Status::INDEX_TYPECHANGE) {
//             v.push("INDEX_TYPECHANGE");
//         }

//         if status.contains(Status::WT_NEW) {
//             v.push("WT_NEW (untracked)");
//         }
//         if status.contains(Status::WT_MODIFIED) {
//             v.push("WT_MODIFIED (modified, unstaged)");
//         }
//         if status.contains(Status::WT_DELETED) {
//             v.push("WT_DELETED (deleted, unstaged)");
//         }
//         if status.contains(Status::WT_RENAMED) {
//             v.push("WT_RENAMED");
//         }
//         if status.contains(Status::WT_TYPECHANGE) {
//             v.push("WT_TYPECHANGE");
//         }
//         if status.contains(Status::WT_UNREADABLE) {
//             v.push("WT_UNREADABLE");
//         }

//         if status.contains(Status::IGNORED) {
//             v.push("IGNORED");
//         }
//         if status.contains(Status::CONFLICTED) {
//             v.push("CONFLICTED");
//         }
//         v
//     }

//     // Only print the difference between Index and Workdir
//     fn print_statuses(handle: &RepoHandle) {
//         let mut status_options = git2::StatusOptions::new();
//         status_options
//             .include_untracked(true)
//             .recurse_untracked_dirs(true)
//             .rename_threshold(50)
//             .renames_index_to_workdir(true);
//         let statuses = handle.repo().statuses(Some(&mut status_options)).unwrap();
//         for status in statuses.iter() {
//             println!(
//                 "{}: {:?}",
//                 status.path().unwrap(),
//                 status_label(status.status())
//             );
//         }
//     }

//     #[test]
//     pub fn test_statuses() {
//         let (repo, path) = create_test_repo();
//         std::fs::write(path.join("test.txt"), "Content".as_bytes()).unwrap();
//         print_statuses(&repo);

//         std::fs::write(path.join("test.txt"), "ContentNew".as_bytes()).unwrap();
//         print_statuses(&repo);
//     }

//     #[test]
//     pub fn test_delete_committed() {
//         let (repo, path) = create_test_repo();
//         let file_path = path.join("test.txt");
//         std::fs::write(&file_path, "Content".as_bytes()).unwrap();
//         print_statuses(&repo);

//         repo.add(&["test.txt"], IndexAddOption::DEFAULT).unwrap();
//         print_statuses(&repo);
//         let sig = Signature::now("test", "test@test.com").unwrap();
//         repo.commit("Test", sig).unwrap();

//         std::fs::remove_file(&file_path).unwrap();
//         print_statuses(&repo);
//     }

//     #[test]
//     pub fn test_rename() {
//         let (repo, path) = create_test_repo();
//         let file_path = path.join("test.txt");
//         std::fs::write(&file_path, "Content".as_bytes()).unwrap();
//         repo.add(&["test.txt"], IndexAddOption::DEFAULT).unwrap();
//         let sig = Signature::now("test", "test@test.com").unwrap();
//         repo.commit("Test", sig).unwrap();

//         std::fs::rename(&file_path, path.join("new_text.txt")).unwrap();
//         print_statuses(&repo);
//     }
// }
