use joinerror::OptionExt;
use moss_applib::errors::Internal;
use moss_asp::AppSecretsProvider;
use moss_common::continue_if_none;
use moss_fs::{CreateOptions, FileSystem};
use moss_git::GitSignInAdapter;
use moss_git_hosting_provider::{
    github::{GitHubApiClient, GithubSignInProvider},
    gitlab::{GitLabApiClient, GitLabSignInProvider},
    models::primitives::GitProviderType,
};
use moss_keyring::KeyringClient;
use moss_user::{AccountSession, models::primitives::AccountId};
use oauth2::ClientId;
use reqwest::Client as HttpClient;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::RwLock;

use crate::{
    config::AppConfig,
    dirs::PROFILES_DIR,
    models::{primitives::ProfileId, types::AccountInfo},
};

#[derive(Debug, Serialize, Deserialize)]
struct ProfileFile {
    name: String,
    accounts: Vec<AccountInfo>,
}

struct AccountItem {
    id: AccountId,
    username: String,
    session: AccountSession,
}

struct ProfileItem {
    id: ProfileId,
    accounts: HashMap<AccountId, AccountItem>,
}

struct ServiceState {
    profiles: HashMap<ProfileId, ProfileItem>,
}

pub(crate) struct ServiceConfig {
    profiles_dir_abs: PathBuf,
    github_client_id: ClientId,
    gitlab_client_id: ClientId,
}

impl ServiceConfig {
    pub fn new(
        profiles_dir_abs: PathBuf,
        github_client_id: String,
        gitlab_client_id: String,
    ) -> joinerror::Result<Self> {
        debug_assert!(profiles_dir_abs.is_absolute());

        if !profiles_dir_abs.exists() {
            return Err(joinerror::Error::new::<Internal>(format!(
                "profiles directory does not exist: {}",
                profiles_dir_abs.display()
            )));
        }

        Ok(Self {
            profiles_dir_abs,
            github_client_id: ClientId::new(github_client_id),
            gitlab_client_id: ClientId::new(gitlab_client_id),
        })
    }
}

pub(crate) struct ProfileService {
    fs: Arc<dyn FileSystem>,
    secrets: AppSecretsProvider,
    keyring: Arc<dyn KeyringClient>,
    state: RwLock<ServiceState>,
    config: ServiceConfig,
}

impl ProfileService {
    pub async fn new(
        fs: Arc<dyn FileSystem>,
        secrets: AppSecretsProvider,
        keyring: Arc<dyn KeyringClient>,
        config: ServiceConfig,
    ) -> joinerror::Result<Self> {
        let profiles = scan(&fs, keyring.clone(), secrets.clone(), &config).await?;

        Ok(Self {
            fs,
            secrets,
            keyring,
            state: RwLock::new(ServiceState { profiles }),
            config,
        })
    }

    pub async fn add_account(
        &self,
        profile_id: ProfileId,
        host: String,
        provider: GitProviderType,
    ) -> joinerror::Result<AccountId> {
        // TODO: Check if the account already exists

        let account_id = AccountId::new();
        let (session, username) = match provider {
            GitProviderType::GitHub => {
                let session = self.add_github_account(account_id.clone(), &host).await?;
                let api_client = GitHubApiClient::new(HttpClient::new());
                let user = api_client.user(&session).await?;

                (session, user.login)
            }
            GitProviderType::GitLab => {
                let session = self.add_gitlab_account(account_id.clone(), &host).await?;
                let api_client = GitLabApiClient::new(HttpClient::new());
                let user = api_client.user(&session).await?;

                (session, user.username)
            }
        };

        let account = AccountInfo {
            id: account_id.clone(),
            username: username.clone(),
            host,
            provider,
        };

        let mut state_lock = self.state.write().await;
        let profile = state_lock.profiles.get_mut(&profile_id).unwrap();

        let abs_path = self
            .config
            .profiles_dir_abs
            .join(format!("{}.json", profile_id));
        let rdr = self.fs.open_file(&abs_path).await?;
        let mut parsed: ProfileFile = serde_json::from_reader(rdr)?;
        parsed.accounts.push(account.clone());
        self.fs
            .create_file_with(
                &abs_path,
                serde_json::to_string(&parsed)?.as_bytes(),
                CreateOptions {
                    overwrite: true,
                    ignore_if_exists: false,
                },
            )
            .await?;

        profile.accounts.insert(
            account_id.clone(),
            AccountItem {
                id: account_id.clone(),
                username,
                session,
            },
        );

        Ok(account_id)
    }

    async fn add_github_account(
        &self,
        account_id: AccountId,
        host: &str,
    ) -> joinerror::Result<AccountSession> {
        let client_id = self.config.github_client_id.clone();
        let client_secret = self.secrets.github_client_secret()?;
        let github_client = GithubSignInProvider::new();
        let token = github_client
            .sign_in_with_pkce(client_id, client_secret, host)
            .await
            .unwrap();

        Ok(AccountSession::github(
            account_id,
            host.to_string(),
            self.secrets.clone(),
            self.keyring.clone(),
            Some(token),
        )?)
    }

    async fn add_gitlab_account(
        &self,
        account_id: AccountId,
        host: &str,
    ) -> joinerror::Result<AccountSession> {
        let client_id = self.config.gitlab_client_id.clone();
        let client_secret = self.secrets.gitlab_client_secret()?;
        let gitlab_client = GitLabSignInProvider::new();
        let token = gitlab_client
            .sign_in_with_pkce(client_id.clone(), client_secret, host)
            .await
            .unwrap();

        Ok(AccountSession::gitlab(
            account_id,
            client_id,
            host.to_string(),
            self.keyring.clone(),
            self.secrets.clone(),
            Some(token),
        )?)
    }

    pub async fn create_profile(&self, name: String) -> joinerror::Result<ProfileId> {
        let id = ProfileId::new();
        let profile = ProfileItem {
            id: id.clone(),
            accounts: HashMap::new(),
        };

        let abs_path = self.config.profiles_dir_abs.join(format!("{}.json", id));
        self.fs
            .create_file_with(
                &abs_path,
                serde_json::to_string(&ProfileFile {
                    name,
                    accounts: vec![],
                })?
                .as_bytes(),
                CreateOptions {
                    overwrite: true,
                    ignore_if_exists: false,
                },
            )
            .await?;

        self.state
            .write()
            .await
            .profiles
            .insert(id.clone(), profile);

        Ok(id)
    }
}

async fn scan(
    fs: &Arc<dyn FileSystem>,
    keyring: Arc<dyn KeyringClient>,
    secrets: AppSecretsProvider,
    config: &ServiceConfig,
) -> joinerror::Result<HashMap<ProfileId, ProfileItem>> {
    let mut profiles = HashMap::new();

    let mut read_dir = fs.read_dir(&config.profiles_dir_abs).await?;
    while let Some(entry) = read_dir.next_entry().await? {
        if entry.file_type().await?.is_dir() {
            continue;
        }

        let rdr = fs.open_file(&entry.path()).await?;
        let parsed: ProfileFile = serde_json::from_reader(rdr)?;
        let id: ProfileId =
            continue_if_none!(entry.path().file_stem().and_then(|s| s.to_str()), || {
                // TODO: Log the error
                println!("invalid profile filename: {}", entry.path().display());
            })
            .to_string()
            .into();

        let mut accounts = HashMap::with_capacity(parsed.accounts.len());
        for account in parsed.accounts {
            let session = match account.provider {
                GitProviderType::GitHub => AccountSession::github(
                    account.id.clone(),
                    account.host,
                    secrets.clone(),
                    keyring.clone(),
                    None,
                )?,
                GitProviderType::GitLab => AccountSession::gitlab(
                    account.id.clone(),
                    config.gitlab_client_id.clone(),
                    account.host,
                    keyring.clone(),
                    secrets.clone(),
                    None,
                )?,
            };

            accounts.insert(
                account.id.clone(),
                AccountItem {
                    id: account.id,
                    username: account.username,
                    session,
                },
            );
        }

        profiles.insert(
            id.clone(),
            ProfileItem {
                id: id.clone(),
                accounts,
            },
        );
    }

    Ok(profiles)
}

#[cfg(test)]
mod tests {
    use git2::{Cred, RemoteCallbacks};
    use moss_git::{GitSignInAdapter, repository::Repository};
    use moss_git_hosting_provider::github::GithubSignInProvider;
    use oauth2::{ClientSecret, TokenResponse};

    use super::*;

    #[tokio::test]
    async fn test_clone() {
        let user = "g10z3r";
        let cid = dotenv::var("GITHUB_CLIENT_ID").unwrap().to_string();
        let csecret = dotenv::var("GITHUB_CLIENT_SECRET").unwrap().to_string();
        let gh = GithubSignInProvider::new();
        let tok = gh
            .sign_in_with_pkce(ClientId::new(cid), ClientSecret::new(csecret), "github.com")
            .await
            .unwrap();

        let mut cb = RemoteCallbacks::new();
        cb.credentials(move |_url, username_from_url, _allowed| {
            // let rt = tokio::runtime::Handle::try_current();
            // let fut = self.session_for_remote(ws, repo_root, remote_name);
            // let (acc, tok) = match rt {
            //     Ok(h) => h.block_on(fut),
            //     Err(_) => tokio::runtime::Runtime::new().unwrap().block_on(fut),
            // }
            // .map_err(|e| git2::Error::from_str(&format!("auth error: {e}")))?;
            // let user = username_from_url.unwrap_or(&acc.login);

            Cred::userpass_plaintext(
                username_from_url.unwrap_or(&user),
                &tok.access_token().secret().to_string(),
            )
        });

        let repo = Repository::clone(
            "https://github.com/moss-foundation/sapic-server",
            &Path::new("sapic-server"),
            cb,
        )
        .unwrap();
    }
}
