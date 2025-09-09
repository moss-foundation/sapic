use joinerror::{Error, OptionExt};
use moss_app_delegate::AppDelegate;
use moss_applib::{
    AppRuntime,
    errors::{AlreadyExists, NotFound},
};
use moss_common::{continue_if_err, continue_if_none};
use moss_fs::{CreateOptions, FileSystem};
use moss_git_hosting_provider::{
    github::{auth::GitHubAuthAdapter, client::GitHubApiClient},
    gitlab::{auth::GitLabAuthAdapter, client::GitLabApiClient},
};
use moss_keyring::KeyringClient;
use moss_logging::session;
use moss_server_api::account_auth_gateway::AccountAuthGatewayApiClient;
use moss_user::{
    AccountSession,
    account::{Account, github::GitHubInitialToken, gitlab::GitLabInitialToken},
    models::{
        primitives::{AccountId, AccountKind, ProfileId},
        types::ProfileInfo,
    },
    profile::Profile,
};

use std::{collections::HashMap, path::Path, sync::Arc};
use tokio::sync::RwLock;

use crate::{dirs, profile::ProfileFile};

struct ServiceState<R: AppRuntime> {
    profiles: HashMap<ProfileId, ProfileInfo>,
    active_profile: Arc<Profile<R>>,
}

pub(crate) struct ProfileService<R: AppRuntime> {
    fs: Arc<dyn FileSystem>,
    auth_api_client: Arc<AccountAuthGatewayApiClient>,
    keyring: Arc<dyn KeyringClient>,
    state: RwLock<ServiceState<R>>,
}

impl<R: AppRuntime> ProfileService<R> {
    pub async fn new(
        dir_abs: &Path,
        fs: Arc<dyn FileSystem>,
        auth_api_client: Arc<AccountAuthGatewayApiClient>,
        keyring: Arc<dyn KeyringClient>,
    ) -> joinerror::Result<Self> {
        dbg!(&dir_abs);

        let profiles = scan(&fs, dir_abs).await?;

        let active_profile = {
            // HACK:
            // Since we don't support having multiple profiles yet, we select
            // the first profile in the folder (which is the default one) as the active one.
            let p = &profiles.values().next().unwrap(); // SAFETY: When we start the app, we must have at least one profile
            let mut accounts = HashMap::with_capacity(p.accounts.len());

            for account in p.accounts.iter() {
                let session = match account.kind {
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
                    account.id.clone(),
                    Account::new(
                        account.id.clone(),
                        account.username.clone(),
                        account.host.clone(),
                        session,
                        account.kind.clone(),
                    ),
                );
            }

            Profile::new(p.id.clone(), accounts)
        };

        Ok(Self {
            fs,
            auth_api_client,
            keyring,
            state: RwLock::new(ServiceState {
                profiles,
                active_profile: Arc::new(active_profile),
            }),
        })
    }

    pub async fn active_profile(&self) -> Arc<Profile<R>> {
        let state_lock = self.state.read().await;
        state_lock.active_profile.clone()
    }

    pub async fn remove_account(
        &self,
        app_delegate: &AppDelegate<R>,
        account_id: AccountId,
    ) -> joinerror::Result<()> {
        let mut state_lock = self.state.write().await;

        let profile_id = state_lock.active_profile.id().clone();
        let profile = state_lock
            .profiles
            .get_mut(&profile_id)
            .ok_or_join_err_with::<NotFound>(|| format!("profile `{}` not found", profile_id))?;

        let mut profile_clone = profile.clone();
        profile_clone
            .accounts
            .retain(|account| account.id != account_id);
        {
            let abs_path = app_delegate
                .app_dir()
                .join(dirs::PROFILES_DIR)
                .join(format!("{}.json", profile_id));

            let content = serde_json::to_string_pretty(&profile_clone)?;
            self.fs
                .create_file_with(
                    &abs_path,
                    content.as_bytes(),
                    CreateOptions {
                        overwrite: true,
                        ignore_if_exists: false,
                    },
                )
                .await?;
        }
        profile.accounts = profile_clone.accounts;

        // In this case, the error isn't critical. Since we removed the account from
        // the profile file, the next time a session for that account won't be established.
        if let Err(err) = state_lock.active_profile.remove_account(&account_id).await {
            session::warn!(&format!(
                "failed to remove account `{}`: {}",
                account_id,
                err.to_string()
            ));
        }

        Ok(())
    }

    pub async fn add_account(
        &self,
        ctx: &R::AsyncContext,
        app_delegate: &AppDelegate<R>,
        host: String,
        kind: AccountKind,
    ) -> joinerror::Result<AccountId> {
        let mut state_lock = self.state.write().await;

        let profile_id = state_lock.active_profile.id().clone();
        let account_id = AccountId::new();
        let (session, username) = match kind {
            AccountKind::GitHub => {
                let auth_client = <dyn GitHubAuthAdapter<R>>::global(app_delegate);
                let api_client = <dyn GitHubApiClient<R>>::global(app_delegate);

                let session = self
                    .add_github_account(ctx, auth_client.as_ref(), account_id.clone(), &host)
                    .await?;
                let user = api_client.get_user(ctx, &session).await?;

                (session, user.login)
            }
            AccountKind::GitLab => {
                let auth_client = <dyn GitLabAuthAdapter<R>>::global(app_delegate);
                let api_client = <dyn GitLabApiClient<R>>::global(app_delegate);

                let session = self
                    .add_gitlab_account(ctx, auth_client.as_ref(), account_id.clone(), &host)
                    .await?;
                let user = api_client.get_user(ctx, &session).await?;

                (session, user.username)
            }
        };

        if let Some(_) = state_lock
            .active_profile
            .is_account_exists(&username, kind.clone(), &host)
            .await
        {
            return Err(joinerror::Error::new::<AlreadyExists>(
                "account already exists",
            ));
        }

        let account = Account::new(
            account_id.clone(),
            username.clone(),
            host.clone(),
            session,
            kind.clone(),
        );
        let account_info = account.info();

        {
            let abs_path = app_delegate
                .app_dir()
                .join(dirs::PROFILES_DIR)
                .join(format!("{}.json", profile_id));
            let rdr = self.fs.open_file(&abs_path).await?;
            let mut parsed: ProfileFile = serde_json::from_reader(rdr)?;
            parsed.accounts.push(account_info.clone());
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

        state_lock
            .profiles
            .get_mut(&profile_id)
            .map(|p| p.accounts.push(account_info.clone()));

        state_lock.active_profile.add_account(account).await;

        Ok(account_id)
    }

    async fn add_github_account(
        &self,
        ctx: &R::AsyncContext,
        auth_client: &dyn GitHubAuthAdapter<R>,
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
        auth_client: &dyn GitLabAuthAdapter<R>,
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

    pub async fn create_profile(
        &self,
        app_delegate: &AppDelegate<R>,
        name: String,
        is_default: bool,
    ) -> joinerror::Result<ProfileId> {
        let id = ProfileId::new();
        let profile = ProfileInfo {
            id: id.clone(),
            name,
            accounts: vec![],
        };

        let abs_path = app_delegate
            .app_dir()
            .join(dirs::PROFILES_DIR)
            .join(format!("{}.json", profile.id));

        self.fs
            .create_file_with(
                &abs_path,
                serde_json::to_string_pretty(&ProfileFile {
                    name: profile.name.clone(),
                    is_default: Some(is_default),
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
            .insert(profile.id.clone(), profile);

        Ok(id)
    }
}

async fn scan(
    fs: &Arc<dyn FileSystem>,
    profiles_dir_abs: &Path,
) -> joinerror::Result<HashMap<ProfileId, ProfileInfo>> {
    debug_assert!(profiles_dir_abs.is_absolute());

    let mut profiles = HashMap::new();

    let mut read_dir = fs.read_dir(&profiles_dir_abs).await?;
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

        profiles.insert(
            id.clone(),
            ProfileInfo {
                id: id.clone(),
                name: parsed.name,
                accounts: parsed.accounts,
            },
        );
    }

    Ok(profiles)
}
