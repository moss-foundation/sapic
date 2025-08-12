use anyhow::{Result, anyhow};
use git2::{
    IndexMatchedPath, IntoCString, PushOptions, RemoteCallbacks, Repository, build::RepoBuilder,
};
use std::{path::Path, sync::Arc};

pub use git2::{BranchType, IndexAddOption, Signature};

use crate::GitAuthAgent;

// https://github.com/rust-lang/git2-rs/issues/194

unsafe impl Send for RepoHandle {}
unsafe impl Sync for RepoHandle {}

/// Since all the git operations are synchronous, and authentication requires blocking `reqwest`
/// We must wrap all RepoHandle operations with `tokio::task::spawn_blocking`
pub struct RepoHandle {
    auth_agent: Arc<dyn GitAuthAgent>,
    repo: Repository,
}

// https://stackoverflow.com/questions/27672722/libgit2-commit-example
// https://github.com/rust-lang/git2-rs/tree/master/examples

// TODO: Use callback to return/display progress
impl RepoHandle {
    pub fn clone(url: &str, path: &Path, auth_agent: Arc<dyn GitAuthAgent>) -> Result<RepoHandle> {
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

    pub fn open(path: &Path, auth_agent: Arc<dyn GitAuthAgent>) -> Result<RepoHandle> {
        let repo = Repository::open(path)?;

        Ok(RepoHandle { auth_agent, repo })
    }

    pub fn init(path: &Path, auth_agent: Arc<dyn GitAuthAgent>) -> Result<RepoHandle> {
        let repo = Repository::init(path)?;

        Ok(RepoHandle { auth_agent, repo })
    }
}

impl RepoHandle {
    pub fn add(
        &self,
        paths: impl IntoIterator<Item = impl IntoCString>,
        opts: IndexAddOption,
    ) -> Result<()> {
        let mut index = self.repo.index()?;
        index.add_all(paths, opts, None)?;
        index.write()?;
        Ok(())
    }

    pub fn remove(&self, paths: impl IntoIterator<Item = impl IntoCString>) -> Result<()> {
        let mut index = self.repo.index()?;
        index.remove_all(paths, None)?;
        index.write()?;
        Ok(())
    }

    pub fn commit(&self, message: &str, sig: Signature) -> Result<()> {
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

    //
    pub fn push(
        &self,
        remote_name: Option<&str>,
        local_branch_name: Option<&str>,
        remote_branch_name: Option<&str>,
        set_upstream: bool,
    ) -> Result<()> {
        let remote_name = remote_name.unwrap_or("origin");
        // FIXME: does it make sense to default to main branch?
        let local_branch_name = local_branch_name.unwrap_or("main");
        let remote_branch_name = remote_branch_name.unwrap_or("main");
        let mut remote = self.repo.find_remote(remote_name)?;
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

        remote.push(
            &[&format!(
                "refs/heads/{}:refs/heads/{}",
                local_branch_name, remote_branch_name
            )],
            Some(&mut PushOptions::new().remote_callbacks(callbacks)),
        )?;

        if set_upstream {
            let mut branch = self
                .repo
                .find_branch(local_branch_name, BranchType::Local)?;
            branch.set_upstream(Some(remote_branch_name))?;
        }
        Ok(())
    }

    pub fn fetch(&self, remote_name: Option<&str>) -> Result<()> {
        let remote_name = remote_name.unwrap_or("origin");
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

    pub fn merge(&self, branch_name: &str) -> Result<()> {
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

    pub fn pull(&self, remote_name: Option<&str>) -> Result<()> {
        // Pull = Fetch + Merge FETCH_HEAD
        let remote = self.repo.find_remote(remote_name.unwrap_or("origin"))?;
        let mut callbacks = RemoteCallbacks::new();
        self.auth_agent.generate_callback(&mut callbacks)?;

        self.fetch(remote.name())?;
        let fetch_head = self.repo.find_reference("FETCH_HEAD")?;
        let fetch_commit = self.repo.reference_to_annotated_commit(&fetch_head)?;

        self.merge_helper(fetch_commit)?;

        Ok(())
    }

    pub fn add_remote(&self, remote_name: Option<&str>, remote_url: &str) -> Result<()> {
        self.repo
            .remote(remote_name.unwrap_or("origin"), remote_url)?;

        Ok(())
    }

    pub fn list_branches(&self, branch_type: Option<BranchType>) -> Result<Vec<String>> {
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

    /// Only supports renaming local branch at the moment
    pub fn rename_branch(&self, old_name: &str, new_name: &str, force: bool) -> Result<()> {
        let mut branch = self.repo.find_branch(old_name, BranchType::Local)?;
        branch.rename(new_name, force)?;
        Ok(())
    }

    /// If base_branch is None, the new branch will be based on the current HEAD
    pub fn create_branch(
        &self,
        branch_name: &str,
        base_branch: Option<&str>,
        force: bool,
    ) -> Result<()> {
        let target = if let Some(base_branch) = base_branch {
            self.repo
                .find_branch(base_branch, BranchType::Local)?
                .get()
                .peel_to_commit()?
        } else {
            self.repo.head()?.peel_to_commit()?
        };

        self.repo.branch(branch_name, &target, force)?;

        Ok(())
    }

    /// Compare a local branch and its remote-tracking branch
    /// Returns (ahead_commits, behind_commits)
    pub fn compare_with_remote_branch(&self, branch_name: &str) -> Result<(usize, usize)> {
        let local = self.repo.find_branch(branch_name, BranchType::Local)?;
        let upstream = local.upstream()?;

        let local_commit = upstream.get().peel_to_commit()?;
        let upstream_commit = upstream.get().peel_to_commit()?;
        let (ahead, behind) = self
            .repo
            .graph_ahead_behind(local_commit.id(), upstream_commit.id())?;
        Ok((ahead, behind))
    }
}
impl RepoHandle {
    fn fast_forward(
        &self,
        our_reference: &mut git2::Reference,
        incoming_commit: &git2::AnnotatedCommit,
    ) -> Result<(), git2::Error> {
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
    ) -> Result<(), git2::Error> {
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

    fn merge_helper(&self, incoming_commit: git2::AnnotatedCommit) -> Result<(), git2::Error> {
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

#[cfg(test)]
mod tests {
    use crate::{GitAuthAgent, repo::RepoHandle};
    use git2::{Cred, IndexAddOption, RemoteCallbacks, Signature};
    use std::{path::Path, sync::Arc, time::SystemTime};
    use url::Url;

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
