use moss_logging::session;

#[cfg(not(target_os = "macos"))]
use keyring::Entry;

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

    #[cfg(target_os = "macos")]
    fn set_secret_macos(&self, key: &str, secret: &str) -> joinerror::Result<()> {
        use security_framework::passwords::{delete_generic_password, set_generic_password};

        let service = key;
        let account = &self.user;
        let password = secret.as_bytes();

        // Delete existing password first (if it exists) to avoid conflicts
        let _ = delete_generic_password(service, account);

        // Add new password entry
        match set_generic_password(service, account, password) {
            Ok(_) => {
                session::trace!("Set secret for key: {}", key);
                Ok(())
            }
            Err(e) => Err(joinerror::Error::new::<()>(format!(
                "Failed to store secret: {}",
                e
            ))),
        }
    }

    #[cfg(target_os = "macos")]
    fn get_secret_macos(&self, key: &str) -> joinerror::Result<Vec<u8>> {
        use security_framework::passwords::get_generic_password;

        let service = key;
        let account = &self.user;

        match get_generic_password(service, account) {
            Ok(password_data) => {
                session::trace!("Retrieved secret for key: {}", key);
                Ok(password_data.to_vec())
            }
            Err(e) => Err(joinerror::Error::new::<()>(format!(
                "Failed to retrieve secret: {}",
                e
            ))),
        }
    }
}

impl KeyringClient for KeyringClientImpl {
    fn set_secret(&self, key: &str, secret: &str) -> joinerror::Result<()> {
        session::trace!("Setting secret for key: {}", key);

        #[cfg(target_os = "macos")]
        {
            // Use our custom macOS implementation that doesn't require password prompts
            return self.set_secret_macos(key, secret);
        }

        #[cfg(not(target_os = "macos"))]
        {
            // Use standard keyring implementation for other platforms
            Entry::new(key, &self.user)
                .map_err(|e| joinerror::Error::new::<()>(e.to_string()))?
                .set_secret(secret.as_bytes())
                .map_err(|e| joinerror::Error::new::<()>(e.to_string()))?;

            Ok(())
        }
    }

    fn get_secret(&self, key: &str) -> joinerror::Result<Vec<u8>> {
        session::trace!("Getting secret for key: {}", key);

        #[cfg(target_os = "macos")]
        {
            // Use our custom macOS implementation that doesn't require password prompts
            return self.get_secret_macos(key);
        }

        #[cfg(not(target_os = "macos"))]
        {
            // Use standard keyring implementation for other platforms
            let bytes = Entry::new(key, &self.user)
                .map_err(|e| joinerror::Error::new::<()>(e.to_string()))?
                .get_secret()
                .map_err(|e| joinerror::Error::new::<()>(e.to_string()))?;

            Ok(bytes)
        }
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
