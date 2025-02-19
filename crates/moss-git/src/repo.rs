use crate::auth::AuthAgent;
use std::path::PathBuf;

pub struct SAPICRepo {
    url: String,
    path: PathBuf,
    auth_agent: Box<dyn AuthAgent>,
}

#[cfg(test)]
mod test {
    use crate::auth::{AuthAgent, OAuthAgent, TestStorage};
    use git2::{IndexAddOption, PushOptions, Signature};
    use std::path::Path;
    use std::str::from_utf8;
    use std::time::SystemTime;

    // cargo test add_commit_push -- --nocapture
    #[test]
    fn add_commit_push() {
        // TODO: Support verified signed commits using `gpg`
        // From example: https://github.com/ramosbugs/oauth2-rs/blob/main/examples/github.rs
        // https://users.rust-lang.org/t/how-to-use-git2-push-correctly/97202/6
        let repo_url = "https://github.com/**/**.git";
        let repo_path = Path::new("Path to your local repo");

        let auth_url = "https://github.com/login/oauth/authorize";
        let token_url = "https://github.com/login/oauth/access_token";
        let client_id = "***";
        let client_secret = "***";

        let mut auth_agent = match OAuthAgent::read_from_file() {
            Ok(agent) => agent,
            Err(_) => Box::new(OAuthAgent::new(
                auth_url,
                token_url,
                client_id,
                client_secret,
                vec![],
                None,
            )),
        };

        let mut callbacks = git2::RemoteCallbacks::new();
        auth_agent.authorize(&mut callbacks).unwrap();
        let mut repo = crate::clone_flow(repo_url, repo_path, callbacks).unwrap();

        let time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        std::fs::write(repo_path.join("TEST.txt"), time.to_string()).unwrap();
        let author = Signature::now("Hongyu Yang", "brutusyhy@gmail.com").unwrap();
        let mut index = repo.index().unwrap();
        index
            .add_all(vec![Path::new("TEST.txt")], IndexAddOption::DEFAULT, None)
            .unwrap();
        index.write().unwrap();

        let tree = repo.find_tree(index.write_tree().unwrap()).unwrap();
        let last_commit = repo.head().unwrap().peel_to_commit().unwrap();

        let commit_buffer = repo
            .commit_create_buffer(
                &author,
                &author,
                &format!("Test Commit {}", time),
                &tree,
                &[&last_commit],
            )
            .unwrap();

        let commit_content = from_utf8(&commit_buffer).unwrap();
        let commit_oid = repo.commit_signed(commit_content, "", None).unwrap();
        repo.reference(
            "refs/heads/main",
            commit_oid,
            true,
            "Update branch with signed commit",
        )
        .unwrap();

        let mut origin = repo.find_remote("origin").unwrap();
        let mut callbacks = git2::RemoteCallbacks::new();
        let mut auth_agent = OAuthAgent::read_from_file().unwrap();
        auth_agent.authorize(&mut callbacks).unwrap();
        origin
            .push(
                &[&format!("refs/heads/main:refs/heads/main")],
                Some(&mut PushOptions::new().remote_callbacks(callbacks)),
            )
            .unwrap();
    }
}
