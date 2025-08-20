pub use git2::{BranchType, IndexAddOption, Signature};
use git2::{
    IntoCString, PushOptions, RemoteCallbacks, Repository, Status, StatusOptions,
    build::{CheckoutBuilder, RepoBuilder},
};
use joinerror::{OptionExt, ResultExt};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::{GitAuthAgent, constants::DEFAULT_REMOTE_NAME};

// https://github.com/rust-lang/git2-rs/issues/194

unsafe impl Send for RepoHandle {}
unsafe impl Sync for RepoHandle {}

pub enum FileStatus {
    Added,
    Modified,
    Deleted,
}

/// Since all the git operations are synchronous, we should wrap all RepoHandle operations
/// with `tokio::task::spawn_blocking`.
/// This will prevent git operations from blocking the main thread
pub struct RepoHandle {
    auth_agent: Arc<dyn GitAuthAgent>,
    repo: Repository,
}

// https://stackoverflow.com/questions/27672722/libgit2-commit-example
// https://github.com/rust-lang/git2-rs/tree/master/examples

// TODO: Use callback to return/display progress
impl RepoHandle {
    // Must
    pub fn clone(
        url: &str,
        path: &Path,
        auth_agent: Arc<dyn GitAuthAgent>,
    ) -> joinerror::Result<RepoHandle> {
        let mut callbacks = RemoteCallbacks::new();
        auth_agent.generate_callback(&mut callbacks)?;

        let mut fetch_opts = git2::FetchOptions::new();
        fetch_opts.remote_callbacks(callbacks);
        let mut builder = RepoBuilder::new();
        builder.fetch_options(fetch_opts);

        let repo = builder.clone(url, &path)?;

        Ok(RepoHandle {
            auth_agent: auth_agent.clone(),
            repo,
        })
    }

    pub fn open(path: &Path, auth_agent: Arc<dyn GitAuthAgent>) -> joinerror::Result<RepoHandle> {
        let repo = Repository::open(path)?;

        Ok(RepoHandle { auth_agent, repo })
    }

    pub fn init(path: &Path, auth_agent: Arc<dyn GitAuthAgent>) -> joinerror::Result<RepoHandle> {
        let repo = Repository::init(path)?;

        Ok(RepoHandle { auth_agent, repo })
    }
}

impl RepoHandle {
    pub fn add(
        &self,
        paths: impl IntoIterator<Item = impl IntoCString>,
        opts: IndexAddOption,
    ) -> joinerror::Result<()> {
        let mut index = self.repo.index()?;
        index.add_all(paths, opts, None)?;
        index.write()?;
        Ok(())
    }

    pub fn remove(
        &self,
        paths: impl IntoIterator<Item = impl IntoCString>,
    ) -> joinerror::Result<()> {
        let mut index = self.repo.index()?;
        index.remove_all(paths, None)?;
        index.write()?;
        Ok(())
    }

    pub fn commit(&self, message: &str, sig: Signature) -> joinerror::Result<()> {
        let mut index = self.repo.index()?;
        let tree = self.repo.find_tree(index.write_tree()?)?;

        let last_commit = self.repo.head().and_then(|r| r.peel_to_commit());
        if let Ok(parent) = last_commit {
            self.repo
                .commit(Some("HEAD"), &sig, &sig, &message, &tree, &[&parent])?;
        } else {
            // Empty repo, make the initial commit
            self.repo
                .commit(Some("HEAD"), &sig, &sig, &message, &tree, &[])?;
        }
        Ok(())
    }

    /// If remote_branch is None, configured remote for the branch will be used, otherwise "origin"
    /// If local_branch is None, currently checked out branch will be pushed, similar to `git push`
    /// If remote_branch is None, configured refspec will be used
    /// If set_upstream is true, configuration will be updated
    pub fn push(
        &self,
        remote_name: Option<&str>,
        local_branch: Option<&str>,
        remote_branch: Option<&str>,
        set_upstream: bool,
    ) -> joinerror::Result<()> {
        let repo: &Repository = &self.repo;

        // If no local_branch is provided, push the current branch
        let local_branch = if let Some(local_branch) = local_branch {
            local_branch.to_string()
        } else {
            self.current_branch()?
        };

        let mut cfg = repo.config()?;

        // If no remote_name is specified, use the configured remote for the branch,
        // before falling back to `origin`
        let remote_name = if let Some(remote_name) = remote_name {
            remote_name.to_string()
        } else {
            cfg.get_string(&format!("branch.{}.remote", local_branch))
                .unwrap_or(DEFAULT_REMOTE_NAME.to_string())
        };

        let mut remote = repo.find_remote(&remote_name)?;

        let mut callbacks = RemoteCallbacks::new();
        self.auth_agent.generate_callback(&mut callbacks)?;

        // register push_update_reference to catch server rejection messages
        callbacks.push_update_reference(|refname, message| {
            if let Some(status_msg) = message {
                eprintln!("push update for {} failed: {}", refname, status_msg);
                return Err(git2::Error::from_str(&format!(
                    "push update for {} failed: {}",
                    refname, status_msg
                )));
            }
            Ok(())
        });

        let mut refspecs = Vec::new();

        // If no remote_branch is specified, use the configured refspec
        if let Some(remote_branch) = remote_branch {
            refspecs.push(format!(
                "refs/heads/{}:refs/heads/{}",
                local_branch, remote_branch
            ));
        }

        remote.push(
            refspecs.as_slice(),
            Some(&mut PushOptions::new().remote_callbacks(callbacks)),
        )?;

        if set_upstream {
            cfg.set_str(&format!("branch.{}.remote", local_branch), &remote_name)?;
            let remote_branch = remote_branch.unwrap_or(local_branch.as_str());
            cfg.set_str(
                &format!("branch.{}.merge", local_branch),
                &format!("refs/heads/{}", remote_branch),
            )?;
        }

        Ok(())
    }

    pub fn fetch(&self, remote_name: Option<&str>) -> joinerror::Result<()> {
        let remote_name = remote_name.unwrap_or(DEFAULT_REMOTE_NAME);
        let mut remote = self.repo.find_remote(remote_name)?;
        let refspec = format!("+refs/heads/*:refs/remotes/{}/*", remote_name);

        let mut fetch_opts = git2::FetchOptions::new();
        let mut callbacks = RemoteCallbacks::new();
        self.auth_agent.generate_callback(&mut callbacks)?;
        fetch_opts.remote_callbacks(callbacks);
        fetch_opts.prune(git2::FetchPrune::On);

        remote.fetch(&[&refspec], Some(&mut fetch_opts), None)?;

        Ok(())
    }

    pub fn merge(&self, branch_name: &str) -> joinerror::Result<()> {
        let incoming_reference = self
            .repo
            .find_branch(branch_name, git2::BranchType::Local)?
            .into_reference();
        self.merge_helper(
            self.repo
                .reference_to_annotated_commit(&incoming_reference)?,
        )?;

        Ok(())
    }

    pub fn pull(&self, remote_name: Option<&str>) -> joinerror::Result<()> {
        // Pull = Fetch + Merge FETCH_HEAD
        let remote = self
            .repo
            .find_remote(remote_name.unwrap_or(DEFAULT_REMOTE_NAME))?;
        let mut callbacks = RemoteCallbacks::new();
        self.auth_agent.generate_callback(&mut callbacks)?;

        self.fetch(remote.name())?;
        let fetch_head = self.repo.find_reference("FETCH_HEAD")?;
        let fetch_commit = self.repo.reference_to_annotated_commit(&fetch_head)?;

        self.merge_helper(fetch_commit)?;

        Ok(())
    }

    pub fn add_remote(&self, remote_name: Option<&str>, remote_url: &str) -> joinerror::Result<()> {
        self.repo
            .remote(remote_name.unwrap_or(DEFAULT_REMOTE_NAME), remote_url)?;

        Ok(())
    }

    /// Return a list of remote names and their urls
    // TODO: Support remote with special push_url?
    pub fn list_remotes(&self) -> joinerror::Result<HashMap<String, String>> {
        let remote_names = self.repo.remotes()?;
        let mut result = HashMap::new();

        for remote_name in remote_names.iter() {
            if remote_name.is_none() {
                continue;
            }
            let remote_name = remote_name.unwrap();
            let remote = self.repo.find_remote(remote_name)?;
            let url = remote.url();
            if url.is_none() {
                continue;
            }
            result.insert(remote_name.to_string(), url.unwrap().to_string());
        }
        Ok(result)
    }

    pub fn list_branches(&self, branch_type: Option<BranchType>) -> joinerror::Result<Vec<String>> {
        let branches = self.repo.branches(branch_type)?;

        let names = branches
            .into_iter()
            .filter_map(|b| {
                if let Ok((branch, _)) = b {
                    branch.name().ok().flatten().map(|name| name.to_string())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        Ok(names)
    }

    pub fn current_branch(&self) -> joinerror::Result<String> {
        let head = self.repo.head()?;

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

    /// If base_branch is None, the new branch will be based on the current HEAD
    pub fn create_branch(
        &self,
        branch_name: &str,
        base_branch: Option<(&str, BranchType)>,
        force: bool,
    ) -> joinerror::Result<()> {
        let target = if let Some(base_branch) = base_branch {
            self.repo
                .find_branch(base_branch.0, base_branch.1)?
                .get()
                .peel_to_commit()?
        } else {
            self.repo.head()?.peel_to_commit()?
        };

        self.repo.branch(branch_name, &target, force)?;

        Ok(())
    }

    /// FIXME: Right now we are only renaming the
    pub fn rename_branch(
        &self,
        old_name: &str,
        new_name: &str,
        force: bool,
    ) -> joinerror::Result<()> {
        let mut branch = self.repo.find_branch(old_name, BranchType::Local)?;
        branch.rename(new_name, force)?;
        Ok(())
    }

    pub fn checkout_branch(
        &self,
        remote_name: Option<&str>,
        branch_name: &str,
        create_from_remote: bool,
    ) -> joinerror::Result<()> {
        // Try to find an existing local branch
        match self.repo.find_branch(branch_name, BranchType::Local) {
            // A local branch is found
            Ok(branch) => {
                // Convert branch wrapper into its underlying reference, get full refname.
                let reference = branch.into_reference();
                let refname = reference
                    .name()
                    .ok_or_join_err::<()>("invalid branch refname")?;
                // Set HEAD to that branch reference (e.g. "refs/heads/<branch>")
                self.repo.set_head(refname)?;
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
                let remote = remote_name.unwrap_or(DEFAULT_REMOTE_NAME);
                let remote_refname = format!("refs/remotes/{}/{}", remote, branch_name);
                let remote_ref = self
                    .repo
                    .find_reference(&remote_refname)
                    .join_err::<()>("failed to find remote branch reference")?;

                let target_commit = remote_ref.peel_to_commit()?;
                let mut local_branch = self.repo.branch(branch_name, &target_commit, false)?;

                let upstream_name = format!("{}/{}", remote, branch_name);
                local_branch.set_upstream(Some(&upstream_name))?;

                // Set HEAD to the new local branch
                let local_refname = format!("refs/heads/{}", branch_name);
                self.repo.set_head(&local_refname)?;
            }
        }

        let mut co = CheckoutBuilder::new();
        self.repo.checkout_head(Some(&mut co))?;
        Ok(())
    }

    /// Compare a local branch and its remote-tracking branch
    /// Returns (ahead_commits, behind_commits)
    pub fn compare_with_remote_branch(
        &self,
        branch_name: &str,
    ) -> joinerror::Result<(usize, usize)> {
        let local = self.repo.find_branch(branch_name, BranchType::Local)?;
        let upstream = local.upstream()?;

        let local_commit = local.get().peel_to_commit()?;
        let upstream_commit = upstream.get().peel_to_commit()?;
        let (ahead, behind) = self
            .repo
            .graph_ahead_behind(local_commit.id(), upstream_commit.id())?;
        Ok((ahead, behind))
    }

    pub fn get_file_statuses(&self) -> joinerror::Result<HashMap<PathBuf, FileStatus>> {
        let mut status_opts = StatusOptions::new();
        status_opts
            .include_untracked(true)
            .recurse_untracked_dirs(true);

        let statuses = self.repo.statuses(Some(&mut status_opts))?;
        let mut out = HashMap::new();

        for entry in statuses.iter() {
            let path = entry.path();
            if path.is_none() {
                // Ignore non-UTF8 paths
                continue;
            }
            let path = PathBuf::from(path.unwrap());

            // FIXME: Technically if the status begins with INDEX_, it means they are already staged
            // This should never happen if the user interacts with git only through our application
            // Since we will only stage files immediately before making a commit
            // Not sure if we should support unstaging such changes in our app
            // For now those changes will not be displayed
            if entry.status().intersects(Status::WT_NEW) {
                out.insert(path, FileStatus::Added);
                continue;
            }

            if entry.status().intersects(Status::WT_MODIFIED) {
                out.insert(path, FileStatus::Modified);
                continue;
            }

            if entry.status().intersects(Status::WT_DELETED) {
                out.insert(path, FileStatus::Deleted);
                continue;
            }

            // TODO: Implement rename detection if it can be done robustly
            // TODO: Support for conflicted files?
        }

        Ok(out)
    }

    pub fn path(&self) -> &Path {
        self.repo.path()
    }
}
impl RepoHandle {
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
        our_reference.set_target(incoming_commit.id(), &msg)?;
        self.repo.set_head(&name)?;
        self.repo.checkout_head(Some(
            git2::build::CheckoutBuilder::default()
                // TODO: Handle dirty workspace state (stashing?)
                .force(),
        ))?;
        Ok(())
    }

    fn normal_merge(
        &self,
        our_commit: &git2::AnnotatedCommit,
        incoming_commit: &git2::AnnotatedCommit,
    ) -> joinerror::Result<()> {
        let our_tree = self.repo.find_commit(our_commit.id())?.tree()?;
        let incoming_tree = self.repo.find_commit(incoming_commit.id())?.tree()?;
        let ancestor = self
            .repo
            .find_commit(
                self.repo
                    .merge_base(our_commit.id(), incoming_commit.id())?,
            )?
            .tree()?;
        let mut idx = self
            .repo
            .merge_trees(&ancestor, &our_tree, &incoming_tree, None)?;

        if idx.has_conflicts() {
            println!("Merge conflicts detected...");
            self.repo.checkout_index(Some(&mut idx), None)?;
            return Ok(());
        }
        let result_tree = self.repo.find_tree(idx.write_tree_to(&self.repo)?)?;
        // now create the merge commit
        let msg = format!("Merge: {} into {}", incoming_commit.id(), our_commit.id());
        let sig = self.repo.signature()?;
        let local_commit = self.repo.find_commit(our_commit.id())?;
        let remote_commit = self.repo.find_commit(incoming_commit.id())?;
        // Do our merge commit and set current branch head to that commit.
        let _merge_commit = self.repo.commit(
            Some("HEAD"),
            &sig,
            &sig,
            &msg,
            &result_tree,
            &[&local_commit, &remote_commit],
        )?;
        // Set working tree to match head.
        self.repo.checkout_head(None)?;
        Ok(())
    }

    fn merge_helper(&self, incoming_commit: git2::AnnotatedCommit) -> joinerror::Result<()> {
        let head = self.repo.head()?;
        let our_branch = head.name().unwrap_or("main");

        // 1. do a merge analysis
        let analysis = self.repo.merge_analysis(&[&incoming_commit])?;

        // 2. Do the appropriate merge
        if analysis.0.is_fast_forward() {
            println!("Doing a fast forward");
            // do a fast forward
            let refname = format!("refs/heads/{}", our_branch);
            match self.repo.find_reference(&refname) {
                Ok(mut r) => {
                    self.fast_forward(&mut r, &incoming_commit)?;
                }
                Err(_) => {
                    // The branch doesn't exist so just set the reference to the
                    // commit directly. Usually this is because you are pulling
                    // into an empty repository.
                    self.repo.reference(
                        &refname,
                        incoming_commit.id(),
                        true,
                        &format!("Setting {} to {}", our_branch, incoming_commit.id()),
                    )?;
                    self.repo.set_head(&refname)?;
                    self.repo.checkout_head(Some(
                        git2::build::CheckoutBuilder::default()
                            .allow_conflicts(true)
                            .conflict_style_merge(true)
                            .force(),
                    ))?;
                }
            };
        } else if analysis.0.is_normal() {
            // do a normal merge
            let head_commit = self
                .repo
                .reference_to_annotated_commit(&self.repo.head()?)?;
            self.normal_merge(&head_commit, &incoming_commit)?;
        } else {
            println!("Nothing to do...");
        }
        Ok(())
    }
}

#[cfg(any(test, feature = "integration-tests"))]
impl RepoHandle {
    pub fn repo(&self) -> &git2::Repository {
        &self.repo
    }
}

#[cfg(test)]
mod tests {
    use git2::{Cred, IndexAddOption, RemoteCallbacks, Signature};
    use std::{path::Path, sync::Arc, time::SystemTime};

    use crate::{GitAuthAgent, repo::RepoHandle};

    // This is so that we don't have circular dependency on git-hosting-provider when testing repo
    struct TestAuthAgent {}
    impl GitAuthAgent for TestAuthAgent {
        fn generate_callback<'a>(&'a self, cb: &mut RemoteCallbacks<'a>) -> anyhow::Result<()> {
            dotenv::dotenv().ok();
            let public_key = dotenv::var("GITHUB_SSH_PUBLIC")?;
            let private_key = dotenv::var("GITHUB_SSH_PRIVATE")?;
            let password = dotenv::var("GITHUB_SSH_PASSWORD")?;

            cb.credentials(move |_url, _username_from_url, _allowed_types| {
                Cred::ssh_key(
                    "git",
                    Some(public_key.as_ref()),
                    private_key.as_ref(),
                    Some(&password),
                )
            });
            Ok(())
        }
    }

    // cargo test test_clone_add_commit_push -- --nocapture
    #[ignore]
    #[test]
    fn manual_clone_add_commit_push() {
        // TODO: Support verified signed commits using `gpg`
        // From example: https://github.com/ramosbugs/oauth2-rs/blob/main/examples/github.rs
        // https://users.rust-lang.org/t/how-to-use-git2-push-correctly/97202/6
        let repo_url = dotenv::var("GITHUB_TEST_REPO_SSH").unwrap();
        let repo_path = Path::new("test-repo");

        let auth_agent = Arc::new(TestAuthAgent {});

        let repo = RepoHandle::clone(&repo_url, &repo_path, auth_agent).unwrap();

        let time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        std::fs::write(repo_path.join("TEST.txt"), time.to_string()).unwrap();
        let author = Signature::now("Hongyu Yang", "brutusyhy@gmail.com").unwrap();

        // Git Add
        repo.add(vec![Path::new("TEST.txt")], IndexAddOption::DEFAULT)
            .expect("Failed to add");

        // Git Commit
        repo.commit(&format!("Test Commit {time}"), author)
            .expect("Failed to commit");

        // Git Push
        repo.push(Some("origin"), Some("main"), Some("main"), true)
            .expect("Failed to push");
    }

    #[ignore]
    #[test]
    fn manual_open_fetch_pull() {
        let _repo_url = dotenv::var("GITHUB_TEST_REPO_SSH").unwrap();
        let repo_path = Path::new("test-repo");

        let auth_agent = Arc::new(TestAuthAgent {});

        let repo = RepoHandle::open(repo_path, auth_agent).unwrap();

        println!(
            "HEAD before pulling: {}",
            repo.repo
                .find_reference("HEAD")
                .unwrap()
                .peel_to_commit()
                .unwrap()
                .message()
                .unwrap()
        );

        let _ = repo.fetch(Some("origin"));

        println!(
            "Current FETCH_HEAD: {}",
            repo.repo
                .find_reference("FETCH_HEAD")
                .unwrap()
                .peel_to_commit()
                .unwrap()
                .message()
                .unwrap()
        );

        repo.pull(Some("origin")).unwrap();

        println!(
            "Current HEAD: {}",
            repo.repo
                .find_reference("HEAD")
                .unwrap()
                .peel_to_commit()
                .unwrap()
                .message()
                .unwrap()
        );
    }
}
