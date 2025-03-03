pub mod vault_store;

use anyhow::Result;

use crate::models::storage::operations::CreateVaultItemInput;

pub trait VaultStore {
    fn create_item(&self, input: CreateVaultItemInput) -> Result<()>;
}
