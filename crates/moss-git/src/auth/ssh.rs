use std::path::{Path, PathBuf};
use git2::{Cred, RemoteCallbacks};
use crate::auth::{AuthAgent};

pub struct SSHAgent {
    public_key: Option<&'static Path>,
    private_key: &'static Path,
    passphrase: Option<String>,
}

impl SSHAgent {
    pub fn new(public_key: Option<&'static Path>, private_key: &'static Path, passphrase: Option<String>) -> Self {
        SSHAgent {
            public_key,
            private_key,
            passphrase,
        }
    }
}

impl AuthAgent for SSHAgent {
    fn authorize(self, remote_callbacks: &mut RemoteCallbacks) {
        remote_callbacks.credentials(move |url, username, allowed| {
            Cred::ssh_key("git", self.public_key, self.private_key, self.passphrase.as_deref())
        });
    }
}