use anyhow::Result;
use moss_app::service_pool::AppService;
use moss_db::{
    encrypted_bincode_store::EncryptedBincodeStore, encrypted_bincode_table::EncryptionOptions,
    ReDbClient,
};

use crate::{
    models::{operations::CreateVaultItemInput, types::VaultEntry},
    storage::TABLE_VAULT,
};

const TEST_PASSWORD: &[u8] = b"test_password_123";
const TEST_AAD: &[u8] = b"additional_authenticated_data";

pub struct VaultService {
    store: EncryptedBincodeStore<'static, String, VaultEntry>,
}

impl VaultService {
    pub fn new(client: ReDbClient) -> Self {
        Self {
            store: EncryptedBincodeStore::new(client, TABLE_VAULT, EncryptionOptions::default()),
        }
    }

    pub fn create_vault_item(&self, input: CreateVaultItemInput) -> Result<()> {
        self.store.begin_write(|mut txn, table, config| {
            table.write(
                &mut txn,
                input.key,
                &VaultEntry {
                    value: input.value,
                    description: input.description,
                },
                TEST_PASSWORD,
                TEST_AAD,
                config,
            )?;
            Ok(())
        })
    }
}

impl AppService for VaultService {}
