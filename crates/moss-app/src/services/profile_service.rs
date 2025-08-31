use moss_applib::errors::Internal;
use moss_asp::AppSecretsProvider;
use moss_common::continue_if_none;
use moss_fs::{CreateOptions, FileSystem};
use moss_git::GitAuthAdapter;
use moss_git_hosting_provider::{
    github::{GitHubApiClient, GitHubAuthAdapter},
    gitlab::{GitLabApiClient, GitLabAuthAdapter},
};
use moss_keyring::KeyringClient;
use moss_user::{
    AccountSession, account::Account, models::primitives::AccountId, profile::ActiveProfile,
};
use oauth2::ClientId;
use reqwest::Client as HttpClient;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

use crate::models::{
    primitives::{AccountKind, ProfileId},
    types::AccountInfo,
};

#[derive(Debug, Serialize, Deserialize)]
struct ProfileFile {
    name: String,
    accounts: Vec<AccountInfo>,
}

struct AccountItem {
    id: AccountId,
    username: String,
    host: String,
    provider: AccountKind,
}

struct ProfileItem {
    #[allow(unused)]
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
    active_profile: Arc<ActiveProfile>,
    config: ServiceConfig,
}

impl ProfileService {
    pub async fn new(
        fs: Arc<dyn FileSystem>,
        secrets: AppSecretsProvider,
        keyring: Arc<dyn KeyringClient>,
        config: ServiceConfig,
    ) -> joinerror::Result<Self> {
        let profiles = scan(&fs, &config).await?;

        // HACK: Use the first profile as the active profile
        let p = profiles.get(&profiles.keys().next().unwrap()).unwrap();
        let mut accounts = HashMap::new();
        for (account_id, account) in p.accounts.iter() {
            let session = match account.provider {
                AccountKind::GitHub => {
                    AccountSession::github(
                        account.id.clone(),
                        account.host.clone(),
                        secrets.clone(),
                        keyring.clone(),
                        None,
                    )
                    .await?
                }
                AccountKind::GitLab => {
                    AccountSession::gitlab(
                        account.id.clone(),
                        config.gitlab_client_id.clone(),
                        account.host.clone(),
                        keyring.clone(),
                        secrets.clone(),
                        None,
                    )
                    .await?
                }
            };

            accounts.insert(
                account_id.clone(),
                Account::new(
                    account_id.clone(),
                    account.username.clone(),
                    account.host.clone(),
                    session,
                ),
            );
        }
        let active_profile = ActiveProfile::new(accounts);
        Ok(Self {
            fs,
            secrets,
            keyring,
            state: RwLock::new(ServiceState { profiles }),
            config,
            active_profile: Arc::new(active_profile),
        })
    }

    pub fn active_profile(&self) -> Arc<ActiveProfile> {
        self.active_profile.clone()
    }

    pub async fn add_account(
        &self,
        profile_id: ProfileId,
        host: String,
        provider: AccountKind,
    ) -> joinerror::Result<AccountId> {
        // TODO: Check if the account already exists

        let account_id = AccountId::new();
        let (session, username) = match provider {
            AccountKind::GitHub => {
                let session = self.add_github_account(account_id.clone(), &host).await?;
                let api_client = GitHubApiClient::new(HttpClient::new());
                let user = api_client.get_user(&session).await?;

                (session, user.login)
            }
            AccountKind::GitLab => {
                let session = self.add_gitlab_account(account_id.clone(), &host).await?;
                let api_client = GitLabApiClient::new(HttpClient::new());
                let user = api_client.get_user(&session).await?;

                (session, user.username)
            }
        };

        let mut state_lock = self.state.write().await;
        let profile = state_lock.profiles.get_mut(&profile_id).unwrap();

        {
            let account = AccountInfo {
                id: account_id.clone(),
                username: username.clone(),
                host: host.clone(),
                provider: provider.clone(),
            };
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
        }

        self.active_profile
            .add_account(Account::new(
                account_id.clone(),
                username.clone(),
                host.clone(),
                session,
            ))
            .await;

        profile.accounts.insert(
            account_id.clone(),
            AccountItem {
                id: account_id.clone(),
                username,
                provider,
                host,
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
        let client_secret = self.secrets.github_client_secret().await?;
        let github_client = GitHubAuthAdapter::new();
        let token = github_client
            .auth_with_pkce(client_id, client_secret, host)
            .await
            .unwrap();

        Ok(AccountSession::github(
            account_id,
            host.to_string(),
            self.secrets.clone(),
            self.keyring.clone(),
            Some(token),
        )
        .await?)
    }

    async fn add_gitlab_account(
        &self,
        account_id: AccountId,
        host: &str,
    ) -> joinerror::Result<AccountSession> {
        let client_id = self.config.gitlab_client_id.clone();
        let client_secret = self.secrets.gitlab_client_secret().await?;
        let gitlab_client = GitLabAuthAdapter::new();
        let token = gitlab_client
            .auth_with_pkce(client_id.clone(), client_secret, host)
            .await
            .unwrap();

        Ok(AccountSession::gitlab(
            account_id,
            client_id,
            host.to_string(),
            self.keyring.clone(),
            self.secrets.clone(),
            Some(token),
        )
        .await?)
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
            accounts.insert(
                account.id.clone(),
                AccountItem {
                    id: account.id,
                    username: account.username,
                    provider: account.provider,
                    host: account.host,
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
