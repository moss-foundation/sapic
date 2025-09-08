use std::collections::HashMap;

use moss_applib::AppRuntime;
use tokio::sync::RwLock;

use crate::{account::Account, models::primitives::AccountId};

pub struct ActiveProfile<R: AppRuntime> {
    accounts: RwLock<HashMap<AccountId, Account<R>>>,
}

impl<R: AppRuntime> ActiveProfile<R> {
    pub fn new(accounts: HashMap<AccountId, Account<R>>) -> Self {
        Self {
            accounts: RwLock::new(accounts),
        }
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

    pub async fn remove_account(&self, account_id: &AccountId) {
        self.accounts.write().await.remove(account_id);
    }
}

// pub struct ProfileAccount {
//     username: String,
//     session: AccountSession,
// }

// impl ProfileAccount {
//     pub fn session(&self) -> &AccountSession {
//         &self.session
//     }
// }
