use aes_gcm::{
    aead::{rand_core::RngCore, Aead, KeyInit, Payload},
    Aes256Gcm, Key as AesKey, Nonce,
};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use redb::{Key, TableDefinition};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::borrow::Borrow;
use std::fmt::{Debug, Display};
use zeroize::Zeroizing;

use crate::{common::DatabaseError, Table, Transaction};

pub const DEFAULT_ENCRYPTION_OPTIONS: EncryptionOptions = EncryptionOptions {
    memory_cost: 65536, // 64MB
    time_cost: 10,
    parallelism: 4,
    salt_len: 32,
    nonce_len: 12,
};

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
            time_cost: 10,
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

        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password, &salt)
            .map_err(|e| DatabaseError::Internal(format!("Failed to hash password: {}", e)))?;

        let mut key_bytes = [0u8; 32];
        key_bytes.copy_from_slice(&password_hash.hash.unwrap().as_bytes()[..32]);
        Ok(Zeroizing::new(key_bytes))
    }

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

    pub fn write(
        &self,
        txn: &mut Transaction,
        key: K,
        value: &V,
        password: &[u8],
        aad: &[u8],
    ) -> Result<(), DatabaseError> {
        match txn {
            Transaction::Write(txn) => {
                let mut table = txn.open_table(self.table)?;
                let serialized = bincode::serialize(value)?;
                let encrypted = self.encrypt(&serialized, password, aad)?;
                table.insert(key.borrow(), encrypted)?;
                Ok(())
            }
            Transaction::Read(_) => Err(DatabaseError::Transaction(
                "Cannot insert into read transaction".to_string(),
            )),
        }
    }

    pub fn read(
        &self,
        txn: &Transaction,
        key: K,
        password: &[u8],
        aad: &[u8],
    ) -> Result<V, DatabaseError> {
        match txn {
            Transaction::Read(txn) => {
                let table = txn.open_table(self.table)?;
                let entry = table
                    .get(key.borrow())?
                    .ok_or_else(|| DatabaseError::NotFound {
                        key: key.to_string(),
                    })?;

                let encrypted = entry.value();
                let decrypted = self.decrypt(&encrypted, password, aad)?;
                let result = bincode::deserialize(&decrypted)?;

                Ok(result)
            }
            Transaction::Write(_) => Err(DatabaseError::Transaction(
                "Cannot read from write transaction".to_string(),
            )),
        }
    }
}
