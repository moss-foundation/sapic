use std::path::PathBuf;

#[derive(Clone)]
pub struct SSHCred {
    public_key: Option<PathBuf>,
    private_key: PathBuf,
    passphrase: Option<String>,
}

impl SSHCred {
    pub fn new(
        public_key: Option<PathBuf>,
        private_key: PathBuf,
        passphrase: Option<String>,
    ) -> Self {
        SSHCred {
            public_key,
            private_key,
            passphrase,
        }
    }
}
