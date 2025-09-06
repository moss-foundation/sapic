use joinerror::Error;
use moss_app_delegate::AppDelegate;
use moss_applib::{AppRuntime, errors::Internal};
use moss_common::{continue_if_err, continue_if_none};
use moss_fs::{CreateOptions, FileSystem};
use moss_git_hosting_provider::{
    GitAuthAdapter,
    github::{GitHubApiClient, GitHubAuthAdapter},
    gitlab::{GitLabApiClient, GitLabAuthAdapter},
};
use moss_keyring::KeyringClient;
use moss_logging::session;
use moss_server_api::account_auth_gateway::AccountAuthGatewayApiClient;
use moss_user::{
    AccountSession,
    account::{Account, github::GitHubInitialToken, gitlab::GitLabInitialToken},
    models::primitives::AccountId,
    profile::ActiveProfile,
};
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
}

impl ServiceConfig {
    pub fn new(profiles_dir_abs: PathBuf) -> joinerror::Result<Self> {
        debug_assert!(profiles_dir_abs.is_absolute());

        if !profiles_dir_abs.exists() {
            return Err(joinerror::Error::new::<Internal>(format!(
                "profiles directory does not exist: {}",
                profiles_dir_abs.display()
            )));
        }

        Ok(Self { profiles_dir_abs })
    }
}

pub(crate) struct ProfileService<R: AppRuntime> {
    fs: Arc<dyn FileSystem>,
    auth_api_client: Arc<AccountAuthGatewayApiClient>,
    keyring: Arc<dyn KeyringClient>,
    state: RwLock<ServiceState>,
    active_profile: Arc<ActiveProfile<R>>,
    config: ServiceConfig,
}

impl<R: AppRuntime> ProfileService<R> {
    pub async fn new(
        fs: Arc<dyn FileSystem>,
        auth_api_client: Arc<AccountAuthGatewayApiClient>,
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
                        keyring.clone(),
                        None,
                    )
                    .await?
                }
                AccountKind::GitLab => {
                    AccountSession::gitlab(
                        account.id.clone(),
                        account.host.clone(),
                        keyring.clone(),
                        auth_api_client.clone(),
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
            auth_api_client,
            keyring,
            state: RwLock::new(ServiceState { profiles }),
            config,
            active_profile: Arc::new(active_profile),
        })
    }

    pub fn active_profile(&self) -> Arc<ActiveProfile<R>> {
        self.active_profile.clone()
    }

    pub async fn add_account(
        &self,
        ctx: &R::AsyncContext,
        app_delegate: &AppDelegate<R>,
        profile_id: ProfileId,
        host: String,
        provider: AccountKind,
    ) -> joinerror::Result<AccountId> {
        // TODO: Check if the account already exists

        let account_id = AccountId::new();
        let (session, username) = match provider {
            AccountKind::GitHub => {
                let auth_client = app_delegate.global::<GitHubAuthAdapter<R>>();
                let api_client = app_delegate.global::<GitHubApiClient>().clone();

                let session = self
                    .add_github_account(ctx, auth_client, account_id.clone(), &host)
                    .await?;
                let user = api_client.get_user::<R>(ctx, &session).await?;

                (session, user.login)
            }
            AccountKind::GitLab => {
                let auth_client = app_delegate.global::<GitLabAuthAdapter<R>>();
                let api_client = app_delegate.global::<GitLabApiClient>().clone();

                let session = self
                    .add_gitlab_account(ctx, auth_client, account_id.clone(), &host)
                    .await?;
                let user = api_client.get_user::<R>(ctx, &session).await?;

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
                    serde_json::to_string_pretty(&parsed)?.as_bytes(),
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
        ctx: &R::AsyncContext,
        auth_client: &GitHubAuthAdapter<R>,
        account_id: AccountId,
        host: &str,
    ) -> joinerror::Result<AccountSession<R>> {
        let token = auth_client.auth_with_pkce(ctx).await.unwrap();

        Ok(AccountSession::github(
            account_id,
            host.to_string(),
            self.keyring.clone(),
            Some(GitHubInitialToken {
                access_token: token.access_token,
            }),
        )
        .await?)
    }

    async fn add_gitlab_account(
        &self,
        ctx: &R::AsyncContext,
        auth_client: &GitLabAuthAdapter<R>,
        account_id: AccountId,
        host: &str,
    ) -> joinerror::Result<AccountSession<R>> {
        let token = auth_client.auth_with_pkce(ctx).await.unwrap();

        Ok(AccountSession::gitlab(
            account_id,
            host.to_string(),
            self.keyring.clone(),
            self.auth_api_client.clone(),
            Some(GitLabInitialToken {
                access_token: token.access_token,
                refresh_token: token.refresh_token,
                expires_in: token.expires_in,
            }),
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
                serde_json::to_string_pretty(&ProfileFile {
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

        let path = entry.path();
        let parsed = continue_if_err!(
            async {
                let rdr = fs.open_file(&path).await?;
                let parsed: ProfileFile = serde_json::from_reader(rdr)?;
                Ok(parsed)
            },
            |e: Error| {
                session::warn!("failed to parse profile file: {}", e.to_string());
            }
        );
        let id: ProfileId = continue_if_none!(path.file_stem().and_then(|s| s.to_str()), || {
            session::warn!("invalid profile filename: {}", path.display().to_string());
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
