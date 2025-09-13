use derive_more::Deref;
use git2::{
    BranchType, IndexAddOption, IntoCString, PushOptions, RemoteCallbacks, Signature, StashFlags,
    Status, StatusOptions,
    build::{CheckoutBuilder, RepoBuilder},
};
use joinerror::{Error, OptionExt, ResultExt};
use moss_logging::session;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::{constants, errors::DirtyWorktree, models::primitives::FileStatus};

#[derive(Deref)]
pub struct Repository {
    #[deref]
    inner: git2::Repository,
}

// SAFETY: git2::Repository is actually thread-safe when properly used.
// The raw pointers are managed internally by libgit2, and we don't expose
// them directly. All operations go through libgit2's safe API.
unsafe impl Send for Repository {}
unsafe impl Sync for Repository {}

impl Repository {
    pub fn init(path: &Path) -> joinerror::Result<Self> {
        Ok(Repository {
            inner: git2::Repository::init(path)?,
        })
    }

    pub fn open(path: &Path) -> joinerror::Result<Self> {
        Ok(Repository {
            inner: git2::Repository::open(path)?,
        })
    }

    pub fn clone<'a>(url: &str, into: &Path, cb: RemoteCallbacks<'a>) -> joinerror::Result<Self> {
        let mut opts = git2::FetchOptions::new();
        opts.remote_callbacks(cb);

        let mut builder = RepoBuilder::new();
        builder.fetch_options(opts);

        Ok(Repository {
            inner: builder.clone(url, into)?,
        })
    }

    pub fn add_remote(&self, remote_name: Option<&str>, url: &str) -> joinerror::Result<()> {
        self.inner
            .remote(remote_name.unwrap_or(constants::DEFAULT_REMOTE_NAME), url)?;

        Ok(())
    }

    pub fn list_remotes(&self) -> joinerror::Result<HashMap<String, String>> {
        let remote_names = self.inner.remotes()?;
        let mut result = HashMap::with_capacity(remote_names.len());

        for remote_name in remote_names.iter() {
            if remote_name.is_none() {
                continue;
            }

            let remote_name = remote_name.unwrap();
            let remote = self.inner.find_remote(remote_name)?;
            let url = remote.url();
            if url.is_none() {
                continue;
            }

            result.insert(remote_name.to_string(), url.unwrap().to_string());
        }

        Ok(result)
    }

    pub fn list_branches(&self, branch_type: Option<BranchType>) -> joinerror::Result<Vec<String>> {
        let branches = self.inner.branches(branch_type)?;

        Ok(branches
            .into_iter()
            .filter_map(|b| {
                if let Ok((branch, _)) = b {
                    branch.name().ok().flatten().map(|name| name.to_string())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>())
    }

    pub fn graph_ahead_behind(&self, branch_name: &str) -> joinerror::Result<(usize, usize)> {
        let local = self.inner.find_branch(branch_name, BranchType::Local)?;
        let upstream = local.upstream()?;
        let local_commit = local.get().peel_to_commit()?;
        let upstream_commit = upstream.get().peel_to_commit()?;

        Ok(self
            .inner
            .graph_ahead_behind(local_commit.id(), upstream_commit.id())?)
    }

    pub fn checkout_branch(
        &self,
        remote_name: Option<&str>,
        branch_name: &str,
        create_from_remote: bool,
    ) -> joinerror::Result<()> {
        // Try to find an existing local branch
        match self.inner.find_branch(branch_name, BranchType::Local) {
            // A local branch is found
            Ok(branch) => {
                // Convert branch wrapper into its underlying reference, get full refname.
                let reference = branch.into_reference();
                let refname = reference
                    .name()
                    .ok_or_join_err::<()>("invalid branch refname")?;
                // Set HEAD to that branch reference (e.g. "refs/heads/<branch>")
                self.inner.set_head(refname)?;
            }
            // A local branch is not found
            Err(_) => {
                if !create_from_remote {
                    return Err(joinerror::Error::new::<()>(format!(
                        "a local branch named `{}` does not exist",
                        branch_name
                    )));
                }

                // Create a local branch based on the remote branch
                let remote = remote_name.unwrap_or(constants::DEFAULT_REMOTE_NAME);
                let remote_refname = format!("refs/remotes/{}/{}", remote, branch_name);
                let remote_ref = self
                    .inner
                    .find_reference(&remote_refname)
                    .join_err::<()>("failed to find remote branch reference")?;

                let target_commit = remote_ref.peel_to_commit()?;
                let mut local_branch = self.inner.branch(branch_name, &target_commit, false)?;

                let upstream_name = format!("{}/{}", remote, branch_name);
                local_branch.set_upstream(Some(&upstream_name))?;

                // Set HEAD to the new local branch
                let local_refname = format!("refs/heads/{}", branch_name);
                self.inner.set_head(&local_refname)?;
            }
        }

        let mut builder = CheckoutBuilder::new();
        Ok(self.inner.checkout_head(Some(&mut builder))?)
    }

    pub fn rename_branch(
        &self,
        old_name: &str,
        new_name: &str,
        force: bool,
    ) -> joinerror::Result<()> {
        let mut branch = self.inner.find_branch(old_name, BranchType::Local)?;
        branch.rename(new_name, force)?;
        Ok(())
    }

    pub fn add_all(
        &self,
        paths: impl IntoIterator<Item = impl IntoCString>,
        opts: IndexAddOption,
    ) -> joinerror::Result<()> {
        let mut index = self.inner.index()?;
        index.add_all(paths, opts, None)?;
        index.write()?;
        Ok(())
    }

    pub fn remove_all(
        &self,
        paths: impl IntoIterator<Item = impl IntoCString>,
    ) -> joinerror::Result<()> {
        let mut index = self.inner.index()?;
        index.remove_all(paths, None)?;
        index.write()?;
        Ok(())
    }

    // We will only update the index immediately before a commit, thus there should only be
    // three types of file statuses (excluding conflict and ignored)
    // that we need to handle:
    // WT_NEW: new file yet to be tracked (never committed)
    // WT_MODIFIED: tracked file that's modified
    // WT_DELETED: tracked file that's deleted

    pub fn stage_paths(
        &self,
        paths: impl IntoIterator<Item = impl AsRef<Path>>,
    ) -> joinerror::Result<()> {
        let workdir = self.inner.workdir().ok_or_join_err::<()>("no workdir")?;
        let mut index = self.inner.index()?;

        for path in paths {
            let abs_path = workdir.join(path.as_ref());

            if abs_path.exists() {
                index.add_path(path.as_ref())?;
            } else {
                index.remove_path(path.as_ref())?;
            }
        }
        index.write()?;

        Ok(())
    }
    pub fn discard_changes(
        &self,
        paths: impl IntoIterator<Item = impl AsRef<Path>>,
    ) -> joinerror::Result<()> {
        // Reset changes to tracked files to the INDEX
        let workdir = self.inner.workdir().ok_or_join_err::<()>("no workdir")?;
        let mut co = CheckoutBuilder::new();
        co.force();

        for path in paths {
            let path = path.as_ref();
            let status = self.inner.status_file(path)?;

            if status.is_wt_new() {
                // Untracked file, discard change by deleting it
                std::fs::remove_file(workdir.join(path))?;
            } else if status.is_wt_modified() || status.is_wt_deleted() {
                // Tracked file, add to the checkout
                co.path(path);
            } else {
                // This should never happen under normal circumstances
                session::warn!(format!("unexpected file status for `{}`", path.display()));
            }
        }
        self.inner.checkout_index(None, Some(&mut co))?;

        Ok(())
    }

    pub fn fetch<'a>(
        &self,
        remote_name: Option<&str>,
        cb: RemoteCallbacks<'a>,
    ) -> joinerror::Result<()> {
        let remote_name = remote_name.unwrap_or(constants::DEFAULT_REMOTE_NAME);
        let mut remote = self.inner.find_remote(remote_name)?;
        let refspec = format!("+refs/heads/*:refs/remotes/{}/*", remote_name);

        let mut opts = git2::FetchOptions::new();
        opts.remote_callbacks(cb);
        opts.prune(git2::FetchPrune::On);

        remote.fetch(&[&refspec], Some(&mut opts), None)?;

        Ok(())
    }

    pub fn pull<'a>(
        &self,
        remote_name: Option<&str>,
        cb: RemoteCallbacks<'a>,
    ) -> joinerror::Result<()> {
        let remote = self
            .inner
            .find_remote(remote_name.unwrap_or(constants::DEFAULT_REMOTE_NAME))?;

        self.fetch(remote.name(), cb)?;

        let fetch_head = self.inner.find_reference("FETCH_HEAD")?;
        let fetch_commit = self.inner.reference_to_annotated_commit(&fetch_head)?;

        self.merge_internal(fetch_commit)?;

        Ok(())
    }

    pub fn merge(&self, branch_name: &str) -> joinerror::Result<()> {
        let incoming_reference = self
            .inner
            .find_branch(branch_name, BranchType::Local)?
            .into_reference();

        self.merge_internal(
            self.inner
                .reference_to_annotated_commit(&incoming_reference)?,
        )?;

        Ok(())
    }

    pub fn commit(&self, message: &str, sig: Signature) -> joinerror::Result<()> {
        let mut index = self.inner.index()?;
        let tree = self.inner.find_tree(index.write_tree()?)?;

        let last_commit = self.inner.head().and_then(|r| r.peel_to_commit());
        if let Ok(parent) = last_commit {
            self.inner
                .commit(Some("HEAD"), &sig, &sig, &message, &tree, &[&parent])?;
        } else {
            // Empty repo, make the initial commit
            self.inner
                .commit(Some("HEAD"), &sig, &sig, &message, &tree, &[])?;
        }
        Ok(())
    }

    pub fn current_branch(&self) -> joinerror::Result<String> {
        let head = self.inner.head()?;

        // If HEAD points to a branch, get the branch name.
        if head.is_branch() {
            let branch_name = head
                .shorthand()
                .ok_or_join_err::<()>("invalid branch name")?;
            Ok(branch_name.to_string())
        } else {
            Err(joinerror::Error::new::<()>("HEAD is detached"))
        }
    }

    pub fn create_branch(
        &self,
        branch_name: &str,
        base_branch: Option<(&str, BranchType)>,
        force: bool,
    ) -> joinerror::Result<()> {
        let target = if let Some(base_branch) = base_branch {
            self.inner
                .find_branch(base_branch.0, base_branch.1)?
                .get()
                .peel_to_commit()?
        } else {
            self.inner.head()?.peel_to_commit()?
        };

        self.inner.branch(branch_name, &target, force)?;

        Ok(())
    }

    /// If remote_branch is None, configured remote for the branch will be used, otherwise "origin"
    /// If local_branch is None, currently checked out branch will be pushed, similar to `git push`
    /// If remote_branch is None, the same name as the local_branch will be used
    /// If set_upstream is true, configuration will be updated
    pub fn push<'a>(
        &self,
        remote_name: Option<&str>,
        local_branch: Option<&str>,
        remote_branch: Option<&str>,
        set_upstream: bool,
        cb: RemoteCallbacks<'a>,
    ) -> joinerror::Result<()> {
        // If no local_branch is provided, push the current branch
        let local_branch = if let Some(local_branch) = local_branch {
            local_branch.to_string()
        } else {
            self.current_branch()?
        };

        let mut conf = self.inner.config()?;

        // If no remote_name is specified, use the configured remote for the branch,
        // before falling back to `origin`
        let remote_name = if let Some(remote_name) = remote_name {
            remote_name.to_string()
        } else {
            conf.get_string(&format!("branch.{}.remote", local_branch))
                .unwrap_or(constants::DEFAULT_REMOTE_NAME.to_string())
        };

        let mut remote = self.inner.find_remote(&remote_name)?;
        let mut refspecs = Vec::new();

        // If no remote_branch is specified, use the configured refspec
        if let Some(remote_branch) = remote_branch {
            refspecs.push(format!(
                "refs/heads/{}:refs/heads/{}",
                local_branch, remote_branch
            ));
        } else {
            refspecs.push(format!(
                "refs/heads/{}:refs/heads/{}",
                local_branch, local_branch
            ));
        }

        remote.push(
            refspecs.as_slice(),
            Some(&mut PushOptions::new().remote_callbacks(cb)),
        )?;

        if set_upstream {
            conf.set_str(&format!("branch.{}.remote", local_branch), &remote_name)?;
            let remote_branch = remote_branch.unwrap_or(local_branch.as_str());
            conf.set_str(
                &format!("branch.{}.merge", local_branch),
                &format!("refs/heads/{}", remote_branch),
            )?;
        }

        Ok(())
    }

    pub fn statuses(&self) -> joinerror::Result<HashMap<PathBuf, FileStatus>> {
        let mut opts = StatusOptions::new();
        opts.include_untracked(true).recurse_untracked_dirs(true);

        let statuses = self.inner.statuses(Some(&mut opts))?;

        let mut result = HashMap::with_capacity(statuses.len());
        for entry in statuses.iter() {
            let path = entry.path();
            if path.is_none() {
                continue; // Ignore non-UTF8 paths
            }

            let path = PathBuf::from(path.unwrap());

            // FIXME: Technically if the status begins with INDEX_, it means they are already staged
            // This should never happen if the user interacts with git only through our application
            // Since we will only stage files immediately before making a commit
            // Not sure if we should support unstaging such changes in our app
            // For now those changes will not be displayed
            match entry.status() {
                Status::WT_NEW => result.insert(path, FileStatus::Added),
                Status::WT_MODIFIED => result.insert(path, FileStatus::Modified),
                Status::WT_DELETED => result.insert(path, FileStatus::Deleted),
                _ => continue,
            };
        }

        Ok(result)
    }

    // Helper functions
}
impl Repository {
    fn merge_internal(&self, incoming_commit: git2::AnnotatedCommit) -> joinerror::Result<()> {
        let our_branch = self.current_branch()?;

        let analysis = self.inner.merge_analysis(&[&incoming_commit])?;
        dbg!("analysis success");
        if analysis.0.is_fast_forward() {
            session::info!("fast forwarding");
            // do a fast forward
            let refname = format!("refs/heads/{}", our_branch);
            match self.inner.find_reference(&refname) {
                Ok(mut r) => {
                    self.fast_forward(&mut r, &incoming_commit)?;
                }
                Err(_) => {
                    // The branch doesn't exist so just set the reference to the
                    // commit directly. Usually this is because you are pulling
                    // into an empty repository.
                    self.inner.reference(
                        &refname,
                        incoming_commit.id(),
                        true,
                        &format!("Setting {} to {}", our_branch, incoming_commit.id()),
                    )?;
                    self.inner.set_head(&refname)?;
                    self.inner.checkout_head(Some(
                        git2::build::CheckoutBuilder::default()
                            .allow_conflicts(true)
                            .conflict_style_merge(true)
                            .force(),
                    ))?;
                }
            };
        } else if analysis.0.is_normal() {
            session::info!("normal merging");
            // do a normal merge
            let head_commit = self
                .inner
                .reference_to_annotated_commit(&self.inner.head()?)?;
            self.normal_merge(&head_commit, &incoming_commit)?;
        } else {
            println!("Nothing to do...");
        }
        Ok(())
    }
    fn fast_forward(
        &self,
        our_reference: &mut git2::Reference,
        incoming_commit: &git2::AnnotatedCommit,
    ) -> joinerror::Result<()> {
        let name = match our_reference.name() {
            Some(s) => s.to_string(),
            None => String::from_utf8_lossy(our_reference.name_bytes()).to_string(),
        };
        let msg = format!(
            "Fast-Forward: Setting {} to id: {}",
            name,
            incoming_commit.id()
        );

        println!("{}", msg);

        // If the worktree is dirty, we stop and prompt the user to stash local changes
        if self.is_worktree_dirty()? {
            // TODO: Prompt the user to stash local changes
            return Err(Error::new::<DirtyWorktree>(
                "cannot fast-forward when the worktree is dirty",
            ));
        }

        // fast-forwarding
        our_reference.set_target(incoming_commit.id(), &msg)?;
        self.inner.set_head(&name)?;
        self.inner.checkout_head(Some(
            // The worktree must be clean
            &mut git2::build::CheckoutBuilder::default(),
        ))?;

        Ok(())
    }

    fn normal_merge(
        &self,
        our_commit: &git2::AnnotatedCommit,
        incoming_commit: &git2::AnnotatedCommit,
    ) -> joinerror::Result<()> {
        let our_tree = self.inner.find_commit(our_commit.id())?.tree()?;
        let incoming_tree = self.inner.find_commit(incoming_commit.id())?.tree()?;
        let ancestor = self
            .inner
            .find_commit(
                self.inner
                    .merge_base(our_commit.id(), incoming_commit.id())?,
            )?
            .tree()?;
        let mut idx = self
            .inner
            .merge_trees(&ancestor, &our_tree, &incoming_tree, None)?;

        if idx.has_conflicts() {
            session::warn!("Conflict detected");
            self.inner.checkout_index(Some(&mut idx), None)?;
            return Ok(());
        }
        let result_tree = self.inner.find_tree(idx.write_tree_to(&self.inner)?)?;

        // Create the merge commit
        let msg = format!("Merge: {} into {}", incoming_commit.id(), our_commit.id());
        let sig = self.inner.signature()?;
        let local_commit = self.inner.find_commit(our_commit.id())?;
        let remote_commit = self.inner.find_commit(incoming_commit.id())?;

        // Do our merge commit and set current branch head to that commit.
        let _merge_commit = self.inner.commit(
            Some("HEAD"),
            &sig,
            &sig,
            &msg,
            &result_tree,
            &[&local_commit, &remote_commit],
        )?;

        // Set working tree to match head.
        self.inner.checkout_head(None)?;
        Ok(())
    }

    fn is_worktree_dirty(&self) -> joinerror::Result<bool> {
        let mut opts = git2::StatusOptions::new();
        opts.include_untracked(true)
            .include_ignored(false)
            .recurse_untracked_dirs(true)
            .show(git2::StatusShow::IndexAndWorkdir);

        let statuses = self.inner.statuses(Some(&mut opts))?;
        Ok(statuses.iter().any(|e| {
            let s = e.status();
            // treat anything that is not CURRENT (unmodified) as dirty
            s != git2::Status::CURRENT
        }))
    }
}
