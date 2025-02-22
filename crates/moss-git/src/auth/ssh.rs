use crate::auth::AuthAgent;
use anyhow::Result;
use git2::{Cred, RemoteCallbacks};
use std::path::PathBuf;

#[derive(Clone)]
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
    fn generate_callback<'a>(&'a self, cb: &mut RemoteCallbacks<'a>) -> Result<()> {
        cb.credentials(|_url, username_from_url, _allowed_types| {
            Cred::ssh_key(
                "git",
                self.public_key.as_deref(),
                &self.private_key,
                self.passphrase.as_deref(),
            )
        });
        Ok(())
    }
}
