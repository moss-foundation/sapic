use moss_keyring::KeyringClient;
use oauth2::ClientSecret;
use std::sync::Arc;

const GITHUB_SECRET_PREFIX: &str = "gh-secret";
const GITLAB_SECRET_PREFIX: &str = "gl-secret";

#[derive(Clone)]
pub struct AppSecretsProvider {
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

        Ok(Self { keyring })
    }

    pub fn github_client_secret(&self) -> joinerror::Result<ClientSecret> {
        let bytes = self.keyring.get_secret(GITHUB_SECRET_PREFIX)?;
        let secret_string =
            String::from_utf8(bytes).map_err(|e| joinerror::Error::new::<()>(e.to_string()))?;

        Ok(ClientSecret::new(secret_string))
    }

    pub fn gitlab_client_secret(&self) -> joinerror::Result<ClientSecret> {
        let bytes = self.keyring.get_secret(GITLAB_SECRET_PREFIX)?;
        let secret_string =
            String::from_utf8(bytes).map_err(|e| joinerror::Error::new::<()>(e.to_string()))?;

        Ok(ClientSecret::new(secret_string))
    }
}
