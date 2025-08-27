pub mod models;
pub mod profile;

#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum ProviderId {
    GitHub,
    GitLab,
}

#[derive(Clone, Debug)]
pub enum TokenType {
    PAT,
    OAuth,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AccountMeta {
    pub id: String,
    pub provider: ProviderId,
    pub host: String,
    pub login: String,
    pub label: String,
}

#[derive(Clone, Debug)]
pub struct AccessToken {
    pub token: String,
    pub token_type: TokenType,
    pub expires_at: Option<std::time::SystemTime>,
    pub refresh_token: Option<String>,
    pub scopes: Vec<String>,
}

#[async_trait::async_trait]
pub trait AuthProvider: Send + Sync {
    fn id(&self) -> ProviderId;
    fn host(&self) -> &str;
    async fn login(&self) -> anyhow::Result<(AccountMeta, AccessToken)>;
    async fn refresh(&self, acc: &AccountMeta, tok: &AccessToken) -> anyhow::Result<AccessToken>;
    async fn revoke(&self, acc: &AccountMeta, tok: &AccessToken) -> anyhow::Result<()>;
}
