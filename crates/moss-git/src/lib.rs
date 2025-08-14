// TODO: Implement other git functionalities
// e.g. branching, merging, conflict resolution
use anyhow::Result;
use git2::RemoteCallbacks;

pub mod repo;
pub mod url;

pub trait GitAuthAgent {
    fn generate_callback<'a>(&'a self, cb: &mut RemoteCallbacks<'a>) -> Result<()>;
}

pub mod constants {
    pub const DEFAULT_REMOTE_NAME: &str = "origin";
}

#[cfg(test)]
pub mod tests {
    use git2::{Status, Statuses};
    use nanoid::nanoid;
    use std::path::PathBuf;

    fn create_test_repo() -> (git2::Repository, PathBuf) {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("data")
            .join(nanoid!(10));
        std::fs::create_dir_all(&path).unwrap();
        (git2::Repository::init(&path).unwrap(), path)
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
    fn print_statuses(repo: &git2::Repository) {
        let mut status_options = git2::StatusOptions::new();
        status_options.include_untracked(true);
        let statuses = repo.statuses(Some(&mut status_options)).unwrap();
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
}
