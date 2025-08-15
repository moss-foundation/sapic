use moss_applib::ServiceMarker;
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
    /// Since operations over RepoHandle must be done in a synchronous closure wrapped by a
    /// `tokio::task::spawn_blocking`
    /// This mutex must be a synchronous one and should not be acquired in an async block
    /// It should always be required in a `spawn_blocking` block to avoid deadlock
    ///
    pub(super) repo_handle: Arc<Mutex<Option<RepoHandle>>>,
}

impl ServiceMarker for GitService {}

impl GitService {
    pub async fn get_file_statuses(&self) -> joinerror::Result<HashMap<PathBuf, FileStatus>> {
        let repo_handle_clone = self.repo_handle.clone();
        let join = tokio::task::spawn_blocking(move || {
            let repo_handle_lock = repo_handle_clone.lock()?;
            let repo_handle_ref = repo_handle_lock.as_ref();
            if repo_handle_ref.is_none() {
                return Ok(HashMap::new());
            }
            let repo_handle_ref = repo_handle_ref.unwrap();
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
    pub async fn get_current_branch(&self) -> joinerror::Result<Option<String>> {
        let repo_handle_clone = self.repo_handle.clone();
        let join = tokio::task::spawn_blocking(move || {
            let repo_handle_lock = repo_handle_clone.lock()?;
            let repo_handle_ref = repo_handle_lock.as_ref();
            if repo_handle_ref.is_none() {
                return Ok(None);
            }
            let repo_handle_ref = repo_handle_ref.unwrap();
            // TODO: Support custom origin name? We assume it's `origin` now, which we use when we create a repo

            let current_branch = repo_handle_ref.current_branch()?;

            Ok(Some(current_branch))
        })
        .await?;

        match join {
            Ok(result) => Ok(result),
            Err(e) => Err(e),
        }
    }

    pub async fn get_current_branch_info(&self) -> joinerror::Result<Option<BranchInfo>> {
        let repo_handle_clone = self.repo_handle.clone();

        let join = tokio::task::spawn_blocking(move || {
            let repo_handle_lock = repo_handle_clone.lock()?;
            let repo_handle_ref = repo_handle_lock.as_ref();
            if repo_handle_ref.is_none() {
                return Ok(None);
            }
            let repo_handle_ref = repo_handle_ref.unwrap();
            // TODO: Support custom origin name? We assume it's `origin` now, which we use when we create a repo

            let current_branch = repo_handle_ref.current_branch()?;

            // git fetch
            repo_handle_ref.fetch(Some("origin"))?;

            // Compare local with remote state
            let (ahead, behind) = repo_handle_ref.compare_with_remote_branch(&current_branch)?;

            Ok(Some(BranchInfo {
                name: current_branch,
                ahead,
                behind,
            }))
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
