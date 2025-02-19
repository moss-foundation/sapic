use crate::auth::AuthAgent;
use anyhow::Result;
use git2::{Cred, RemoteCallbacks};
use std::ops::Deref;
use std::path::PathBuf;

pub struct SSHAgent {
    public_key: Option<PathBuf>,
    private_key: PathBuf,
    passphrase: Option<String>,
}

impl SSHAgent {
    pub fn new(
        public_key: Option<PathBuf>,
        private_key: PathBuf,
        passphrase: Option<String>,
    ) -> Self {
        SSHAgent {
            public_key,
            private_key,
            passphrase,
        }
    }
}

impl AuthAgent for SSHAgent {
    fn authorize<'a>(mut self, remote_callbacks: &mut RemoteCallbacks<'a>) -> Result<()> {
        remote_callbacks.credentials(move |url, username, allowed| {
            Cred::ssh_key(
                username.unwrap(),
                self.public_key.as_deref(),
                self.private_key.deref(),
                self.passphrase.as_deref(),
            )
        });
        Ok(())
    }
}
