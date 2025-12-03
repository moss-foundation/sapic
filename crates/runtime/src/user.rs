pub mod user_accounts;
pub mod user_settings;

use async_trait::async_trait;
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
    user::{AddAccountParams, UpdateAccountParams, User, account::Account},
};
use std::{path::PathBuf, sync::Arc};

use crate::user::{user_accounts::UserAccountsService, user_settings::UserSettingsService};

pub struct AppUser {
    settings: Arc<UserSettingsService>,
    accounts: UserAccountsService,
}

impl AppUser {
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
        let user_abs_path = abs_path.join("user");

        Ok(Self {
            settings: UserSettingsService::new(user_abs_path.clone(), fs.clone())
                .await?
                .into(),
            accounts: UserAccountsService::new(
                user_abs_path,
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
}

#[async_trait]
impl User for AppUser {
    fn settings(&self) -> Arc<dyn SettingsStore> {
        self.settings.clone()
    }

    async fn accounts(&self) -> Vec<AccountInfo> {
        self.accounts.accounts().await
    }

    async fn account(&self, account_id: &AccountId) -> Option<Account> {
        self.accounts.account(account_id).await
    }

    async fn add_account(
        &self,
        ctx: &dyn AnyAsyncContext,
        params: AddAccountParams,
    ) -> joinerror::Result<AccountId> {
        self.accounts.add_account(ctx, params).await
    }

    async fn remove_account(
        &self,
        ctx: &dyn AnyAsyncContext,
        account_id: &AccountId,
    ) -> joinerror::Result<()> {
        self.accounts.remove_account(ctx, account_id).await
    }

    async fn update_account(
        &self,
        ctx: &dyn AnyAsyncContext,
        account_id: &AccountId,
        params: UpdateAccountParams,
    ) -> joinerror::Result<()> {
        self.accounts.update_account(ctx, account_id, params).await
    }
}
