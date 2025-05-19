use arc_swap::ArcSwap;
use async_trait::async_trait;
use moss_db::{ClientState, DatabaseResult, Transaction};
use serde_json::Value as JsonValue;
use std::any::TypeId;
use std::collections::HashMap;
use std::path::Path;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::Notify;

use crate::new_storage::{
    Dump, NewReset, NewResettableStorage, NewStorage, new_storage::NewStorageHandle,
};
use crate::storage::{Transactional, table::Table};

pub struct NewResettableStorageHandle {
    state: ArcSwap<ClientState<NewStorageHandle>>,
    tables: Vec<Arc<dyn Table>>,
}

impl NewResettableStorageHandle {
    pub fn new(path: impl AsRef<Path>, tables: Vec<Arc<dyn Table>>) -> DatabaseResult<Self> {
        let storage = NewStorageHandle::new(path, tables.clone())?;

        Ok(Self {
            state: ArcSwap::new(Arc::new(ClientState::Loaded(storage))),
            tables,
        })
    }
}

#[async_trait]
impl Transactional for NewResettableStorageHandle {
    async fn begin_write(&self) -> DatabaseResult<Transaction> {
        loop {
            match self.state.load().as_ref() {
                ClientState::Loaded(storage) => return storage.begin_write().await,
                ClientState::Reloading { notify } => notify.notified().await,
            }
        }
    }

    async fn begin_read(&self) -> DatabaseResult<Transaction> {
        loop {
            match self.state.load().as_ref() {
                ClientState::Loaded(storage) => return storage.begin_read().await,
                ClientState::Reloading { notify } => notify.notified().await,
            }
        }
    }
}

#[async_trait]
impl Dump for NewResettableStorageHandle {
    async fn dump(&self) -> DatabaseResult<HashMap<String, JsonValue>> {
        loop {
            match self.state.load().as_ref() {
                ClientState::Loaded(storage) => return storage.dump().await,
                ClientState::Reloading { notify } => notify.notified().await,
            }
        }
    }
}

#[async_trait]
impl NewStorage for NewResettableStorageHandle {
    async fn table(&self, id: &TypeId) -> DatabaseResult<Arc<dyn Table>> {
        loop {
            match self.state.load().as_ref() {
                ClientState::Loaded(storage) => return storage.table(id).await,
                ClientState::Reloading { notify } => notify.notified().await,
            }
        }
    }
}

#[async_trait]
impl NewReset for NewResettableStorageHandle {
    async fn reset(
        &self,
        path: &Path,
        after_drop: Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>>,
    ) -> anyhow::Result<()> {
        // Wait for current operations to complete
        tokio::task::yield_now().await;
        dbg!(1);
        let local_notify = Arc::new(Notify::new());
        dbg!(2);
        let reloading_state = Arc::new(ClientState::Reloading {
            notify: local_notify.clone(),
        });
        dbg!(3);
        let old_state = self.state.swap(reloading_state);
        dbg!(4);
        drop(old_state);
        dbg!(5);
        after_drop.await?;
        dbg!(6);
        let new_storage = NewStorageHandle::new(path, self.tables.clone())?;
        let new_state = Arc::new(ClientState::Loaded(new_storage));
        self.state.store(new_state);
        dbg!(7);
        // Notify waiting operations
        local_notify.notify_waiters();
        Ok(())
    }
}

impl NewResettableStorage for NewResettableStorageHandle {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::segkey::{SegKey, SegKeyBuf};
    use moss_db::bincode_table::BincodeTable;
    use moss_db::primitives::AnyValue;
    use moss_testutils::random_name::random_string;
    use serde::{Deserialize, Serialize};
    use std::time::Duration;

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

    struct TestResettableStore {
        handle: Arc<dyn NewResettableStorage>,
    }

    impl TestResettableStore {
        pub fn new(path: &Path) -> Self {
            Self {
                handle: Arc::new(
                    NewResettableStorageHandle::new(path, vec![Arc::new(TABLE1)]).unwrap(),
                ),
            }
        }
        pub async fn table1(&self) -> DatabaseResult<Arc<dyn Table>> {
            self.handle.table(&TypeId::of::<Table1>()).await
        }
    }

    #[async_trait]
    impl Transactional for TestResettableStore {
        async fn begin_write(&self) -> DatabaseResult<Transaction> {
            self.handle.begin_write().await
        }

        async fn begin_read(&self) -> DatabaseResult<Transaction> {
            self.handle.begin_read().await
        }
    }

    #[async_trait]
    impl Dump for TestResettableStore {
        async fn dump(&self) -> DatabaseResult<HashMap<String, JsonValue>> {
            self.handle.dump().await
        }
    }

    #[async_trait]
    impl NewReset for TestResettableStore {
        async fn reset(
            &self,
            path: &Path,
            after_drop: Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>>,
        ) -> anyhow::Result<()> {
            self.handle.reset(path, after_drop).await
        }
    }

    #[derive(Serialize, Deserialize, PartialEq)]
    struct TestData {
        string: String,
        number: i32,
        boolean: bool,
    }

    #[tokio::test]
    async fn test_resettable_storage_basic() {
        let filename = format!("{}.db", random_string(10));
        let new_filename = format!("{}.renamed", filename);

        let path = Path::new("tests").join(&filename);
        let new_path = Path::new("tests").join(&new_filename);

        let store = Arc::new(TestResettableStore::new(&path));
        let table1 = store.table1().await.unwrap();
        let table2 = store.table1().await.unwrap();

        let mut write_txn = store.begin_write().await.unwrap();

        table1
            .definition()
            .insert(
                &mut write_txn,
                SegKey::new("table1").join("string"),
                &AnyValue::new(serde_json::to_vec("string").unwrap()),
            )
            .unwrap();

        table1
            .definition()
            .insert(
                &mut write_txn,
                SegKey::new("table1").join("number"),
                &AnyValue::new(serde_json::to_vec(&42).unwrap()),
            )
            .unwrap();

        let data = TestData {
            string: "String".to_string(),
            number: 42,
            boolean: true,
        };

        table2
            .definition()
            .insert(
                &mut write_txn,
                SegKey::new("table2").join("testdata"),
                &AnyValue::new(serde_json::to_vec(&data).unwrap()),
            )
            .unwrap();

        write_txn.commit().unwrap();

        let path_clone = path.clone();
        let new_path_clone = new_path.clone();
        let after_drop = Box::pin(async {
            tokio::fs::rename(path_clone, new_path_clone).await.unwrap();
            tokio::time::sleep(Duration::from_secs(2)).await;
            Ok(())
        });

        let new_path_clone = new_path.clone();
        let store_clone = store.clone();

        // Test reset in action will block other operations
        tokio::task::spawn(async move {
            store_clone
                .reset(&new_path_clone, after_drop)
                .await
                .unwrap();
        });

        tokio::time::sleep(Duration::from_secs(1)).await;

        let dumped = store.dump().await.unwrap();

        assert_eq!(dumped.len(), 3);
        assert_eq!(
            dumped["table1:string"],
            JsonValue::String("string".to_string())
        );
        assert_eq!(dumped["table1:number"], JsonValue::Number(42.into()));
        assert_eq!(
            dumped["table2:testdata"],
            serde_json::to_value(&data).unwrap()
        );

        tokio::fs::remove_file(&new_path).await.unwrap();
    }

    #[tokio::test]
    async fn test_resettable_storage_reset_during_operation() {
        let filename = format!("{}.db", random_string(10));
        let new_filename = format!("{}.renamed", filename);

        let path = Path::new("tests").join(&filename);
        let new_path = Path::new("tests").join(&new_filename);

        let store = Arc::new(TestResettableStore::new(&path));
        let table1 = store.table1().await.unwrap();

        let store_clone = store.clone();
        let table1_clone = table1.clone();
        tokio::task::spawn(async move {
            // Simulate a long-running operation
            dbg!("A");
            let mut write_txn = store_clone.begin_write().await.unwrap();

            dbg!("B");
            table1
                .definition()
                .insert(
                    &mut write_txn,
                    SegKey::new("table1").join("string"),
                    &AnyValue::new(serde_json::to_vec("string").unwrap()),
                )
                .unwrap();

            dbg!("C");
            tokio::time::sleep(Duration::from_secs(3)).await;

            dbg!("D");
            write_txn.commit().unwrap();
        });

        tokio::time::sleep(Duration::from_secs(1)).await;

        // Trying to reset while an operation is in action
        let path_clone = path.clone();
        let new_path_clone = new_path.clone();
        let after_drop = Box::pin(async {
            tokio::fs::rename(path_clone, new_path_clone).await.unwrap();
            Ok(())
        });

        store.reset(&new_path, after_drop).await.unwrap();

        let dumped = store.dump().await.unwrap();
        assert_eq!(dumped.len(), 1);
        assert_eq!(
            dumped["table1:string"],
            JsonValue::String("string".to_string())
        );

        tokio::fs::remove_file(&new_path).await.unwrap();
    }
}
