use aes_gcm::{
    aead::{Aead, KeyInit, Payload},
    Aes256Gcm, Key, Nonce,
};
use anyhow::{anyhow, Context as _, Result};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use rand::RngCore;
use redb::{
    Database, Key as ReDbKey, ReadTransaction as InnerReadTransaction, TableDefinition,
    WriteTransaction as InnerWriteTransaction,
};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::{borrow::Borrow, path::Path};
use zeroize::Zeroizing;

const TABLE_VAULT: BincodeTable<&str, MyStruct> = BincodeTable::new("vault");

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MyStruct {
    val: u64,
}

pub enum Transaction {
    Read(InnerReadTransaction),
    Write(InnerWriteTransaction),
}

impl Transaction {
    pub fn commit(self) -> Result<()> {
        match self {
            Transaction::Read(_) => Ok(()),
            Transaction::Write(txn) => Ok(txn.commit()?),
        }
    }
}

pub trait DatabaseClient: Sized {
    fn begin_write(&self) -> Result<InnerWriteTransaction>;
}

pub struct ReDbClient(Database);

impl ReDbClient {
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        Ok(Self(Database::create(path)?))
    }
}

impl DatabaseClient for ReDbClient {
    fn begin_write(&self) -> Result<InnerWriteTransaction> {
        Ok(self.0.begin_write()?)
    }
}

pub trait Store<
    'a: 'static,
    K: ReDbKey + 'static + Borrow<K::SelfType<'a>>,
    V: Serialize + DeserializeOwned,
>
{
    fn write<'txn, F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(Transaction, BincodeTable<K, V>) -> Result<T>;
    fn read<'txn, F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(Transaction, BincodeTable<K, V>) -> Result<T>;
}

#[derive(Clone)]
pub struct BincodeTable<'a, K, V>
where
    'a: 'static,
    K: ReDbKey + 'static + Borrow<K::SelfType<'a>>,
    V: Serialize + DeserializeOwned,
{
    table: TableDefinition<'a, K, Vec<u8>>,
    _marker: std::marker::PhantomData<V>,
}

impl<'a, K, V> BincodeTable<'a, K, V>
where
    'a: 'static,
    K: ReDbKey + 'static + Borrow<K::SelfType<'a>>,
    V: Serialize + DeserializeOwned,
{
    pub const fn new(table_name: &'static str) -> Self {
        Self {
            table: TableDefinition::new(table_name),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn clone(&self) -> Self {
        Self {
            table: self.table.clone(),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn insert(&self, txn: &mut Transaction, key: K, value: &V) -> Result<()> {
        match txn {
            Transaction::Write(txn) => {
                let mut table = txn.open_table(self.table)?;
                let bytes = bincode::serialize(value)?;
                table.insert(key.borrow(), bytes)?;
                Ok(())
            }
            Transaction::Read(_) => Err(anyhow!("Cannot insert into read transaction")),
        }
    }

    pub fn read(&self, txn: &Transaction, key: K) -> Result<V> {
        match txn {
            Transaction::Read(txn) => {
                let table = txn.open_table(self.table).context("Failed to open table")?;
                let entry = table
                    .get(key)
                    .context("Failed to retrieve value from table")?
                    .ok_or_else(|| anyhow!("No value found for the specified key"))?;

                let value = entry.value().to_vec();
                let result =
                    bincode::deserialize(&value).context("Failed to deserialize the data")?;

                Ok(result)
            }
            Transaction::Write(_) => Err(anyhow!("Cannot read from write transaction")),
        }
    }
}

// pub struct ReadOnlyTable<'a, >(BincodeTable<'a, K, V>)
// where
//     'a: 'static,
//     K: Key + 'static + Borrow<K::SelfType<'a>>,
//     V: Serialize + DeserializeOwned;

// impl<'a, K, V> BincodeReadOnlyTable<'a, K, V>
// where
//     'a: 'static,
//     K: Key + 'static + Borrow<K::SelfType<'a>>,
//     V: Serialize + DeserializeOwned,
// {
//     pub fn read(&self, txn: &Transaction, key: K) -> Result<V> {
//         self.0.read(txn, key)
//     }
// }

pub struct BincodeStore<'a, K, V>
where
    'a: 'static,
    K: ReDbKey + 'static + Borrow<K::SelfType<'a>>,
    V: Serialize + DeserializeOwned,
{
    client: ReDbClient,
    table: BincodeTable<'a, K, V>,
}

impl<'a, K, V> BincodeStore<'a, K, V>
where
    'a: 'static,
    K: ReDbKey + 'static + Borrow<K::SelfType<'a>>,
    V: Serialize + DeserializeOwned,
{
    pub fn new(client: ReDbClient, table: BincodeTable<K, V>) -> Self {
        Self { client, table }
    }
}

// impl<'a, K, V> Store<'a, K, V> for BincodeStore<'a, K, V>
// where
//     'a: 'static,
//     K: ReDbKey + 'static + Borrow<K::SelfType<'a>>,
//     V: Serialize + DeserializeOwned,
// {
//     fn write<'txn, F, T>(&self, f: F) -> Result<T>
//     where
//         F: FnOnce(Transaction, BincodeTable<K, V>) -> Result<T>,
//     {
//         let write_txn = self.client.begin_write()?;
//         f(Transaction::Write(write_txn), self.table.clone())
//     }

//     fn read<'txn, F, T>(&self, f: F) -> Result<T>
//     where
//         F: FnOnce(Transaction, BincodeTable<K, V>) -> Result<T>,
//     {
//         let read_txn = self.client.0.begin_read()?;
//         f(Transaction::Read(read_txn), self.table.clone())
//     }
// }

impl<'a, K, V> Store2<'a, K, V> for BincodeStore<'a, K, V>
where
    'a: 'static,
    K: ReDbKey + 'static + Borrow<K::SelfType<'a>>,
    V: Serialize + DeserializeOwned,
{
    type Table = BincodeTable<'a, K, V>;
    type Config = ();

    fn write<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(Transaction, &Self::Table, &Self::Config) -> Result<T>,
    {
        let write_txn = self.client.begin_write()?;
        f(Transaction::Write(write_txn), &self.table, &())
    }

    fn read<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(Transaction, &Self::Table, &Self::Config) -> Result<T>,
    {
        let read_txn = self.client.0.begin_read()?;
        f(Transaction::Read(read_txn), &self.table, &())
    }
}

#[derive(Debug, Clone)]
pub struct EncryptionConfig {
    pub memory_cost: u32,
    pub time_cost: u32,
    pub parallelism: u32,
    pub salt_len: usize,
    pub nonce_len: usize,
}

impl Default for EncryptionConfig {
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
    'a: 'static,
    K: ReDbKey + 'static + Borrow<K::SelfType<'a>>,
    V: Serialize + DeserializeOwned,
{
    table: TableDefinition<'a, K, Vec<u8>>,
    _marker: std::marker::PhantomData<V>,
}

impl<'a, K, V> EncryptedBincodeTable<'a, K, V>
where
    'a: 'static,
    K: ReDbKey + 'static + Borrow<K::SelfType<'a>>,
    V: Serialize + DeserializeOwned,
{
    pub const fn new(table_name: &'static str) -> Self {
        Self {
            table: TableDefinition::new(table_name),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn clone(&self) -> Self {
        Self {
            table: self.table.clone(),
            _marker: std::marker::PhantomData,
        }
    }

    fn derive_key(&self, password: &[u8], salt: &[u8]) -> Result<Zeroizing<[u8; 32]>> {
        let salt = SaltString::encode_b64(salt)
            .map_err(|e| anyhow::anyhow!("Failed to encode salt: {}", e))?;

        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password, &salt)
            .map_err(|e| anyhow::anyhow!("Failed to hash password: {}", e))?;

        let mut key_bytes = [0u8; 32];
        key_bytes.copy_from_slice(&password_hash.hash.unwrap().as_bytes()[..32]);
        Ok(Zeroizing::new(key_bytes))
    }

    fn encrypt(
        &self,
        data: &[u8],
        password: &[u8],
        aad: &[u8],
        config: &EncryptionConfig,
    ) -> Result<Vec<u8>> {
        let mut salt = vec![0u8; config.salt_len];
        let mut nonce_bytes = vec![0u8; config.nonce_len];

        OsRng.fill_bytes(&mut salt);
        OsRng.fill_bytes(&mut nonce_bytes);

        let derived_key = self.derive_key(password, &salt)?;
        let aes_key = Key::<Aes256Gcm>::from_slice(derived_key.as_slice());
        let cipher = Aes256Gcm::new(aes_key);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let payload = Payload { msg: data, aad };
        let ciphertext = cipher
            .encrypt(nonce, payload)
            .map_err(|e| anyhow::anyhow!("Encryption failed: {}", e))?;

        // Combine salt + nonce + ciphertext
        let mut result = Vec::with_capacity(salt.len() + nonce_bytes.len() + ciphertext.len());
        result.extend_from_slice(&salt);
        result.extend_from_slice(&nonce_bytes);
        result.extend_from_slice(&ciphertext);
        Ok(result)
    }

    fn decrypt(
        &self,
        data: &[u8],
        password: &[u8],
        aad: &[u8],
        config: &EncryptionConfig,
    ) -> Result<Vec<u8>> {
        let min_len = config.salt_len + config.nonce_len;
        if data.len() < min_len {
            return Err(anyhow::anyhow!("Invalid encrypted data: too short"));
        }

        let (salt, rest) = data.split_at(config.salt_len);
        let (nonce_bytes, ciphertext) = rest.split_at(config.nonce_len);
        let derived_key = self.derive_key(password, salt)?;
        let aes_key = Key::<Aes256Gcm>::from_slice(derived_key.as_slice());
        let cipher = Aes256Gcm::new(aes_key);
        let nonce = Nonce::from_slice(nonce_bytes);

        let payload = Payload {
            msg: ciphertext,
            aad,
        };

        cipher
            .decrypt(nonce, payload)
            .map_err(|e| anyhow::anyhow!("Decryption failed: {}", e))
    }

    pub fn insert(
        &self,
        txn: &mut Transaction,
        key: K,
        value: &V,
        password: &[u8],
        aad: &[u8],
        config: &EncryptionConfig,
    ) -> Result<()> {
        match txn {
            Transaction::Write(txn) => {
                let mut table = txn.open_table(self.table)?;
                let serialized = bincode::serialize(value)?;
                let encrypted = self.encrypt(&serialized, password, aad, config)?;
                table.insert(key.borrow(), encrypted)?;
                Ok(())
            }
            Transaction::Read(_) => Err(anyhow!("Cannot insert into read transaction")),
        }
    }

    pub fn read(
        &self,
        txn: &Transaction,
        key: K,
        password: &[u8],
        aad: &[u8],
        config: &EncryptionConfig,
    ) -> Result<V> {
        match txn {
            Transaction::Read(txn) => {
                let table = txn.open_table(self.table).context("Failed to open table")?;
                let entry = table
                    .get(key)
                    .context("Failed to retrieve value from table")?
                    .ok_or_else(|| anyhow!("No value found for the specified key"))?;

                let encrypted = entry.value();
                let decrypted = self.decrypt(&encrypted, password, aad, config)?;
                let result = bincode::deserialize(&decrypted)
                    .context("Failed to deserialize the decrypted data")?;

                Ok(result)
            }
            Transaction::Write(_) => Err(anyhow!("Cannot read from write transaction")),
        }
    }
}

pub struct EncryptedBincodeStore<'a, K, V>
where
    'a: 'static,
    K: ReDbKey + 'static + Borrow<K::SelfType<'a>>,
    V: Serialize + DeserializeOwned,
{
    client: ReDbClient,
    config: EncryptionConfig,
    table: EncryptedBincodeTable<'a, K, V>,
}

impl<'a, K, V> EncryptedBincodeStore<'a, K, V>
where
    'a: 'static,
    K: ReDbKey + 'static + Borrow<K::SelfType<'a>>,
    V: Serialize + DeserializeOwned,
{
    pub fn new(
        client: ReDbClient,
        table: EncryptedBincodeTable<K, V>,
        config: EncryptionConfig,
    ) -> Self {
        Self {
            client,
            table,
            config,
        }
    }

    // pub fn write<'txn, F, T>(&self, f: F) -> Result<T>
    // where
    //     F: FnOnce(Transaction, EncryptedBincodeTable<K, V>, &EncryptionConfig) -> Result<T>,
    // {
    //     let write_txn = self.client.begin_write()?;
    //     f(
    //         Transaction::Write(write_txn),
    //         self.table.clone(),
    //         &self.config,
    //     )
    // }

    // pub fn read<'txn, F, T>(&self, f: F) -> Result<T>
    // where
    //     F: FnOnce(Transaction, EncryptedBincodeTable<K, V>, &EncryptionConfig) -> Result<T>,
    // {
    //     let read_txn = self.client.0.begin_read()?;
    //     f(
    //         Transaction::Read(read_txn),
    //         self.table.clone(),
    //         &self.config,
    //     )
    // }
}

pub trait Store2<'a, K, V>
where
    'a: 'static,
    K: ReDbKey + 'static + Borrow<K::SelfType<'a>>,
    V: Serialize + DeserializeOwned,
{
    type Table;
    type Config;

    fn write<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(Transaction, &Self::Table, &Self::Config) -> Result<T>;
    fn read<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(Transaction, &Self::Table, &Self::Config) -> Result<T>;
}

impl<'a, K, V> Store2<'a, K, V> for EncryptedBincodeStore<'a, K, V>
where
    'a: 'static,
    K: ReDbKey + 'static + Borrow<K::SelfType<'a>>,
    V: Serialize + DeserializeOwned,
{
    type Table = EncryptedBincodeTable<'a, K, V>;
    type Config = EncryptionConfig;

    fn write<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(Transaction, &Self::Table, &Self::Config) -> Result<T>,
    {
        let write_txn = self.client.begin_write()?;
        let result = f(Transaction::Write(write_txn), &self.table, &self.config)?;
        Ok(result)
    }

    fn read<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(Transaction, &Self::Table, &Self::Config) -> Result<T>,
    {
        let read_txn = self.client.0.begin_read()?;
        let result = f(Transaction::Read(read_txn), &self.table, &self.config)?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write() {
        let client = ReDbClient::new("sapic.db").unwrap();
        let vault_store = BincodeStore::new(client, TABLE_VAULT);

        vault_store
            .write(|mut txn, table, _| -> Result<()> {
                table.insert(&mut txn, "my_key", &MyStruct { val: 42 })?;

                Ok(txn.commit()?)
            })
            .unwrap();

        // let client = Arc::new(ReDbClient::new(Database::create("sapic.db").unwrap()));
        // let vault_store = VaultStore::new(client);

        // vault_store
        //     .write(|txn, wrapper| {
        //         let mut table = txn.open_table(wrapper.table_definition())?;
        //         wrapper.insert(&mut table, "my_key", &42u64)?;
        //         Ok(())
        //     })
        //     .unwrap();
    }

    #[test]
    fn test_read() {
        let client = ReDbClient::new("sapic.db").unwrap();
        let vault_store = BincodeStore::new(client, TABLE_VAULT);

        let r = vault_store
            .read(|txn, table, _| {
                // let t = txn.open_table(table.table)?;

                // let r = t.get("my_key")?.unwrap();
                // let value = r.value();
                // let r = bincode::deserialize(&value)?;
                let r = table.read(&txn, "my_key")?;

                Ok(r)
            })
            .unwrap();

        println!("{:?}", r);

        // let read_txn = db.begin_read().unwrap();
        // let table = read_txn.open_table(TABLE_VAULT).unwrap();

        // assert_eq!(table.get("my_key").unwrap().unwrap().value(), 123);
    }

    const TABLE_VAULT_2: EncryptedBincodeTable<&str, MyStruct> =
        EncryptedBincodeTable::new("vault_2");
    const TEST_PASSWORD: &[u8] = b"test_password_123";
    const TEST_AAD: &[u8] = b"additional_authenticated_data";

    #[test]
    fn test_encrypted_write_read() {
        let client = ReDbClient::new("sapic.db").unwrap();
        let store = EncryptedBincodeStore::new(
            client,
            TABLE_VAULT_2,
            EncryptionConfig {
                memory_cost: 65536,
                time_cost: 10,
                parallelism: 4,
                salt_len: 32,
                nonce_len: 12,
            },
        );

        store
            .write(|mut txn, table, config| {
                table
                    .insert(
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
            .read(|txn, table, config| {
                let r = table.read(&txn, "my_key", TEST_PASSWORD, TEST_AAD, config)?;

                Ok(r)
            })
            .unwrap();

        println!("{:?}", r);
    }
}
