pub mod account;
pub mod profile;

use async_trait::async_trait;
use sapic_base::user::types::{
    AccountInfo,
    primitives::{AccountId, AccountKind},
};
use sapic_core::context::AnyAsyncContext;
use std::sync::Arc;

use crate::{configuration::SettingsStore, user::account::Account};

pub struct AddAccountParams {
    pub host: String,
    pub kind: AccountKind,
    pub pat: Option<String>,
}

pub struct UpdateAccountParams {
    pub pat: Option<String>,
}

#[async_trait]
pub trait User: Send + Sync {
    fn settings(&self) -> Arc<dyn SettingsStore>;
    async fn accounts(&self) -> Vec<AccountInfo>;
    async fn account(&self, account_id: &AccountId) -> Option<Account>;
    async fn add_account(
        &self,
        ctx: &dyn AnyAsyncContext,
        params: AddAccountParams,
    ) -> joinerror::Result<AccountId>;
    async fn remove_account(
        &self,
        ctx: &dyn AnyAsyncContext,
        account_id: &AccountId,
    ) -> joinerror::Result<()>;
    async fn update_account(
        &self,
        ctx: &dyn AnyAsyncContext,
        account_id: &AccountId,
        params: UpdateAccountParams,
    ) -> joinerror::Result<()>;
}
