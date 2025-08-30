use std::collections::HashMap;

use tokio::sync::RwLock;

use crate::{AccountSession, account::Account, models::primitives::AccountId};

pub struct ActiveProfile {
    accounts: RwLock<HashMap<AccountId, Account>>,
}

impl ActiveProfile {
    pub fn new(accounts: HashMap<AccountId, Account>) -> Self {
        Self {
            accounts: RwLock::new(accounts),
        }
    }

    pub async fn account(&self, account_id: &AccountId) -> Option<Account> {
        self.accounts
            .read()
            .await
            .get(account_id)
            .map(|account| account.clone())
    }
}

pub struct ProfileAccount {
    username: String,
    session: AccountSession,
}

impl ProfileAccount {
    pub fn session(&self) -> &AccountSession {
        &self.session
    }
}
