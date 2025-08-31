use async_trait::async_trait;
use joinerror::Error;
use keyring::Entry;
use moss_logging::session;
use rustc_hash::FxHashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use zeroize::{Zeroize, ZeroizeOnDrop};

const SERVICE: &str = "sapic";

#[async_trait]
pub trait KeyringClient: Send + Sync {
    async fn set_secret(&self, key: &str, secret: &str) -> joinerror::Result<()>;
    async fn get_secret(&self, key: &str) -> joinerror::Result<Vec<u8>>;
    async fn delete_secret(&self, key: &str) -> joinerror::Result<()>;
}

#[derive(Clone, Zeroize, ZeroizeOnDrop)]
struct SecretString(String);

impl SecretString {
    pub fn new(secret: String) -> Self {
        Self(secret)
    }

    fn expose(&self) -> &str {
        &self.0
    }
}

pub struct KeyringClientImpl {
    user: String,
    cache: Arc<RwLock<FxHashMap<String, SecretString>>>,
}

impl KeyringClientImpl {
    pub fn new() -> Self {
        Self {
            user: whoami::username(),
            cache: Arc::new(RwLock::new(FxHashMap::default())),
        }
    }
}

#[async_trait]
impl KeyringClient for KeyringClientImpl {
    async fn set_secret(&self, key: &str, secret: &str) -> joinerror::Result<()> {
        let key = format!("{}/{}", SERVICE, key);
        session::trace!("Setting secret for key: {}", key);

        Entry::new(&key, &self.user)
            .map_err(|e| Error::new::<()>(e.to_string()))?
            .set_secret(secret.as_bytes())
            .map_err(|e| Error::new::<()>(e.to_string()))?;

        self.cache
            .write()
            .await
            .insert(key, SecretString::new(secret.to_string()));

        Ok(())
    }

    async fn get_secret(&self, key: &str) -> joinerror::Result<Vec<u8>> {
        let key = format!("{}/{}", SERVICE, key);

        let cache = self.cache.read().await;
        if let Some(cached_secret) = cache.get(&key) {
            session::trace!("Getting secret for key: {} from cache", key);

            Ok(cached_secret.expose().as_bytes().to_vec())
        } else {
            session::trace!("Getting secret for key: {}", key);

            let bytes = Entry::new(&key, &self.user)
                .map_err(|e| Error::new::<()>(e.to_string()))?
                .get_secret()
                .map_err(|e| Error::new::<()>(e.to_string()))?;

            let secret_string = String::from_utf8(bytes.clone())
                .map_err(|e| Error::new::<()>(format!("Invalid UTF-8 in secret: {}", e)))?;

            self.cache
                .write()
                .await
                .insert(key, SecretString::new(secret_string));

            Ok(bytes)
        }
    }

    async fn delete_secret(&self, key: &str) -> joinerror::Result<()> {
        let key = format!("{}/{}", SERVICE, key);
        session::trace!("Deleting secret for key: {}", key);

        Entry::new(&key, &self.user)
            .map_err(|e| Error::new::<()>(e.to_string()))?
            .delete_credential()
            .map_err(|e| Error::new::<()>(e.to_string()))?;

        self.cache.write().await.remove(&key);

        Ok(())
    }
}

#[cfg(any(test, feature = "test"))]
pub mod test {
    use super::*;

    pub struct MockKeyringClient {
        values: Arc<RwLock<FxHashMap<String, SecretString>>>,
    }

    impl MockKeyringClient {
        pub fn new() -> Self {
            Self {
                values: Arc::new(RwLock::new(FxHashMap::default())),
            }
        }
    }

    #[async_trait]
    impl KeyringClient for MockKeyringClient {
        async fn set_secret(&self, key: &str, secret: &str) -> joinerror::Result<()> {
            self.values
                .write()
                .await
                .insert(key.to_string(), SecretString::new(secret.to_string()));

            Ok(())
        }

        async fn get_secret(&self, key: &str) -> joinerror::Result<Vec<u8>> {
            if let Some(secret) = self.values.read().await.get(key) {
                Ok(secret.expose().as_bytes().to_vec())
            } else {
                Err(Error::new::<()>(format!(
                    "[MockKeyringClient] Secret not found for key: {}",
                    key
                )))
            }
        }

        async fn delete_secret(&self, key: &str) -> joinerror::Result<()> {
            self.values.write().await.remove(key);
            Ok(())
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
