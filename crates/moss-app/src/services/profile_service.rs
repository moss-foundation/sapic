use joinerror::OptionExt;
use moss_app_delegate::AppDelegate;
use moss_applib::{
    AppRuntime,
    errors::{AlreadyExists, NotFound},
    subscription::EventEmitter,
};
use moss_common::collections::{nonempty_hashmap::NonEmptyHashMap, nonempty_vec::NonEmptyVec};
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
    account::{
        Account,
        github::{GitHubInitialToken, GitHubPAT},
        gitlab::{GitLabInitialToken, GitLabPAT},
    },
    models::primitives::{AccountId, AccountKind, ProfileId, SessionKind},
    profile::Profile,
};
use std::{cell::LazyCell, collections::HashMap, path::Path, sync::Arc};
use tokio::sync::RwLock;

use crate::{
    OnDidChangeProfile, dirs,
    profile::{
        PROFILES_REGISTRY_FILE, ProfileRegistryAccount, ProfileRegistryAccountMetadata,
        ProfileRegistryItem,
    },
};

const DEFAULT_PROFILE: LazyCell<ProfileRegistryItem> = LazyCell::new(|| ProfileRegistryItem {
    id: ProfileId::new(),
    name: "Default".to_string(),
    accounts: vec![],
    is_default: Some(true),
});

struct ServiceState<R: AppRuntime> {
    profiles: NonEmptyHashMap<ProfileId, ProfileRegistryItem>,
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

        on_did_change_profile_emitter: EventEmitter<OnDidChangeProfile>,
    ) -> joinerror::Result<Self> {
        let profiles = load_or_init_profiles(fs.as_ref(), dir_abs).await?;

        // HACK: since we don't support having multiple profiles yet, we select
        // the first profile in the folder (which is the default one) as the active one.
        let active_profile = activate_profile(
            profiles.first(),
            keyring.clone(),
            auth_api_client.clone(),
            on_did_change_profile_emitter,
        )
        .await?;

        let profiles = {
            let first_profile_item = profiles.first().clone();
            let mut result =
                NonEmptyHashMap::new(first_profile_item.id.clone(), first_profile_item);

            for item in profiles.tail() {
                result.insert(item.id.clone(), item.clone());
            }

            result
        };

        Ok(Self {
            fs,
            auth_api_client,
            keyring,
            state: RwLock::new(ServiceState {
                profiles: profiles,
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
    ) -> joinerror::Result<AccountId> {
        let mut state_lock = self.state.write().await;

        let profile_id = state_lock.active_profile.id().clone();
        let profile = state_lock
            .profiles
            .get_mut(&profile_id)
            .ok_or_join_err_with::<NotFound>(|| format!("profile `{}` not found", profile_id))?;

        profile.accounts.retain(|account| account.id != account_id);

        self.save_profiles_registry(app_delegate, &state_lock.profiles)
            .await?;

        // In this case, the error isn't critical. Since we removed the account from
        // the profile file, the next time a session for that account won't be established.
        if let Err(err) = state_lock.active_profile.remove_account(&account_id).await {
            session::warn!(&format!(
                "failed to remove account `{}`: {}",
                account_id,
                err.to_string()
            ));
        }

        Ok(account_id)
    }

    pub async fn add_account(
        &self,
        ctx: &R::AsyncContext,
        app_delegate: &AppDelegate<R>,
        host: String,
        kind: AccountKind,
        pat: Option<String>,
    ) -> joinerror::Result<AccountId> {
        let mut state_lock = self.state.write().await;

        let profile_id = state_lock.active_profile.id().clone();
        let account_id = AccountId::new();

        let (session, session_kind, username) = match kind {
            AccountKind::GitHub => {
                let (session, session_kind) = if let Some(pat) = pat {
                    (
                        AccountSession::github_pat(
                            account_id.clone(),
                            host.to_string(),
                            Some(GitHubPAT { token: pat }),
                            self.keyring.clone(),
                        )
                        .await?,
                        SessionKind::PAT,
                    )
                } else {
                    let auth_client = <dyn GitHubAuthAdapter<R>>::global(app_delegate);
                    let token = auth_client.auth_with_pkce(ctx).await.unwrap();

                    (
                        AccountSession::github_oauth(
                            account_id.clone(),
                            host.to_string(),
                            Some(GitHubInitialToken {
                                access_token: token.access_token,
                            }),
                            self.keyring.clone(),
                        )
                        .await?,
                        SessionKind::OAuth,
                    )
                };

                let api_client = <dyn GitHubApiClient<R>>::global(app_delegate);
                let user = api_client.get_user(ctx, &session).await?;

                (session, session_kind, user.login)
            }
            AccountKind::GitLab => {
                let (session, session_kind) = if let Some(pat) = pat {
                    (
                        AccountSession::gitlab_pat(
                            account_id.clone(),
                            host.to_string(),
                            Some(GitLabPAT { token: pat }),
                            self.keyring.clone(),
                        )
                        .await?,
                        SessionKind::PAT,
                    )
                } else {
                    let auth_client = <dyn GitLabAuthAdapter<R>>::global(app_delegate);
                    let token = auth_client.auth_with_pkce(ctx).await?;

                    (
                        AccountSession::gitlab_oauth(
                            account_id.clone(),
                            host.to_string(),
                            self.auth_api_client.clone(),
                            Some(GitLabInitialToken {
                                access_token: token.access_token,
                                refresh_token: token.refresh_token,
                                expires_in: token.expires_in,
                            }),
                            self.keyring.clone(),
                        )
                        .await?,
                        SessionKind::OAuth,
                    )
                };

                let api_client = <dyn GitLabApiClient<R>>::global(app_delegate);
                let user = api_client.get_user(ctx, &session).await?;

                (session, session_kind, user.username)
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

        // Add account to profile registry item
        state_lock.profiles.get_mut(&profile_id).map(|p| {
            p.accounts.push(ProfileRegistryAccount {
                id: account_info.id.clone(),
                username: account_info.username.clone(),
                host: account_info.host.clone(),
                kind: account_info.kind.clone(),
                metadata: ProfileRegistryAccountMetadata { session_kind },
            })
        });

        // Update the registry file
        self.save_profiles_registry(app_delegate, &state_lock.profiles)
            .await?;

        state_lock.active_profile.add_account(account).await;

        Ok(account_id)
    }

    pub async fn create_profile(
        &self,
        app_delegate: &AppDelegate<R>,
        name: String,
        _is_default: bool,
    ) -> joinerror::Result<ProfileId> {
        let mut state_lock = self.state.write().await;

        let id = ProfileId::new();
        let profile = ProfileRegistryItem {
            id: id.clone(),
            name,
            is_default: None,
            accounts: vec![],
        };

        // Add profile to state
        state_lock.profiles.insert(profile.id.clone(), profile);

        // Update the registry file
        self.save_profiles_registry(app_delegate, &state_lock.profiles)
            .await?;

        Ok(id)
    }

    /// Save all profiles to the registry file
    async fn save_profiles_registry(
        &self,
        app_delegate: &AppDelegate<R>,
        profiles: &NonEmptyHashMap<ProfileId, ProfileRegistryItem>,
    ) -> joinerror::Result<()> {
        let registry_items: Vec<ProfileRegistryItem> = profiles
            .iter()
            .map(|(_, profile_item)| profile_item.clone())
            .collect();

        let content = serde_json::to_string_pretty(&registry_items)?;
        let registry_path = app_delegate
            .app_dir()
            .join(dirs::PROFILES_DIR)
            .join(PROFILES_REGISTRY_FILE);

        self.fs
            .create_file_with(
                &registry_path,
                content.as_bytes(),
                CreateOptions {
                    overwrite: true,
                    ignore_if_exists: false,
                },
            )
            .await?;

        Ok(())
    }
}

async fn activate_profile<R: AppRuntime>(
    profile_item: &ProfileRegistryItem,
    keyring: Arc<dyn KeyringClient>,
    auth_api_client: Arc<AccountAuthGatewayApiClient>,

    on_did_change_profile_emitter: EventEmitter<OnDidChangeProfile>,
) -> joinerror::Result<Profile<R>> {
    let mut accounts = HashMap::with_capacity(profile_item.accounts.len());

    for account in profile_item.accounts.iter() {
        let session = match (&account.kind, &account.metadata.session_kind) {
            (AccountKind::GitHub, SessionKind::OAuth) => {
                AccountSession::github_oauth(
                    account.id.clone(),
                    account.host.clone(),
                    None,
                    keyring.clone(),
                )
                .await?
            }
            (AccountKind::GitHub, SessionKind::PAT) => {
                AccountSession::github_pat(
                    account.id.clone(),
                    account.host.clone(),
                    None,
                    keyring.clone(),
                )
                .await?
            }
            (AccountKind::GitLab, SessionKind::OAuth) => {
                AccountSession::gitlab_oauth(
                    account.id.clone(),
                    account.host.clone(),
                    auth_api_client.clone(),
                    None,
                    keyring.clone(),
                )
                .await?
            }
            (AccountKind::GitLab, SessionKind::PAT) => {
                AccountSession::gitlab_pat(
                    account.id.clone(),
                    account.host.clone(),
                    None,
                    keyring.clone(),
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

    let profile = Profile::new(profile_item.id.clone(), accounts);
    on_did_change_profile_emitter
        .fire(OnDidChangeProfile {
            id: profile_item.id.clone(),
        })
        .await;

    Ok(profile)
}

async fn load_or_init_profiles(
    fs: &dyn FileSystem,
    dir_abs: &Path,
) -> joinerror::Result<NonEmptyVec<ProfileRegistryItem>> {
    // If the profile registry file was not found, then we create this file by adding the default profile to it.
    if !dir_abs.join(PROFILES_REGISTRY_FILE).exists() {
        return create_default_profile(fs, dir_abs).await;
    }

    let rdr = fs.open_file(&dir_abs.join(PROFILES_REGISTRY_FILE)).await?;
    let profiles: Vec<ProfileRegistryItem> = serde_json::from_reader(rdr)?;

    match NonEmptyVec::from_vec_option(profiles) {
        Some(non_empty_profiles) => Ok(non_empty_profiles),
        None => {
            // If the profile registry file is empty, create with default profile
            create_default_profile(fs, dir_abs).await
        }
    }
}

async fn create_default_profile(
    fs: &dyn FileSystem,
    dir_abs: &Path,
) -> joinerror::Result<NonEmptyVec<ProfileRegistryItem>> {
    let default_profiles = NonEmptyVec::new(DEFAULT_PROFILE.clone());
    let content = serde_json::to_string_pretty(&default_profiles.clone().into_vec())?;
    fs.create_file_with(
        &dir_abs.join(PROFILES_REGISTRY_FILE),
        content.as_bytes(),
        CreateOptions {
            overwrite: true,
            ignore_if_exists: false,
        },
    )
    .await?;

    return Ok(default_profiles);
}
