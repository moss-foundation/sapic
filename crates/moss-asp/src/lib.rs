use joinerror::Error;
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
    pub async fn new(
        github_client_secret: String,
        gitlab_client_secret: String,
        keyring: Arc<dyn KeyringClient>,
    ) -> joinerror::Result<Self> {
        keyring
            .set_secret(GITHUB_SECRET_PREFIX, &github_client_secret)
            .await?;
        keyring
            .set_secret(GITLAB_SECRET_PREFIX, &gitlab_client_secret)
            .await?;

        Ok(Self { keyring })
    }

    pub async fn github_client_secret(&self) -> joinerror::Result<ClientSecret> {
        let bytes = self.keyring.get_secret(GITHUB_SECRET_PREFIX).await?;
        let utf8_string = String::from_utf8(bytes).map_err(|e| Error::new::<()>(e.to_string()))?;

        Ok(ClientSecret::new(utf8_string))
    }

    pub async fn gitlab_client_secret(&self) -> joinerror::Result<ClientSecret> {
        let bytes = self.keyring.get_secret(GITLAB_SECRET_PREFIX).await?;
        let secret_string =
            String::from_utf8(bytes).map_err(|e| Error::new::<()>(e.to_string()))?;

        Ok(ClientSecret::new(secret_string))
    }
}
