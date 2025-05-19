use async_trait::async_trait;
use moss_db::{DatabaseClient, DatabaseResult, ReDbClient, Transaction};
use serde_json::Value as JsonValue;
use std::any::TypeId;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use crate::new_storage::{Dump, NewStorage};
use crate::primitives::segkey::SegKeyBuf;
use crate::storage::{Storage, Transactional, table::Table};

pub struct NewStorageHandle {
    client: ReDbClient,
    pub(crate) tables: HashMap<TypeId, Arc<dyn Table>>,
}

impl NewStorageHandle {
    pub fn new(path: impl AsRef<Path>, tables: Vec<Arc<dyn Table>>) -> DatabaseResult<Self> {
        let mut client = ReDbClient::new(path.as_ref())?;
        let mut known_tables = HashMap::new();
        for table in tables {
            client = client.with_table(table.definition())?;
            known_tables.insert(table.type_id(), table);
        }

        Ok(Self {
            client,
            tables: known_tables,
        })
    }
}

#[async_trait]
impl Transactional for NewStorageHandle {
    async fn begin_write(&self) -> DatabaseResult<Transaction> {
        self.client.begin_write()
    }

    async fn begin_read(&self) -> DatabaseResult<Transaction> {
        self.client.begin_read()
    }
}

#[async_trait]
impl Dump for NewStorageHandle {
    async fn dump(&self) -> DatabaseResult<HashMap<String, JsonValue>> {
        // FIXME: Error propagation seems to make pipelining very tricky

        let read_txn = self.begin_read().await?;
        let mut result = HashMap::new();
        for table in self.tables.values() {
            for (k, v) in table.definition().scan(&read_txn)? {
                result.insert(k.to_string(), serde_json::from_slice(v.as_bytes())?);
            }
        }

        Ok(result)
    }
}

#[async_trait]
impl NewStorage for NewStorageHandle {
    async fn table(&self, id: &TypeId) -> DatabaseResult<Arc<dyn Table>> {
        Ok(self.tables.get(id).unwrap().clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::segkey::SegKey;
    use moss_db::bincode_table::BincodeTable;
    use moss_db::primitives::AnyValue;
    use moss_testutils::random_name::random_string;

    struct Table1 {
        table: BincodeTable<'static, SegKeyBuf, AnyValue>,
    }
    impl Table1 {
        pub const fn new() -> Self {
            Self {
                table: BincodeTable::new("table1"),
            }
        }
    }
    impl Table for Table1 {
        fn definition(&self) -> &BincodeTable<SegKeyBuf, AnyValue> {
            &self.table
        }
    }

    const TABLE1: Table1 = Table1::new();

    struct TestStore {
        handle: Arc<dyn NewStorage>,
    }

    impl TestStore {
        pub fn new(path: &Path) -> Self {
            Self {
                handle: Arc::new(NewStorageHandle::new(path, vec![Arc::new(TABLE1)]).unwrap()),
            }
        }
        pub async fn table1(&self) -> DatabaseResult<Arc<dyn Table>> {
            self.handle.table(&TypeId::of::<Table1>()).await
        }
    }
    #[async_trait]
    impl Transactional for TestStore {
        async fn begin_write(&self) -> DatabaseResult<Transaction> {
            self.handle.begin_write().await
        }

        async fn begin_read(&self) -> DatabaseResult<Transaction> {
            self.handle.begin_read().await
        }
    }
    #[async_trait]
    impl Dump for TestStore {
        async fn dump(&self) -> DatabaseResult<HashMap<String, JsonValue>> {
            self.handle.dump().await
        }
    }

    #[tokio::test]
    async fn test_storage() {
        let store = TestStore::new(&Path::new("tests").join(format!("{}.db", random_string(10))));
        let table1 = store.table1().await.unwrap();

        let mut write_txn = store.begin_write().await.unwrap();

        table1
            .definition()
            .insert(
                &mut write_txn,
                SegKey::new("table1").join("key1"),
                &AnyValue::new(serde_json::to_vec("value1").unwrap()),
            )
            .unwrap();

        write_txn.commit().unwrap();

        dbg!(store.dump().await.unwrap());
    }
}
