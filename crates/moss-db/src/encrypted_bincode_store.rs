use anyhow::Result;
use redb::Key;
use serde::{de::DeserializeOwned, Serialize};
use std::borrow::Borrow;

use crate::{
    encrypted_bincode_table::{EncryptedBincodeTable, EncryptionOptions},
    DatabaseClient, ReDbClient, Transaction,
};

pub struct EncryptedBincodeStore<'a, K, V>
where
    K: Key + 'static + Borrow<K::SelfType<'a>>,
    V: Serialize + DeserializeOwned,
{
    client: ReDbClient,
    config: EncryptionOptions,
    table: EncryptedBincodeTable<'a, K, V>,
}

impl<'a, K, V> EncryptedBincodeStore<'a, K, V>
where
    K: Key + 'static + Borrow<K::SelfType<'a>>,
    V: Serialize + DeserializeOwned,
{
    pub fn new(
        client: ReDbClient,
        table: EncryptedBincodeTable<'a, K, V>,
        config: EncryptionOptions,
    ) -> Self {
        Self {
            client,
            table,
            config,
        }
    }

    pub fn begin_write<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(Transaction, &EncryptedBincodeTable<'a, K, V>, &EncryptionOptions) -> Result<T>,
    {
        let write_txn = self.client.begin_write()?;
        let result = f(Transaction::Write(write_txn), &self.table, &self.config)?;
        Ok(result)
    }

    pub fn begin_read<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(Transaction, &EncryptedBincodeTable<'a, K, V>, &EncryptionOptions) -> Result<T>,
    {
        let read_txn = self.client.0.begin_read()?;
        let result = f(Transaction::Read(read_txn), &self.table, &self.config)?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use crate::encrypted_bincode_table::DEFAULT_ENCRYPTION_OPTIONS;

    use super::*;

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct MyStruct {
        val: u64,
    }

    const TABLE_VAULT_2: EncryptedBincodeTable<&str, MyStruct> =
        EncryptedBincodeTable::new("vault_2", DEFAULT_ENCRYPTION_OPTIONS);
    const TEST_PASSWORD: &[u8] = b"test_password_123";
    const TEST_AAD: &[u8] = b"additional_authenticated_data";

    #[test]
    fn test_encrypted_write_read() {
        let client = ReDbClient::new("sapic.db").unwrap();
        let store = EncryptedBincodeStore::new(
            client,
            TABLE_VAULT_2,
            EncryptionOptions {
                memory_cost: 65536,
                time_cost: 10,
                parallelism: 4,
                salt_len: 32,
                nonce_len: 12,
            },
        );

        store
            .begin_write(|mut txn, table, config| {
                table
                    .write(
                        &mut txn,
                        "my_key",
                        &MyStruct { val: 42 },
                        TEST_PASSWORD,
                        TEST_AAD,
                        config,
                    )
                    .unwrap();

                Ok(txn.commit()?)
            })
            .unwrap();

        let r = store
            .begin_read(|txn, table, config| {
                let r = table.read(&txn, "my_key", TEST_PASSWORD, TEST_AAD, config)?;

                Ok(r)
            })
            .unwrap();

        println!("{:?}", r);
    }
}
