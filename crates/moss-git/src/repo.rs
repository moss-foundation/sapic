use crate::auth::{AuthAgent, OAuthAgent, TestStorage};
use anyhow::Result;
use git2::build::RepoBuilder;
use git2::{IndexAddOption, IntoCString, PushOptions, RemoteCallbacks, Repository, Signature};
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub struct SAPICRepo {
    url: String,
    path: PathBuf,
    auth_agent: Arc<dyn AuthAgent>,
    repo: Repository,
}

// https://stackoverflow.com/questions/27672722/libgit2-commit-example
impl SAPICRepo {
    pub fn clone(url: &str, path: &Path, auth_agent: Arc<dyn AuthAgent>) -> Result<SAPICRepo> {
        let mut callbacks = RemoteCallbacks::new();
        auth_agent.generate_callback(&mut callbacks)?;

        let mut fo = git2::FetchOptions::new();
        fo.remote_callbacks(callbacks);
        let mut builder = RepoBuilder::new();
        builder.fetch_options(fo);

        let repo = builder.clone(url, &path)?;

        Ok(SAPICRepo {
            url: url.to_string(),
            path: path.to_owned(),
            auth_agent: auth_agent.clone(),
            repo,
        })
    }

    pub fn open(path: &Path, auth_agent: Arc<dyn AuthAgent>) -> Result<SAPICRepo> {}

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

    pub fn remove(
        &self,
        paths: impl IntoIterator<Item = impl IntoCString>,
    ) -> Result<()> {
        let mut index = self.repo.index()?;
        index.remove_all(paths, None)?;
        index.write()?;
        Ok(())
    }

    pub fn commit(&self, message: &str, sig: Signature) -> Result<()> {
        let mut index = self.repo.index()?;
        let tree = self.repo.find_tree(index.write_tree()?)?;
        let last_commit = self.repo.head()?.peel_to_commit();

        if let Ok(parent) = last_commit {
            self.repo
                .commit(Some("HEAD"), &sig, &sig, &message, &tree, &[&parent])?;
        } else {
            self.repo
                .commit(Some("HEAD"), &sig, &sig, &message, &tree, &[])?;
        }
        Ok(())
    }

    pub fn push(
        &self,
        remote_name: Option<&str>,
        local_branch_name: Option<&str>,
        remote_branch_name: Option<&str>,
    ) -> Result<()> {
        let remote_name = remote_name.unwrap_or("origin");
        let local_branch_name = local_branch_name.unwrap_or("main");
        let remote_branch_name = remote_branch_name.unwrap_or("main");
        let mut remote = self.repo.find_remote(remote_name)?;
        let mut callbacks = RemoteCallbacks::new();
        self.auth_agent.generate_callback(&mut callbacks)?;
        remote.push(
            &[&format!(
                "refs/heads/{}:refs/heads/{}",
                local_branch_name, remote_branch_name
            )],
            Some(&mut PushOptions::new().remote_callbacks(callbacks)),
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::auth::{OAuthAgent, TestStorage};
    use crate::repo::SAPICRepo;
    use git2::{IndexAddOption, Signature};
    use std::path::Path;
    use std::sync::Arc;
    use std::time::SystemTime;

    // cargo test add_commit_push -- --nocapture
    #[test]
    fn add_commit_push() {
        // TODO: Support verified signed commits using `gpg`
        // From example: https://github.com/ramosbugs/oauth2-rs/blob/main/examples/github.rs
        // https://users.rust-lang.org/t/how-to-use-git2-push-correctly/97202/6
        let repo_url = dotenv::var("GITHUB_TEST_REPO_HTTPS").unwrap();
        let repo_path = Path::new("test-repo");

        let mut auth_agent =
            OAuthAgent::read_from_file().unwrap_or_else(|_| Arc::new(OAuthAgent::github()));

        let repo = SAPICRepo::clone(&repo_url, &repo_path, auth_agent).unwrap();

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

        // Git push
        repo.push(Some("origin"), Some("main"), Some("main"))
            .expect("Failed to push");
    }
}
