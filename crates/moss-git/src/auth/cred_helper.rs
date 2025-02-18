use git2::{Config, Cred, RemoteCallbacks};
use crate::auth::AuthAgent;

/// For now, Credential Helper seems only useful for GitLab
/// As GitLab implemented the protocol that allows it to achieve the same functionality as OAuth
/// I still don't fully understand how git configs work
pub struct CredHelper {}

impl CredHelper {
    pub fn new() -> CredHelper {
        CredHelper {}
    }

}

impl AuthAgent for CredHelper {
    fn authorize(self, remote_callbacks: &mut RemoteCallbacks) {
        remote_callbacks.credentials(move |url, username_from_url, _allowed_types| {
            let default_config = Config::open_default().unwrap();
            Cred::credential_helper(&default_config, url, username_from_url)
        });
    }
}