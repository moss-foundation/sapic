pub mod vault_store;

use moss_db::{
    encrypted_bincode_store::EncryptedBincodeStore,
    encrypted_bincode_table::{EncryptedBincodeTable, EncryptionOptions},
    ReDbClient,
};

use crate::models::types::VaultEntry;

pub const TABLE_VAULT: EncryptedBincodeTable<String, VaultEntry> =
    EncryptedBincodeTable::new("vault");

pub fn create_vault_store(
    client: ReDbClient,
) -> EncryptedBincodeStore<'static, String, VaultEntry> {
    EncryptedBincodeStore::new(client, TABLE_VAULT, EncryptionOptions::default())
}
