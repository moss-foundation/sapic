use keyring::{Entry, Result};

pub trait KeyringClient {
    fn set_secret(&self, key: &str, secret: &str) -> Result<()>;
    fn get_secret(&self, key: &str) -> Result<Vec<u8>>;
}

pub struct KeyringClientImpl {
    user: String,
}

impl KeyringClientImpl {
    pub fn new() -> Self {
        Self {
            user: whoami::username(),
        }
    }
}

impl KeyringClient for KeyringClientImpl {
    fn set_secret(&self, key: &str, secret: &str) -> Result<()> {
        Entry::new(key, &self.user)?.set_secret(secret.as_bytes())
    }

    fn get_secret(&self, key: &str) -> Result<Vec<u8>> {
        Entry::new(key, &self.user)?.get_secret()
    }
}

#[cfg(test)]
mod tests {
    use keyring::Entry;

    #[test]
    #[ignore]
    fn manual_set() {
        let entry = Entry::new("my-service", "my-name").unwrap();
        entry.set_secret("topS3cr3tP4$$w0rd".as_bytes()).unwrap();
    }

    #[test]
    #[ignore]
    fn manual_get() {
        let entry = Entry::new("gitlab_auth_agent", &whoami::username()).unwrap();
        entry.delete_credential().unwrap();
        // let password = entry.get_secret().unwrap();
        // println!("My password is '{}'", password);
    }
}
