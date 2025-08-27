use moss_applib::errors::Internal;
use moss_common::continue_if_none;
use moss_fs::{CreateOptions, FileSystem};
use moss_git::AuthProvider;
use moss_git_hosting_provider::{github::GithubAuthProvider, models::primitives::GitProviderType};
use moss_keyring::KeyringClient;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::RwLock;

use crate::models::{
    primitives::{AccountId, ProfileId},
    types::AccountInfo,
};

#[derive(Debug, Serialize, Deserialize)]
struct ProfileFile {
    name: String,
    accounts: HashMap<AccountId, AccountInfo>,
}

struct ProfileItem {
    id: ProfileId,
    accounts: HashMap<AccountId, AccountInfo>,
}

struct ServiceState {
    profiles: HashMap<ProfileId, ProfileItem>,
}

pub(crate) struct ProfileService {
    profiles_dir: PathBuf,
    fs: Arc<dyn FileSystem>,
    keyring_client: Arc<dyn KeyringClient>,
    state: RwLock<ServiceState>,
}

impl ProfileService {
    pub async fn new(
        fs: Arc<dyn FileSystem>,
        keyring_client: Arc<dyn KeyringClient>,
        profiles_dir: PathBuf,
    ) -> joinerror::Result<Self> {
        let profiles = scan(&fs, &profiles_dir).await?;

        Ok(Self {
            profiles_dir,
            fs,
            keyring_client,
            state: RwLock::new(ServiceState { profiles }),
        })
    }

    pub async fn add_account(
        &self,
        profile_id: ProfileId,
        host: String,
        label: Option<String>,
        provider: GitProviderType,
    ) -> joinerror::Result<AccountId> {
        // FIXME: Temporary, should be moved to the builder
        dotenv::dotenv().ok();
        let github_client_id = dotenv::var("GITHUB_CLIENT_ID").unwrap_or_default();
        let github_client_secret = dotenv::var("GITHUB_CLIENT_SECRET").unwrap_or_default();
        let gitlab_client_id = dotenv::var("GITLAB_CLIENT_ID").unwrap_or_default();
        let gitlab_client_secret = dotenv::var("GITLAB_CLIENT_SECRET").unwrap_or_default();

        // FIXME: Temporary, should be moved to the builder
        let gh = GithubAuthProvider::new();
        let tok = gh
            .login_pkce(
                &github_client_id,
                Some(&github_client_secret),
                &host,
                &["repo", "user:email", "read:user"],
            )
            .await
            .unwrap();

        // TODO: Fetch account info to get username

        let account_id = AccountId::new();
        let account = AccountInfo {
            id: account_id.clone(),
            username: "HARDCODED".to_string(), // FIXME: Hardcoded for now
            label,
            host: None, // TODO: Hardcoded for now
            provider,
        };

        let key = format!("gh:{}:{}", host, account_id);
        self.keyring_client
            .set_secret(&key, &tok.refresh_token.unwrap_or(tok.token))
            .map_err(|e| joinerror::Error::new::<Internal>(e.to_string()))?;

        let mut state_lock = self.state.write().await;
        let profile = state_lock.profiles.get_mut(&profile_id).unwrap();

        let abs_path = self.profiles_dir.join(format!("{}.json", profile_id));
        let rdr = self.fs.open_file(&abs_path).await?;
        let mut parsed: ProfileFile = serde_json::from_reader(rdr)?;
        parsed.accounts.insert(account_id.clone(), account.clone());
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

        profile.accounts.insert(account_id.clone(), account);

        Ok(account_id)
    }

    pub async fn create_profile(&self, name: String) -> joinerror::Result<ProfileId> {
        let id = ProfileId::new();
        let profile = ProfileItem {
            id: id.clone(),
            accounts: HashMap::new(),
        };

        let abs_path = self.profiles_dir.join(format!("{}.json", id));
        self.fs
            .create_file_with(
                &abs_path,
                serde_json::to_string(&ProfileFile {
                    name,
                    accounts: profile.accounts.clone(),
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
    abs_path: &Path,
) -> joinerror::Result<HashMap<ProfileId, ProfileItem>> {
    debug_assert!(abs_path.is_absolute());

    if !abs_path.exists() {
        return Err(joinerror::Error::new::<Internal>(format!(
            "profiles directory does not exist: {}",
            abs_path.display()
        )));
    }

    let mut profiles = HashMap::new();

    let mut read_dir = fs.read_dir(&abs_path).await?;
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

        let profile = ProfileItem {
            id: id.clone(),
            accounts: parsed.accounts,
        };

        profiles.insert(id.clone(), profile);
    }

    Ok(profiles)
}

#[cfg(test)]
mod tests {
    use git2::{Cred, RemoteCallbacks};
    use moss_git::{AuthProvider, repository::Repository};
    use moss_git_hosting_provider::github::GithubAuthProvider;

    use super::*;

    #[tokio::test]
    async fn test_clone() {
        let user = "g10z3r";
        let cid = "Ov23liUcOSgOxO9K2wb8";
        let csecret = "d4c37cfe59d95c9c8c9bca0a5c0e3d631cbd48a7";
        let gh = GithubAuthProvider::new();
        let tok = gh
            .login_pkce(
                cid,
                Some(csecret),
                "github.com",
                &["repo", "user:email", "read:user"],
            )
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

            Cred::userpass_plaintext(username_from_url.unwrap_or(&user), &tok.token)
        });

        let repo = Repository::clone(
            "https://github.com/moss-foundation/sapic-server",
            &Path::new("sapic-server"),
            cb,
        )
        .unwrap();
    }
}
