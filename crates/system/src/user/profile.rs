use sapic_base::user::types::{
    AccountInfo,
    primitives::{AccountId, AccountKind, ProfileId},
};
use sapic_core::context::AnyAsyncContext;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

use crate::{ports::server_api::RevokeApiReq, user::account::Account};

pub struct Profile {
    id: ProfileId,
    accounts: RwLock<HashMap<AccountId, Account>>,
}

impl Profile {
    pub fn new(id: ProfileId, accounts: HashMap<AccountId, Account>) -> Self {
        Self {
            id,
            accounts: RwLock::new(accounts),
        }
    }

    pub fn id(&self) -> &ProfileId {
        &self.id
    }

    pub async fn is_account_exists(
        &self,
        username: &str,
        kind: AccountKind,
        host: &str,
    ) -> Option<AccountInfo> {
        let accounts = self.accounts.read().await;
        for account in accounts.values() {
            if account.username == username && account.kind == kind && account.host == host {
                return Some(account.info());
            }
        }

        None
    }

    pub async fn account(&self, account_id: &AccountId) -> Option<Account> {
        self.accounts
            .read()
            .await
            .get(account_id)
            .map(|account| account.clone())
    }

    // HACK: Use the first account as the default account
    // FIXME: We can avoid this by not having passing account as a parameter from the frontend
    pub async fn first(&self) -> Option<Account> {
        self.accounts
            .read()
            .await
            .values()
            .next()
            .map(|account| account.clone())
    }

    pub async fn add_account(&self, account: Account) {
        self.accounts.write().await.insert(account.id(), account);
    }

    pub async fn remove_account(
        &self,
        ctx: &dyn AnyAsyncContext,
        api_client: Arc<dyn RevokeApiReq>,
        account_id: &AccountId,
    ) -> joinerror::Result<()> {
        let account = self.accounts.write().await.remove(account_id);
        if let Some(account) = account {
            account.revoke(ctx, api_client).await?;
        }

        Ok(())
    }
}
