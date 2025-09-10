use std::collections::HashMap;

use moss_applib::AppRuntime;
use tokio::sync::RwLock;

use crate::{
    account::Account,
    models::{
        primitives::{AccountId, AccountKind, ProfileId},
        types::AccountInfo,
    },
};

pub struct Profile<R: AppRuntime> {
    id: ProfileId,
    accounts: RwLock<HashMap<AccountId, Account<R>>>,
}

impl<R: AppRuntime> Profile<R> {
    pub fn new(id: ProfileId, accounts: HashMap<AccountId, Account<R>>) -> Self {
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

    pub async fn account(&self, account_id: &AccountId) -> Option<Account<R>> {
        self.accounts
            .read()
            .await
            .get(account_id)
            .map(|account| account.clone())
    }

    // HACK: Use the first account as the default account
    // FIXME: We can avoid this by not having passing account as a parameter from the frontend
    pub async fn first(&self) -> Option<Account<R>> {
        self.accounts
            .read()
            .await
            .values()
            .next()
            .map(|account| account.clone())
    }

    pub async fn add_account(&self, account: Account<R>) {
        self.accounts.write().await.insert(account.id(), account);
    }

    pub async fn remove_account(&self, account_id: &AccountId) -> joinerror::Result<()> {
        // TODO: Revoke the account session

        self.accounts.write().await.remove(account_id);

        Ok(())
    }
}
