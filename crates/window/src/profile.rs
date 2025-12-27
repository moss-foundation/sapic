mod registry;

use crate::profile::registry::ProfileRegistryItem;
use moss_common::collections::{nonempty_hashmap::NonEmptyHashMap, nonempty_vec::NonEmptyVec};
use moss_fs::{CreateOptions, FileSystem};
use moss_keyring::KeyringClient;
use sapic_base::user::types::primitives::{AccountKind, ProfileId, SessionKind};
use sapic_core::context::AnyAsyncContext;
use sapic_system::{
    ports::server_api::{
        ServerApiClient,
        auth_github_account_api::GitHubRevokeApiReq,
        auth_gitlab_account_api::{GitLabRevokeApiReq, GitLabTokenRefreshApiReq},
    },
    user::{
        account::{Account, session::AccountSession},
        profile::Profile,
    },
};
use std::{cell::LazyCell, collections::HashMap, path::Path, sync::Arc};
use tokio::sync::RwLock;

pub(crate) const PROFILES_REGISTRY_FILE: &str = "profiles.json";

const DEFAULT_PROFILE: LazyCell<ProfileRegistryItem> = LazyCell::new(|| ProfileRegistryItem {
    id: ProfileId::new(),
    name: "Default".to_string(),
    accounts: vec![],
    is_default: Some(true),
});

struct ServiceCache {
    profiles: NonEmptyHashMap<ProfileId, ProfileRegistryItem>,
}

pub(crate) struct ProfileService {
    server_api_client: Arc<dyn ServerApiClient>,
    keyring: Arc<dyn KeyringClient>,
    active_profile: RwLock<Option<Arc<Profile>>>,
    cache: RwLock<ServiceCache>,
}

impl ProfileService {
    pub async fn new(
        ctx: &dyn AnyAsyncContext,
        dir_abs: &Path,
        fs: Arc<dyn FileSystem>,
        server_api_client: Arc<dyn ServerApiClient>,
        keyring: Arc<dyn KeyringClient>,
    ) -> joinerror::Result<Self> {
        let profiles = load_or_init_profiles(ctx, fs.as_ref(), dir_abs).await?;

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
            server_api_client,
            keyring,
            active_profile: RwLock::new(None),
            cache: RwLock::new(ServiceCache { profiles: profiles }),
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
                    server_api_client.clone() as Arc<dyn GitHubRevokeApiReq>,
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
                    server_api_client.clone() as Arc<dyn GitLabTokenRefreshApiReq>,
                    server_api_client.clone() as Arc<dyn GitLabRevokeApiReq>,
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
    ctx: &dyn AnyAsyncContext,
    fs: &dyn FileSystem,
    dir_abs: &Path,
) -> joinerror::Result<NonEmptyVec<ProfileRegistryItem>> {
    // If the profile registry file was not found, then we create this file by adding the default profile to it.
    if !dir_abs.join(PROFILES_REGISTRY_FILE).exists() {
        return create_default_profile(ctx, fs, dir_abs).await;
    }

    let rdr = fs
        .open_file(ctx, &dir_abs.join(PROFILES_REGISTRY_FILE))
        .await?;
    let profiles: Vec<ProfileRegistryItem> = serde_json::from_reader(rdr)?;

    match NonEmptyVec::from_vec_option(profiles) {
        Some(non_empty_profiles) => Ok(non_empty_profiles),
        None => {
            // If the profile registry file is empty, create with default profile
            create_default_profile(ctx, fs, dir_abs).await
        }
    }
}

async fn create_default_profile(
    ctx: &dyn AnyAsyncContext,
    fs: &dyn FileSystem,
    dir_abs: &Path,
) -> joinerror::Result<NonEmptyVec<ProfileRegistryItem>> {
    let default_profiles = NonEmptyVec::new(DEFAULT_PROFILE.clone());
    let content = serde_json::to_string_pretty(&default_profiles.clone().into_vec())?;
    fs.create_file_with(
        ctx,
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
