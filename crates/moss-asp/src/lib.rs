use moss_keyring::KeyringClient;
use oauth2::ClientSecret;
use rustc_hash::FxHashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use zeroize::{Zeroize, ZeroizeOnDrop};

const GITHUB_SECRET_PREFIX: &str = "gh-secret";
const GITLAB_SECRET_PREFIX: &str = "gl-secret";

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

#[derive(Clone)]
pub struct AppSecretsProvider {
    cache: Arc<RwLock<FxHashMap<String, SecretString>>>,
    keyring: Arc<dyn KeyringClient>,
}

impl AppSecretsProvider {
    pub fn new(
        github_client_secret: String,
        gitlab_client_secret: String,
        keyring: Arc<dyn KeyringClient>,
    ) -> joinerror::Result<Self> {
        keyring.set_secret(GITHUB_SECRET_PREFIX, &github_client_secret)?;
        keyring.set_secret(GITLAB_SECRET_PREFIX, &gitlab_client_secret)?;

        Ok(Self {
            keyring,
            cache: Arc::new(RwLock::new(FxHashMap::default())),
        })
    }

    pub async fn github_client_secret(&self) -> joinerror::Result<ClientSecret> {
        let cache = self.cache.read().await;
        if let Some(cached_secret) = cache.get(GITHUB_SECRET_PREFIX) {
            return Ok(ClientSecret::new(cached_secret.expose().to_string()));
        }

        let bytes = self.keyring.get_secret(GITHUB_SECRET_PREFIX)?;
        let utf8_string =
            String::from_utf8(bytes).map_err(|e| joinerror::Error::new::<()>(e.to_string()))?;

        self.cache.write().await.insert(
            GITHUB_SECRET_PREFIX.to_string(),
            SecretString::new(utf8_string.clone()),
        );

        Ok(ClientSecret::new(utf8_string))
    }

    pub async fn gitlab_client_secret(&self) -> joinerror::Result<ClientSecret> {
        let cache = self.cache.read().await;
        if let Some(cached_secret) = cache.get(GITLAB_SECRET_PREFIX) {
            return Ok(ClientSecret::new(cached_secret.expose().to_string()));
        }

        let bytes = self.keyring.get_secret(GITLAB_SECRET_PREFIX)?;
        let secret_string =
            String::from_utf8(bytes).map_err(|e| joinerror::Error::new::<()>(e.to_string()))?;

        self.cache.write().await.insert(
            GITLAB_SECRET_PREFIX.to_string(),
            SecretString::new(secret_string.clone()),
        );

        Ok(ClientSecret::new(secret_string))
    }
}
