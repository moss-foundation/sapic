use joinerror::OptionExt;
use moss_fs::{FileSystem, RemoveOptions};
use moss_git::{
    models::types::BranchInfo,
    repo::{FileStatus, RepoHandle},
};
use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex},
};

pub struct GitService {
    /// All operations over the `RepoHandle` will be wrapped inside a `spawn_blocking` closure
    /// to avoid blocking the main thread
    pub(super) repo_handle: Arc<Mutex<Option<RepoHandle>>>,
}

impl GitService {
    // Sometimes objects might be set as readonly, preventing them from being deleted
    // we will need to recursively set all files in .git/objects as writable
    pub async fn dispose(&self, fs: Arc<dyn FileSystem>) -> joinerror::Result<()> {
        let repo_handle = self.repo_handle.lock()?.take();
        if repo_handle.is_none() {
            return Ok(());
        }
        let repo_handle = repo_handle.unwrap();
        let path = repo_handle.path().to_path_buf();
        drop(repo_handle);

        let mut folders = vec![path.join("objects")];

        while let Some(folder) = folders.pop() {
            let mut read_dir = fs.read_dir(&folder).await?;
            while let Some(entry) = read_dir.next_entry().await? {
                if entry.file_type().await?.is_dir() {
                    folders.push(entry.path());
                }
                let mut perms = entry.metadata().await?.permissions();
                perms.set_readonly(false);
                tokio::fs::set_permissions(&entry.path(), perms).await?;
            }
        }

        fs.remove_dir(
            &path,
            RemoveOptions {
                recursive: true,
                ignore_if_not_exists: true,
            },
        )
        .await?;

        Ok(())
    }

    pub async fn has_repo(&self) -> joinerror::Result<bool> {
        let repo_handle_clone = self.repo_handle.clone();
        let join = tokio::task::spawn_blocking(move || {
            let repo_handle_lock = repo_handle_clone.lock()?;
            return Ok(repo_handle_lock.is_some());
        })
        .await?;

        match join {
            Ok(result) => Ok(result),
            Err(e) => Err(e),
        }
    }

    pub async fn get_file_statuses(&self) -> joinerror::Result<HashMap<PathBuf, FileStatus>> {
        let repo_handle_clone = self.repo_handle.clone();
        let join = tokio::task::spawn_blocking(move || {
            let repo_handle_lock = repo_handle_clone.lock()?;
            let repo_handle_ref = repo_handle_lock
                .as_ref()
                .ok_or_join_err::<()>("no repo handle")?;
            repo_handle_ref.get_file_statuses()
        })
        .await?;

        match join {
            Ok(result) => Ok(result),
            Err(e) => Err(e),
        }
    }

    // FIXME: Maybe it doesn't make sense to have a separate method just to get the current branch name
    // Although we don't need any comparison with remote branch just for getting the name
    pub async fn get_current_branch(&self) -> joinerror::Result<String> {
        let repo_handle_clone = self.repo_handle.clone();
        let join = tokio::task::spawn_blocking(move || {
            let repo_handle_lock = repo_handle_clone.lock()?;
            let repo_handle_ref = repo_handle_lock
                .as_ref()
                .ok_or_join_err::<()>("no repo handle")?;
            let current_branch = repo_handle_ref.current_branch()?;

            Ok(current_branch)
        })
        .await?;

        match join {
            Ok(result) => Ok(result),
            Err(e) => Err(e),
        }
    }

    pub async fn get_current_branch_info(&self) -> joinerror::Result<BranchInfo> {
        let repo_handle_clone = self.repo_handle.clone();

        let join = tokio::task::spawn_blocking(move || {
            let repo_handle_lock = repo_handle_clone.lock()?;
            let repo_handle_ref = repo_handle_lock
                .as_ref()
                .ok_or_join_err::<()>("no repo handle")?;
            // TODO: Support custom origin name? We assume it's `origin` now, which we use when we create a repo

            let current_branch = repo_handle_ref.current_branch()?;

            // git fetch
            repo_handle_ref.fetch(Some("origin"))?;

            // Compare local with remote state
            let (ahead, behind) = repo_handle_ref.compare_with_remote_branch(&current_branch)?;

            Ok(BranchInfo {
                name: current_branch,
                ahead,
                behind,
            })
        })
        .await?;

        match join {
            Ok(result) => Ok(result),
            Err(e) => Err(e),
        }
    }
}

impl GitService {
    pub fn new(repo_handle: Option<RepoHandle>) -> Self {
        Self {
            repo_handle: Arc::new(Mutex::new(repo_handle)),
        }
    }
}
