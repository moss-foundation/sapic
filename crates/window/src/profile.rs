mod registry;

use joinerror::{Error, OptionExt, ResultExt};
use moss_app_delegate::AppDelegate;
use moss_applib::{
    AppRuntime,
    errors::{AlreadyExists, FailedPrecondition, NotFound},
};
use moss_common::collections::{nonempty_hashmap::NonEmptyHashMap, nonempty_vec::NonEmptyVec};
use moss_fs::{CreateOptions, FileSystem};
use moss_keyring::KeyringClient;
use moss_logging::session;
use sapic_base::user::types::{
    AccountInfo, AccountMetadata,
    primitives::{AccountId, AccountKind, ProfileId, SessionKind},
};
use sapic_core::context::AnyAsyncContext;
use sapic_system::{
    ports::{
        github_api::{GitHubApiClient, GitHubAuthAdapter},
        gitlab_api::{GitLabApiClient, GitLabAuthAdapter},
        server_api::ServerApiClient,
    },
    user::{
        account::{
            Account,
            github_session::{GitHubInitialToken, GitHubPAT},
            gitlab_session::{GitLabInitialToken, GitLabPAT},
            session::AccountSession,
        },
        profile::Profile,
    },
};
use std::{cell::LazyCell, collections::HashMap, path::Path, sync::Arc};
use tokio::sync::RwLock;

use crate::{
    dirs,
    models::types::UpdateAccountParams,
    profile::registry::{
        ProfileRegistryAccount, ProfileRegistryAccountMetadata, ProfileRegistryItem,
    },
};

pub(crate) const PROFILES_REGISTRY_FILE: &str = "profiles.json";
// pub(crate) const PROFILE_SETTINGS_FILE: &str = "settings.json";

const DEFAULT_PROFILE: LazyCell<ProfileRegistryItem> = LazyCell::new(|| ProfileRegistryItem {
    id: ProfileId::new(),
    name: "Default".to_string(),
    accounts: vec![],
    is_default: Some(true),
});

pub(crate) struct ProfileDetails {
    pub name: String,
    pub accounts: Vec<AccountInfo>,
}

struct ServiceCache {
    profiles: NonEmptyHashMap<ProfileId, ProfileRegistryItem>,
}

pub(crate) struct ProfileService {
    fs: Arc<dyn FileSystem>,
    server_api_client: Arc<dyn ServerApiClient>,
    github_api_client: Arc<dyn GitHubApiClient>,
    gitlab_api_client: Arc<dyn GitLabApiClient>,
    github_auth_adapter: Arc<dyn GitHubAuthAdapter>,
    gitlab_auth_adapter: Arc<dyn GitLabAuthAdapter>,
    keyring: Arc<dyn KeyringClient>,
    active_profile: RwLock<Option<Arc<Profile>>>,
    cache: RwLock<ServiceCache>,
}

impl ProfileService {
    pub async fn new(
        dir_abs: &Path,
        fs: Arc<dyn FileSystem>,
        server_api_client: Arc<dyn ServerApiClient>,
        github_api_client: Arc<dyn GitHubApiClient>,
        gitlab_api_client: Arc<dyn GitLabApiClient>,
        github_auth_adapter: Arc<dyn GitHubAuthAdapter>,
        gitlab_auth_adapter: Arc<dyn GitLabAuthAdapter>,
        keyring: Arc<dyn KeyringClient>,
    ) -> joinerror::Result<Self> {
        let profiles = load_or_init_profiles(fs.as_ref(), dir_abs).await?;

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
            server_api_client,
            github_api_client,
            gitlab_api_client,
            github_auth_adapter,
            gitlab_auth_adapter,
            keyring,
            active_profile: RwLock::new(None),
            cache: RwLock::new(ServiceCache { profiles: profiles }),
        })
    }

    pub async fn profile_details(&self, id: &ProfileId) -> Option<ProfileDetails> {
        let cache_lock = self.cache.read().await;
        let profile = cache_lock.profiles.get(id).cloned();
        profile.map(|p| ProfileDetails {
            name: p.name,
            accounts: p
                .accounts
                .iter()
                .map(|a| AccountInfo {
                    id: a.id.clone(),
                    username: a.username.clone(),
                    host: a.host.clone(),
                    kind: a.kind.clone(),
                    method: a.metadata.session_kind.clone().into(),
                    metadata: AccountMetadata {
                        pat_expires_at: a.metadata.expires_at,
                    },
                })
                .collect(),
        })
    }

    pub async fn activate_profile(&self) -> joinerror::Result<Arc<Profile>> {
        // HACK: since we don't support having multiple profiles yet, we select
        // the first profile in the folder (which is the default one) as the active one.
        let cache_lock = self.cache.read().await;
        let active_profile: Arc<Profile> = activate_profile(
            cache_lock.profiles.first().1,
            self.keyring.clone(),
            self.server_api_client.clone(),
        )
        .await?
        .into();

        let _ = self
            .active_profile
            .write()
            .await
            .insert(active_profile.clone());

        Ok(active_profile)
    }

    pub async fn active_profile(&self) -> Option<Arc<Profile>> {
        self.active_profile.read().await.clone()
    }

    pub async fn remove_account<R: AppRuntime>(
        &self,
        ctx: &dyn AnyAsyncContext,
        app_delegate: &AppDelegate<R>,
        account_id: AccountId,
    ) -> joinerror::Result<AccountId> {
        let active_profile_lock = self.active_profile.write().await;
        let active_profile = active_profile_lock
            .as_ref()
            .ok_or_join_err::<FailedPrecondition>("active profile not found")?;

        let mut cache_lock = self.cache.write().await;
        let profile = cache_lock
            .profiles
            .get_mut(active_profile.id())
            .ok_or_join_err_with::<NotFound>(|| {
                format!("profile `{}` not found", active_profile.id())
            })?;

        profile.accounts.retain(|account| account.id != account_id);

        self.save_profiles_registry(app_delegate, &cache_lock.profiles)
            .await?;

        // In this case, the error isn't critical. Since we removed the account from
        // the profile file, the next time a session for that account won't be established.
        if let Err(err) = active_profile
            .remove_account(ctx, self.server_api_client.clone(), &account_id)
            .await
        {
            session::warn!(&format!(
                "failed to remove account `{}`: {}",
                account_id,
                err.to_string()
            ));
        }

        Ok(account_id)
    }

    pub async fn add_account<R: AppRuntime>(
        &self,
        ctx: &dyn AnyAsyncContext,
        app_delegate: &AppDelegate<R>,
        host: String,
        kind: AccountKind,
        pat: Option<String>,
    ) -> joinerror::Result<AccountId> {
        let active_profile_lock = self.active_profile.write().await;
        let active_profile = active_profile_lock
            .as_ref()
            .ok_or_join_err::<FailedPrecondition>("active profile not found")?;

        let account_id = AccountId::new();
        let (session, session_kind, username, expires_at) = match kind {
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
                    let auth_client = self.github_auth_adapter.clone();
                    let token = auth_client
                        .auth_with_pkce(ctx)
                        .await
                        .join_err::<()>("failed to authenticate with GitHub")?;

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

                let api_client = self.github_api_client.clone();
                let user = api_client.get_user(ctx, &session).await?;

                let expires_at = if session_kind == SessionKind::PAT {
                    api_client.get_pat_expires_at(ctx, &session).await.unwrap()
                } else {
                    None
                };

                (session, session_kind, user.login, expires_at)
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
                    let auth_client = self.gitlab_auth_adapter.clone();
                    let token = auth_client
                        .auth_with_pkce(ctx)
                        .await
                        .join_err::<()>("failed to authenticate with GitLab")?;

                    (
                        AccountSession::gitlab_oauth(
                            account_id.clone(),
                            host.to_string(),
                            self.server_api_client.clone(),
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

                let api_client = self.gitlab_api_client.clone();
                let user = api_client.get_user(ctx, &session).await?;

                let expires_at = if session_kind == SessionKind::PAT {
                    api_client.get_pat_expires_at(ctx, &session).await.unwrap()
                } else {
                    None
                };

                (session, session_kind, user.username, expires_at)
            }
        };

        let mut cache_lock = self.cache.write().await;

        if active_profile
            .is_account_exists(&username, kind.clone(), &host)
            .await
            .is_some()
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
            expires_at,
        );
        let account_info = account.info();

        // Add account to profile registry item
        cache_lock.profiles.get_mut(active_profile.id()).map(|p| {
            p.accounts.push(ProfileRegistryAccount {
                id: account_info.id.clone(),
                username: account_info.username.clone(),
                host: account_info.host.clone(),
                kind: account_info.kind.clone(),
                metadata: ProfileRegistryAccountMetadata {
                    session_kind,
                    expires_at,
                },
            })
        });

        // Update the registry file
        self.save_profiles_registry(app_delegate, &cache_lock.profiles)
            .await?;

        active_profile.add_account(account).await;

        Ok(account_id)
    }

    pub async fn update_account(
        &self,
        ctx: &dyn AnyAsyncContext,
        params: &UpdateAccountParams,
    ) -> joinerror::Result<()> {
        let active_profile_lock = self.active_profile.write().await;
        let active_profile = active_profile_lock
            .as_ref()
            .ok_or_join_err::<FailedPrecondition>("active profile not found")?;
        let account = active_profile
            .account(&params.id)
            .await
            .ok_or_join_err::<()>(format!("Account id `{}` not found", params.id))?;

        if let Some(ref pat) = params.pat {
            let old_pat = account.update_pat(ctx, pat).await?;
            let user_response = self
                .github_api_client
                .get_user(ctx, account.session())
                .await;

            if user_response.is_err() {
                account.update_pat(ctx, &old_pat).await?;
                return Err(Error::new::<()>(format!(
                    "failed to authenticate the user after updating the PAT: {}",
                    user_response.unwrap_err()
                )));
            }

            if user_response.unwrap().login != account.username() {
                account.update_pat(ctx, &old_pat).await?;
                return Err(Error::new::<()>(
                    "the new PAT does not belong to the same account as the old PAT",
                ))?;
            }
        }

        Ok(())
    }

    pub async fn create_profile<R: AppRuntime>(
        &self,
        app_delegate: &AppDelegate<R>,
        name: String,
        _is_default: bool,
    ) -> joinerror::Result<ProfileId> {
        let mut state_lock = self.cache.write().await;

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
    async fn save_profiles_registry<R: AppRuntime>(
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
            .user_dir()
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

async fn activate_profile(
    profile_item: &ProfileRegistryItem,
    keyring: Arc<dyn KeyringClient>,
    server_api_client: Arc<dyn ServerApiClient>,
) -> joinerror::Result<Profile> {
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
                    server_api_client.clone(),
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
                account.metadata.expires_at,
            ),
        );
    }

    let profile = Profile::new(profile_item.id.clone(), accounts);

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
