// TODO: Implement other git functionalities
// e.g. branching, merging, conflict resolution
use anyhow::Result;
use async_trait::async_trait;
use git2::RemoteCallbacks;
use oauth2::{ClientId, ClientSecret};

pub mod models;
pub mod repo;
pub mod repository;
pub mod url;

#[async_trait]
pub trait GitSignInAdapter {
    type PkceToken;
    type PatToken;

    async fn sign_in_with_pkce(
        &self,
        client_id: ClientId,
        client_secret: ClientSecret,
        host: &str,
    ) -> anyhow::Result<Self::PkceToken>;
    async fn sign_in_with_pat(&self) -> joinerror::Result<Self::PatToken>;
}

pub trait GitAuthAgent {
    fn generate_callback<'a>(&'a self, cb: &mut RemoteCallbacks<'a>) -> Result<()>;
}

pub mod constants {
    pub const DEFAULT_REMOTE_NAME: &str = "origin";
}

#[cfg(test)]
pub mod tests {
    use git2::{IndexAddOption, RemoteCallbacks, Signature, Status};
    use nanoid::nanoid;
    use std::{path::PathBuf, sync::Arc};

    use crate::{GitAuthAgent, repo::RepoHandle};

    // All the tests here will be local operations, no need for authentication
    struct TestGitAuthAgent;

    impl GitAuthAgent for TestGitAuthAgent {
        fn generate_callback<'a>(
            &'a self,
            _cb: &mut RemoteCallbacks<'a>,
        ) -> Result<(), anyhow::Error> {
            Ok(())
        }
    }

    fn create_test_repo() -> (RepoHandle, PathBuf) {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("data")
            .join(nanoid!(10));
        std::fs::create_dir_all(&path).unwrap();

        let auth_agent = Arc::new(TestGitAuthAgent {});

        (RepoHandle::init(&path, auth_agent).unwrap(), path)
    }

    fn status_label(status: Status) -> Vec<&'static str> {
        let mut v = Vec::new();
        if status.is_empty() {
            v.push("clean");
            return v;
        }
        if status.contains(Status::CURRENT) {
            v.push("CURRENT");
        }
        if status.contains(Status::INDEX_NEW) {
            v.push("INDEX_NEW (staged new)");
        }
        if status.contains(Status::INDEX_MODIFIED) {
            v.push("INDEX_MODIFIED (staged modified)");
        }
        if status.contains(Status::INDEX_DELETED) {
            v.push("INDEX_DELETED (staged deleted)");
        }
        if status.contains(Status::INDEX_RENAMED) {
            v.push("INDEX_RENAMED");
        }
        if status.contains(Status::INDEX_TYPECHANGE) {
            v.push("INDEX_TYPECHANGE");
        }

        if status.contains(Status::WT_NEW) {
            v.push("WT_NEW (untracked)");
        }
        if status.contains(Status::WT_MODIFIED) {
            v.push("WT_MODIFIED (modified, unstaged)");
        }
        if status.contains(Status::WT_DELETED) {
            v.push("WT_DELETED (deleted, unstaged)");
        }
        if status.contains(Status::WT_RENAMED) {
            v.push("WT_RENAMED");
        }
        if status.contains(Status::WT_TYPECHANGE) {
            v.push("WT_TYPECHANGE");
        }
        if status.contains(Status::WT_UNREADABLE) {
            v.push("WT_UNREADABLE");
        }

        if status.contains(Status::IGNORED) {
            v.push("IGNORED");
        }
        if status.contains(Status::CONFLICTED) {
            v.push("CONFLICTED");
        }
        v
    }

    // Only print the difference between Index and Workdir
    fn print_statuses(handle: &RepoHandle) {
        let mut status_options = git2::StatusOptions::new();
        status_options
            .include_untracked(true)
            .recurse_untracked_dirs(true)
            .rename_threshold(50)
            .renames_index_to_workdir(true);
        let statuses = handle.repo().statuses(Some(&mut status_options)).unwrap();
        for status in statuses.iter() {
            println!(
                "{}: {:?}",
                status.path().unwrap(),
                status_label(status.status())
            );
        }
    }

    #[test]
    pub fn test_statuses() {
        let (repo, path) = create_test_repo();
        std::fs::write(path.join("test.txt"), "Content".as_bytes()).unwrap();
        print_statuses(&repo);

        std::fs::write(path.join("test.txt"), "ContentNew".as_bytes()).unwrap();
        print_statuses(&repo);
    }

    #[test]
    pub fn test_delete_committed() {
        let (repo, path) = create_test_repo();
        let file_path = path.join("test.txt");
        std::fs::write(&file_path, "Content".as_bytes()).unwrap();
        print_statuses(&repo);

        repo.add(&["test.txt"], IndexAddOption::DEFAULT).unwrap();
        print_statuses(&repo);
        let sig = Signature::now("test", "test@test.com").unwrap();
        repo.commit("Test", sig).unwrap();

        std::fs::remove_file(&file_path).unwrap();
        print_statuses(&repo);
    }

    #[test]
    pub fn test_rename() {
        let (repo, path) = create_test_repo();
        let file_path = path.join("test.txt");
        std::fs::write(&file_path, "Content".as_bytes()).unwrap();
        repo.add(&["test.txt"], IndexAddOption::DEFAULT).unwrap();
        let sig = Signature::now("test", "test@test.com").unwrap();
        repo.commit("Test", sig).unwrap();

        std::fs::rename(&file_path, path.join("new_text.txt")).unwrap();
        print_statuses(&repo);
    }
}
