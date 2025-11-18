use aes_gcm::{
    Aes256Gcm, Key as AesKey, Nonce,
    aead::{Aead, KeyInit, Payload, rand_core::RngCore},
};
use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use redb::{Key, TableDefinition};
use sapic_core::context::AnyAsyncContext;
use serde::{Serialize, de::DeserializeOwned};
use std::{
    borrow::Borrow,
    fmt::{Debug, Display},
};
use zeroize::Zeroizing;

use crate::{DatabaseError, Table, Transaction};

// See https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html#argon2id
// OWASP recommends m=47104 (46 MiB), t=1, p=1. Our default is stronger, and appears to have acceptable performance

#[derive(Debug, Clone)]
pub struct EncryptionOptions {
    pub memory_cost: u32,
    pub time_cost: u32,
    pub parallelism: u32,
    pub salt_len: usize,
    pub nonce_len: usize,
}

impl Default for EncryptionOptions {
    fn default() -> Self {
        Self {
            memory_cost: 65536, // 64MB
            time_cost: 2,
            parallelism: 4,
            salt_len: 32,
            nonce_len: 12,
        }
    }
}

#[derive(Clone)]
pub struct EncryptedBincodeTable<'a, K, V>
where
    K: Key + 'static + Borrow<K::SelfType<'a>> + Debug + Display,
    V: Serialize + DeserializeOwned,
{
    table: TableDefinition<'a, K, Vec<u8>>,
    options: EncryptionOptions,
    _marker: std::marker::PhantomData<V>,
}

impl<'a, K, V> From<&EncryptedBincodeTable<'a, K, V>> for TableDefinition<'a, K, Vec<u8>>
where
    K: Key + 'static + Borrow<K::SelfType<'a>> + Clone + Eq + Debug + Display,
    for<'b> K::SelfType<'b>: ToOwned<Owned = K>,
    V: Serialize + DeserializeOwned,
{
    fn from(value: &EncryptedBincodeTable<'a, K, V>) -> Self {
        value.table
    }
}

impl<'a, K, V> Table<'a, K, V> for EncryptedBincodeTable<'a, K, V>
where
    K: Key + 'static + Borrow<K::SelfType<'a>> + Clone + Eq + Debug + Display,
    for<'b> K::SelfType<'b>: ToOwned<Owned = K>,
    V: Serialize + DeserializeOwned,
{
    fn table_definition(&self) -> TableDefinition<'a, K, Vec<u8>> {
        self.table.clone()
    }
}

impl<'a, K, V> EncryptedBincodeTable<'a, K, V>
where
    K: Key + Borrow<K::SelfType<'a>> + Debug + Display,
    V: Serialize + DeserializeOwned,
{
    pub const fn new(table_name: &'static str, options: EncryptionOptions) -> Self {
        Self {
            table: TableDefinition::new(table_name),
            options,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn clone(&self) -> Self {
        Self {
            table: self.table.clone(),
            options: self.options.clone(),
            _marker: std::marker::PhantomData,
        }
    }

    fn derive_key(
        &self,
        password: &[u8],
        salt: &[u8],
    ) -> Result<Zeroizing<[u8; 32]>, DatabaseError> {
        let salt = SaltString::encode_b64(salt)
            .map_err(|e| DatabaseError::Internal(format!("Failed to encode salt: {}", e)))?;

        let params = argon2::ParamsBuilder::new()
            .m_cost(self.options.memory_cost)
            .t_cost(self.options.time_cost)
            .p_cost(self.options.parallelism)
            .output_len(32)
            .build()
            .map_err(|e| {
                DatabaseError::Internal(format!("Failed to build Argon2 params: {}", e))
            })?;

        let argon2 = Argon2::new(
            argon2::Algorithm::default(),
            argon2::Version::default(),
            params,
        );
        let password_hash = argon2
            .hash_password(password, &salt)
            .map_err(|e| DatabaseError::Internal(format!("Failed to hash password: {}", e)))?;

        let mut key_bytes = [0u8; 32];
        key_bytes.copy_from_slice(&password_hash.hash.unwrap().as_bytes()[..32]);
        Ok(Zeroizing::new(key_bytes))
    }

    #[allow(deprecated)] // Fine since gonna replace with SQLite database in the future
    fn encrypt(&self, data: &[u8], password: &[u8], aad: &[u8]) -> Result<Vec<u8>, DatabaseError> {
        let mut salt = vec![0u8; self.options.salt_len];
        let mut nonce_bytes = vec![0u8; self.options.nonce_len];

        OsRng.fill_bytes(&mut salt);
        OsRng.fill_bytes(&mut nonce_bytes);

        let derived_key = self.derive_key(password, &salt)?;
        let aes_key = AesKey::<Aes256Gcm>::from_slice(derived_key.as_slice());
        let cipher = Aes256Gcm::new(aes_key);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let payload = Payload { msg: data, aad };
        let ciphertext = cipher
            .encrypt(nonce, payload)
            .map_err(|e| DatabaseError::Internal(format!("Encryption failed: {}", e)))?;

        // Combine salt + nonce + ciphertext
        let mut result = Vec::with_capacity(salt.len() + nonce_bytes.len() + ciphertext.len());
        result.extend_from_slice(&salt);
        result.extend_from_slice(&nonce_bytes);
        result.extend_from_slice(&ciphertext);
        Ok(result)
    }

    #[allow(deprecated)] // Fine since gonna replace with SQLite database in the future
    fn decrypt(&self, data: &[u8], password: &[u8], aad: &[u8]) -> Result<Vec<u8>, DatabaseError> {
        let min_len = self.options.salt_len + self.options.nonce_len;
        if data.len() < min_len {
            return Err(DatabaseError::Internal(
                "Invalid encrypted data: too short".to_string(),
            ));
        }

        let (salt, rest) = data.split_at(self.options.salt_len);
        let (nonce_bytes, ciphertext) = rest.split_at(self.options.nonce_len);
        let derived_key = self.derive_key(password, salt)?;
        let aes_key = AesKey::<Aes256Gcm>::from_slice(derived_key.as_slice());
        let cipher = Aes256Gcm::new(aes_key);
        let nonce = Nonce::from_slice(nonce_bytes);

        let payload = Payload {
            msg: ciphertext,
            aad,
        };

        cipher
            .decrypt(nonce, payload)
            .map_err(|e| DatabaseError::Internal(format!("Decryption failed: {}", e)))
    }

    pub async fn write<C: AnyAsyncContext>(
        &self,
        ctx: &C,
        txn: &mut Transaction,
        key: K,
        value: &V,
        password: &[u8],
        aad: &[u8],
    ) -> Result<(), DatabaseError> {
        if let Some(reason) = ctx.done() {
            return Err(DatabaseError::Canceled(reason));
        }

        tokio::time::timeout(ctx.deadline(), async move {
            match txn {
                Transaction::Write(txn) => {
                    let mut table = txn.open_table(self.table)?;

                    let bytes = serde_json::to_vec(value)?;

                    let encrypted = self.encrypt(&bytes, password, aad)?;
                    table.insert(key.borrow(), encrypted)?;
                    Ok(())
                }
                Transaction::Read(_) => Err(DatabaseError::Transaction(
                    "Cannot insert into read transaction".to_string(),
                )),
            }
        })
        .await
        .map_err(|_| DatabaseError::Timeout("write".to_string()))?
    }

    pub async fn read<C: AnyAsyncContext>(
        &self,
        ctx: &C,
        txn: &Transaction,
        key: K,
        password: &[u8],
        aad: &[u8],
    ) -> Result<V, DatabaseError> {
        if let Some(reason) = ctx.done() {
            return Err(DatabaseError::Canceled(reason));
        }

        tokio::time::timeout(ctx.deadline(), async move {
            match txn {
                Transaction::Read(txn) => {
                    let table = txn.open_table(self.table)?;

                    let encrypted = table
                        .get(key.borrow())?
                        .ok_or_else(|| DatabaseError::NotFound {
                            key: key.to_string(),
                        })?
                        .value();

                    let decrypted = self.decrypt(&encrypted, password, aad)?;
                    let result = serde_json::from_slice(&decrypted)?;

                    Ok(result)
                }
                Transaction::Write(_) => Err(DatabaseError::Transaction(
                    "Cannot read from write transaction".to_string(),
                )),
            }
        })
        .await
        .map_err(|_| DatabaseError::Timeout("read".to_string()))?
    }
}
