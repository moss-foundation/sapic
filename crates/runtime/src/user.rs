pub mod user_accounts;
pub mod user_settings;

pub use user_accounts::{AddAccountParams, UpdateAccountParams};

use moss_fs::FileSystem;
use moss_keyring::KeyringClient;
use sapic_base::user::types::{AccountInfo, primitives::AccountId};
use sapic_core::context::AnyAsyncContext;
use sapic_system::{
    configuration::SettingsStore,
    ports::{
        github_api::{GitHubApiClient, GitHubAuthAdapter},
        gitlab_api::{GitLabApiClient, GitLabAuthAdapter},
        server_api::ServerApiClient,
    },
    user::account::Account,
};
use std::{path::PathBuf, sync::Arc};

use crate::user::{user_accounts::UserAccountsService, user_settings::UserSettingsService};

pub struct User {
    settings: Arc<UserSettingsService>,
    accounts: UserAccountsService,
}

impl User {
    pub async fn new(
        abs_path: PathBuf,
        fs: Arc<dyn FileSystem>,
        server_api_client: Arc<dyn ServerApiClient>,
        github_api_client: Arc<dyn GitHubApiClient>,
        gitlab_api_client: Arc<dyn GitLabApiClient>,
        github_auth_adapter: Arc<dyn GitHubAuthAdapter>,
        gitlab_auth_adapter: Arc<dyn GitLabAuthAdapter>,
        keyring: Arc<dyn KeyringClient>,
    ) -> joinerror::Result<Arc<Self>> {
        Ok(Self {
            settings: UserSettingsService::new(abs_path.clone(), fs.clone())
                .await?
                .into(),
            accounts: UserAccountsService::new(
                abs_path,
                fs,
                server_api_client,
                github_api_client,
                gitlab_api_client,
                github_auth_adapter,
                gitlab_auth_adapter,
                keyring,
            )
            .await?,
        }
        .into())
    }

    pub fn settings(&self) -> Arc<dyn SettingsStore> {
        self.settings.clone()
    }

    pub async fn accounts(&self) -> Vec<AccountInfo> {
        self.accounts.accounts().await
    }

    pub async fn account(&self, account_id: &AccountId) -> Option<Account> {
        self.accounts.account(account_id).await
    }

    pub async fn add_account(
        &self,
        ctx: &dyn AnyAsyncContext,
        params: AddAccountParams,
    ) -> joinerror::Result<()> {
        self.accounts.add_account(ctx, params).await
    }

    pub async fn remove_account(
        &self,
        ctx: &dyn AnyAsyncContext,
        account_id: &AccountId,
    ) -> joinerror::Result<()> {
        self.accounts.remove_account(ctx, account_id).await
    }

    pub async fn update_account(
        &self,
        ctx: &dyn AnyAsyncContext,
        account_id: &AccountId,
        params: UpdateAccountParams,
    ) -> joinerror::Result<()> {
        self.accounts.update_account(ctx, account_id, params).await
    }
}
