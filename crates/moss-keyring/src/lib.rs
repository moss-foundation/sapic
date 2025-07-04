use keyring::{Entry, Result};

pub trait KeyringClient {
    fn set_secret(&self, key: &str, secret: &[u8]) -> Result<()>;
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
    fn set_secret(&self, key: &str, secret: &[u8]) -> Result<()> {
        Entry::new(key, &self.user)?.set_secret(secret)
    }

    fn get_secret(&self, key: &str) -> Result<Vec<u8>> {
        Entry::new(key, &self.user)?.get_secret()
    }
}

// Helper methods for backward compatibility with string-based secrets
impl KeyringClientImpl {
    /// Convenience method for setting string secrets
    pub fn set_secret_str(&self, key: &str, secret: &str) -> Result<()> {
        self.set_secret(key, secret.as_bytes())
    }

    /// Convenience method for getting string secrets
    /// Returns an error if the stored secret is not valid UTF-8
    pub fn get_secret_str(&self, key: &str) -> Result<String> {
        let bytes = self.get_secret(key)?;
        String::from_utf8(bytes).map_err(|_| {
            keyring::Error::NoEntry
        })
    }
}

#[cfg(test)]
mod tests {
    use keyring::Entry;

    #[ignore]
    #[test]
    fn manual_set() {
        let entry = Entry::new("my-service", "my-name").unwrap();
        entry.set_secret("topS3cr3tP4$$w0rd".as_bytes()).unwrap();
    }

    #[ignore]
    #[test]
    fn manual_get() {
        let entry = Entry::new("gitlab_auth_agent", &whoami::username()).unwrap();
        entry.delete_credential().unwrap();
        // let password = entry.get_secret().unwrap();
        // println!("My password is '{}'", password);
    }
}
