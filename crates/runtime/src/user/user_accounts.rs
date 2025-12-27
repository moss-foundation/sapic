use joinerror::{OptionExt, ResultExt};
use moss_fs::{CreateOptions, FileSystem};
use moss_keyring::KeyringClient;
use sapic_base::{
    errors::AlreadyExists,
    user::{
        manifest::{AccountsManifestItem, UserAccountsManifest},
        types::{
            AccountInfo,
            primitives::{AccountId, AccountKind, SessionKind},
        },
    },
};
use sapic_core::context::AnyAsyncContext;
use sapic_system::{
    ports::{
        github_api::{GitHubApiClient, GitHubAuthAdapter},
        gitlab_api::{GitLabApiClient, GitLabAuthAdapter},
        server_api::{
            ServerApiClient,
            auth_github_account_api::GitHubRevokeApiReq,
            auth_gitlab_account_api::{GitLabRevokeApiReq, GitLabTokenRefreshApiReq},
        },
    },
    user::{
        AddAccountParams, UpdateAccountParams,
        account::{
            Account,
            github_session::{GitHubInitialToken, GitHubPAT},
            gitlab_session::{GitLabInitialToken, GitLabPAT},
            session::AccountSession,
        },
    },
};
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

const ACCOUNTS_FILE: &str = "accounts.json";

pub struct UserAccountsService {
    // Path to the user directory: .sapic/user/
    abs_path: PathBuf,
    fs: Arc<dyn FileSystem>,

    server_api_client: Arc<dyn ServerApiClient>,
    github_api_client: Arc<dyn GitHubApiClient>,
    gitlab_api_client: Arc<dyn GitLabApiClient>,
    github_auth_adapter: Arc<dyn GitHubAuthAdapter>,
    gitlab_auth_adapter: Arc<dyn GitLabAuthAdapter>,
    keyring: Arc<dyn KeyringClient>,

    accounts: RwLock<HashMap<AccountId, Account>>,
}

impl UserAccountsService {
    pub async fn new(
        ctx: &dyn AnyAsyncContext,
        abs_path: PathBuf,
        fs: Arc<dyn FileSystem>,
        server_api_client: Arc<dyn ServerApiClient>,
        github_api_client: Arc<dyn GitHubApiClient>,
        gitlab_api_client: Arc<dyn GitLabApiClient>,
        github_auth_adapter: Arc<dyn GitHubAuthAdapter>,
        gitlab_auth_adapter: Arc<dyn GitLabAuthAdapter>,
        keyring: Arc<dyn KeyringClient>,
    ) -> joinerror::Result<Self> {
        let accounts = load_or_init_accounts(
            ctx,
            server_api_client.clone(),
            keyring.clone(),
            fs.as_ref(),
            &abs_path,
        )
        .await?;

        Ok(Self {
            abs_path,
            fs,
            server_api_client,
            github_api_client,
            gitlab_api_client,
            github_auth_adapter,
            gitlab_auth_adapter,
            keyring,
            accounts: RwLock::new(accounts),
        })
    }

    pub async fn account(&self, account_id: &AccountId) -> Option<Account> {
        let accounts = self.accounts.read().await;
        accounts.get(account_id).cloned()
    }

    pub async fn accounts(&self) -> Vec<AccountInfo> {
        let accounts = self.accounts.read().await;
        accounts.values().map(|account| account.info()).collect()
    }

    pub async fn remove_account(
        &self,
        ctx: &dyn AnyAsyncContext,
        account_id: &AccountId,
    ) -> joinerror::Result<()> {
        let mut accounts: Vec<AccountInfo> = self
            .accounts
            .read()
            .await
            .values()
            .map(|account| account.info())
            .collect();

        accounts.retain(|account| account.id != *account_id);

        self.sync_accounts_file(ctx, accounts)
            .await
            .join_err_with::<()>(|| {
                format!(
                    "failed to sync accounts file: {}",
                    self.abs_path.join(ACCOUNTS_FILE).display()
                )
            })?;

        let account = self.accounts.write().await.remove(account_id);
        if let Some(account) = account {
            // In this case, the error isn't critical. Since we removed the account from
            // the profile file, the next time a session for that account won't be established.
            if let Err(err) = account.revoke(ctx).await {
                tracing::warn!("failed to revoke account `{}`: {}", account_id, err);
            }
        }

        Ok(())
    }

    pub async fn add_account(
        &self,
        ctx: &dyn AnyAsyncContext,
        params: AddAccountParams,
    ) -> joinerror::Result<AccountId> {
        let account_id = AccountId::new();
        let (session, _, username, expires_at) = match params.kind {
            AccountKind::GitHub => {
                let (session, session_kind) = if let Some(pat) = params.pat {
                    (
                        AccountSession::github_pat(
                            account_id.clone(),
                            params.host.to_string(),
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
                            params.host.to_string(),
                            self.server_api_client.clone() as Arc<dyn GitHubRevokeApiReq>,
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
                let (session, session_kind) = if let Some(pat) = params.pat {
                    (
                        AccountSession::gitlab_pat(
                            account_id.clone(),
                            params.host.to_string(),
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
                            params.host.to_string(),
                            self.server_api_client.clone() as Arc<dyn GitLabTokenRefreshApiReq>,
                            self.server_api_client.clone() as Arc<dyn GitLabRevokeApiReq>,
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

        {
            let accounts = self.accounts.read().await;
            for account in accounts.values() {
                if account.username() == username
                    && account.kind() == params.kind
                    && account.host() == params.host
                {
                    return Err(joinerror::Error::new::<AlreadyExists>(
                        "account already exists",
                    ));
                }
            }
        }

        self.accounts.write().await.insert(
            account_id.clone(),
            Account::new(
                account_id.clone(),
                username.clone(),
                params.host.clone(),
                session,
                params.kind.clone(),
                expires_at,
            ),
        );

        let accounts = self.accounts().await;
        self.sync_accounts_file(ctx, accounts)
            .await
            .join_err_with::<()>(|| {
                format!(
                    "failed to sync accounts file: {}",
                    self.abs_path.join(ACCOUNTS_FILE).display()
                )
            })?;

        Ok(account_id)
    }

    pub async fn update_account(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &AccountId,
        params: UpdateAccountParams,
    ) -> joinerror::Result<()> {
        let accounts_lock = self.accounts.read().await;
        let account = accounts_lock
            .get(id)
            .ok_or_join_err_with::<()>(|| format!("account `{}` not found", id))?;

        if let Some(ref pat) = params.pat {
            let old_pat = account.update_pat(ctx, pat).await?;
            let user_response = self
                .github_api_client
                .get_user(ctx, account.session())
                .await;

            if user_response.is_err() {
                account.update_pat(ctx, &old_pat).await?;
                return Err(joinerror::Error::new::<()>(format!(
                    "failed to authenticate the user after updating the PAT: {}",
                    user_response.unwrap_err()
                )));
            }

            if user_response.unwrap().login != account.username() {
                account.update_pat(ctx, &old_pat).await?;
                return Err(joinerror::Error::new::<()>(
                    "the new PAT does not belong to the same account as the old PAT",
                ))?;
            }
        }

        Ok(())
    }

    async fn sync_accounts_file(
        &self,
        ctx: &dyn AnyAsyncContext,
        accounts: Vec<AccountInfo>,
    ) -> joinerror::Result<()> {
        let accounts: Vec<AccountsManifestItem> =
            accounts.into_iter().map(|account| account.into()).collect();
        let content = serde_json::to_string_pretty(&UserAccountsManifest(accounts))?;

        self.fs
            .create_file_with(
                ctx,
                &self.abs_path.join(ACCOUNTS_FILE),
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

async fn load_or_init_accounts(
    ctx: &dyn AnyAsyncContext,
    server_api_client: Arc<dyn ServerApiClient>,
    keyring: Arc<dyn KeyringClient>,
    fs: &dyn FileSystem,
    abs_path: &PathBuf,
) -> joinerror::Result<HashMap<AccountId, Account>> {
    let accounts_path = abs_path.join(ACCOUNTS_FILE);
    if !accounts_path.exists() {
        return Ok(HashMap::new());
    }

    let rdr = fs.open_file(ctx, &accounts_path).await?;
    let accounts: UserAccountsManifest =
        serde_json::from_reader(rdr).join_err_with::<()>(|| {
            format!(
                "failed to parse accounts manifest file: {}",
                accounts_path.display()
            )
        })?;

    let mut result = HashMap::new();
    for account in accounts.0 {
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

        result.insert(
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

    Ok(result)
}
