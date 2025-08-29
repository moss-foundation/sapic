use keyring::Entry;
use moss_logging::session;

pub trait KeyringClient: Send + Sync {
    fn set_secret(&self, key: &str, secret: &str) -> joinerror::Result<()>;
    fn get_secret(&self, key: &str) -> joinerror::Result<Vec<u8>>;
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
    fn set_secret(&self, key: &str, secret: &str) -> joinerror::Result<()> {
        session::trace!("Setting secret for key: {}", key);

        Entry::new(key, &self.user)
            .map_err(|e| joinerror::Error::new::<()>(e.to_string()))?
            .set_secret(secret.as_bytes())
            .map_err(|e| joinerror::Error::new::<()>(e.to_string()))?;

        Ok(())
    }

    fn get_secret(&self, key: &str) -> joinerror::Result<Vec<u8>> {
        session::trace!("Getting secret for key: {}", key);

        let bytes = Entry::new(key, &self.user)
            .map_err(|e| joinerror::Error::new::<()>(e.to_string()))?
            .get_secret()
            .map_err(|e| joinerror::Error::new::<()>(e.to_string()))?;

        Ok(bytes)
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
